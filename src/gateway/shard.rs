use std::time::Duration;

use serde::{ser::SerializeSeq, Deserialize, Serialize};
use tokio::time::Instant;

use crate::{
    error::{Error, Result},
    model::gateway::{
        dispatch::DispatchEvent,
        event::{Event, ReceiveEventData},
        intents::GatewayIntents,
    },
};

use super::{error::Error as GatewayError, websocket::WebsocketClient};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ShardInformation {
    pub id: u64,
    pub total: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ReconnectionKind {
    Identify,
    Resume,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ShardAction {
    Heartbeat,
    Identify,
    Reconnect(ReconnectionKind),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ConnectionStage {
    Connecting,
    Connected,
    Disconnected,
    Handshake,
    Identifying,
    Resuming,
}

impl Serialize for ShardInformation {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(2))?;
        seq.serialize_element(&self.id)?;
        seq.serialize_element(&self.total)?;
        seq.end()
    }
}

impl<'de> Deserialize<'de> for ShardInformation {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        <(u64, u64)>::deserialize(deserializer).map(|(id, total)| Self { id, total })
    }
}

#[derive(Debug)]
pub struct Shard {
    websocket_url: String,
    pub websocket: WebsocketClient,
    connection_stage: ConnectionStage,
    heartbeat_interval: Option<Duration>,
    last_heartbeat_sent: Option<Instant>,
    last_heartbeat_received: bool,
    sequence: u64,
    session_id: Option<String>,
    resume_url: Option<String>,
    pub shard_information: Option<ShardInformation>,
    token: String,
    pub intents: GatewayIntents,
}

impl Shard {
    pub async fn new(
        websocket_url: &str,
        token: &str,
        shard_information: ShardInformation,
        intents: GatewayIntents,
    ) -> Result<Self> {
        let websocket = WebsocketClient::connect(websocket_url).await?;
        Ok(Self {
            websocket_url: websocket_url.to_string(),
            websocket,
            connection_stage: ConnectionStage::Handshake,
            heartbeat_interval: None,
            last_heartbeat_sent: None,
            last_heartbeat_received: false,
            sequence: 0,
            session_id: None,
            resume_url: None,
            shard_information: Some(shard_information),
            token: token.to_string(),
            intents,
        })
    }

    pub async fn init(&mut self) -> Result<()> {
        self.connection_stage = ConnectionStage::Connecting;
        let url = self.resume_url.as_ref().unwrap_or(&self.websocket_url);
        let client = WebsocketClient::connect(url.as_str()).await?;
        self.websocket = client;
        Ok(())
    }

    pub fn reset(&mut self, resuming: bool) {
        self.last_heartbeat_sent = Some(Instant::now());
        self.last_heartbeat_received = true;
        self.heartbeat_interval = None;
        self.connection_stage = ConnectionStage::Disconnected;
        self.sequence = 0;
        if !resuming {
            self.session_id = None;
            self.resume_url = None;
        }
    }

    pub fn handle_event(&mut self, event: Result<&Event>) -> Result<Option<ShardAction>> {
        match event {
            Ok(event) => {
                let Some(ref data) = event.receive_data else {
                    return Ok(None);
                };

                match data {
                    ReceiveEventData::Dispatch(data) => {
                        match data {
                            DispatchEvent::Ready(ready) => {
                                self.resume_url = Some(ready.resume_gateway_url.clone());
                                self.session_id = Some(ready.session_id.clone());
                                self.connection_stage = ConnectionStage::Connected;
                                self.last_heartbeat_received = true;
                            }
                            DispatchEvent::Resumed => {
                                self.connection_stage = ConnectionStage::Connected;
                                self.last_heartbeat_received = true;
                                self.last_heartbeat_sent = Some(Instant::now());
                            }
                            _ => {}
                        }
                        self.sequence = event.sequence.unwrap_or(self.sequence);
                        Ok(None)
                    }
                    ReceiveEventData::Heartbeat => {
                        if self.connection_stage == ConnectionStage::Handshake {
                            self.connection_stage = ConnectionStage::Identifying;
                            Ok(Some(ShardAction::Identify))
                        } else {
                            Ok(Some(ShardAction::Heartbeat))
                        }
                    }
                    ReceiveEventData::Reconnect => {
                        Ok(Some(ShardAction::Reconnect(ReconnectionKind::Resume)))
                    }
                    ReceiveEventData::InvalidSession(resumable) => Ok(Some(if *resumable {
                        ShardAction::Reconnect(ReconnectionKind::Resume)
                    } else {
                        ShardAction::Reconnect(ReconnectionKind::Identify)
                    })),
                    ReceiveEventData::Hello { heartbeat_interval } => {
                        self.heartbeat_interval = Some(Duration::from_millis(*heartbeat_interval));

                        Ok(Some(
                            if self.connection_stage == ConnectionStage::Handshake {
                                ShardAction::Identify
                            } else {
                                ShardAction::Reconnect(if self.resume_url.is_some() {
                                    ReconnectionKind::Resume
                                } else {
                                    ReconnectionKind::Identify
                                })
                            },
                        ))
                    }
                    ReceiveEventData::HeartbeatAck => {
                        self.last_heartbeat_received = true;
                        Ok(None)
                    }
                }
            }
            // TODO: handle gateway being closed
            Err(err) => Err(err),
        }
    }

    pub async fn heartbeat(&mut self) -> Result<()> {
        self.websocket.send_heartbeat(Some(self.sequence)).await?;
        self.last_heartbeat_sent = Some(Instant::now());
        self.last_heartbeat_received = false;
        Ok(())
    }

    pub async fn do_heartbeat_interval(&mut self) -> bool {
        let Some(heartbeat_interval) = self.heartbeat_interval else {
            return true;
        };

        if let Some(last_sent) = self.last_heartbeat_sent {
            if last_sent.elapsed() <= heartbeat_interval {
                return true;
            }
        }

        if !self.last_heartbeat_received {
            return false;
        }

        self.heartbeat().await.is_ok()
    }

    pub async fn identify(&mut self) -> Result<()> {
        self.websocket
            .send_identify(&self.token, &self.shard_information, &self.intents)
            .await?;

        self.last_heartbeat_sent = Some(Instant::now());
        self.connection_stage = ConnectionStage::Identifying;

        Ok(())
    }

    pub async fn resume(&mut self) -> Result<()> {
        self.init().await?;
        self.connection_stage = ConnectionStage::Resuming;

        let Some(ref session_id) = self.session_id else {
            return Err(Error::Gateway(GatewayError::NoSessionToResume));
        };
        self.websocket
            .send_resume(&self.token, session_id.as_str(), self.sequence)
            .await
    }
}
