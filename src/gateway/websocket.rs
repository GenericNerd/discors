use std::{io::Read, time::Duration};

use flate2::read::ZlibDecoder;
use futures::{stream::FusedStream, SinkExt, StreamExt};
use serde_json::{from_str, to_string};
use tokio::{net::TcpStream, time::timeout};
use tokio_tungstenite::{
    connect_async_with_config,
    tungstenite::{protocol::WebSocketConfig, Message},
    MaybeTlsStream, WebSocketStream,
};

use crate::{
    error::Result,
    model::gateway::{
        event::{Event, IdentifyProperties, OpCode, SendEventData},
        intents::GatewayIntents,
    },
};

use super::{error::Error as GatewayError, shard::ShardInformation};

#[derive(Debug)]
pub struct WebsocketClient(WebSocketStream<MaybeTlsStream<TcpStream>>);

impl WebsocketClient {
    pub async fn connect(url: &str) -> Result<Self> {
        let config = WebSocketConfig::default();

        let (stream, _) = connect_async_with_config(url, Some(config), false).await?;

        Ok(Self(stream))
    }

    pub async fn receive(&mut self) -> Result<Option<Event>> {
        if self.0.is_terminated() {
            return Err(GatewayError::Closed(None))?;
        }

        let message = match timeout(Duration::from_millis(500), self.0.next()).await {
            Ok(Some(Ok(message))) => message,
            Ok(Some(Err(err))) => return Err(err)?,
            Ok(None) | Err(_) => return Ok(None),
        };

        let value = match message {
            Message::Binary(bytes) => {
                let mut decompressed = String::with_capacity(bytes.len() * 3);
                ZlibDecoder::new(&bytes[..]).read_to_string(&mut decompressed)?;
                from_str(decompressed.as_str())?
            }
            Message::Text(text) => from_str(text.as_str())?,
            Message::Close(frame) => return Err(GatewayError::Closed(frame))?,
            _ => return Ok(None),
        };

        Ok(Some(value))
    }

    pub async fn send(&mut self, message: &impl serde::Serialize) -> Result<()> {
        let message = to_string(message).map(Message::Text)?;
        self.0.send(message).await?;
        Ok(())
    }

    pub async fn send_heartbeat(&mut self, sequence: Option<u64>) -> Result<()> {
        self.send(&Event {
            op: OpCode::Heartbeat,
            send_data: Some(SendEventData::Heartbeat(sequence)),
            ..Default::default()
        })
        .await
    }

    pub async fn send_identify(
        &mut self,
        token: &str,
        shard_information: &Option<ShardInformation>,
        intents: &GatewayIntents,
    ) -> Result<()> {
        self.send(&Event {
            op: OpCode::Identify,
            send_data: Some(SendEventData::Identify {
                token: token.to_string(),
                properties: IdentifyProperties::default(),
                compress: None,
                large_threshold: None,
                shard: *shard_information,
                intents: *intents,
            }),
            ..Default::default()
        })
        .await
    }

    pub async fn send_resume(
        &mut self,
        token: &str,
        session_id: &str,
        sequence: u64,
    ) -> Result<()> {
        self.send(&Event {
            op: OpCode::Resume,
            send_data: Some(SendEventData::Resume {
                token: token.to_string(),
                session_id: session_id.to_string(),
                sequence,
            }),
            ..Default::default()
        })
        .await
    }
}
