//! Events are sent from the gateway to the client and contain
//! connection information, [`DispatchEvent`]s and other information
//! important for the functionality of the client.

use serde::{Deserialize, Serialize};

use crate::gateway::shard::ShardInformation;

use super::{dispatch::DispatchEvent, intents::GatewayIntents};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[repr(u8)]
#[serde(into = "u8", from = "u8")]
#[non_exhaustive]
/// Used to identify the type of event sent and received by the gateway.
///
/// [Discord documentation](https://discord.com/developers/docs/topics/opcodes-and-status-codes#gateway-gateway-opcodes)
pub enum OpCode {
    /// **Receive** only
    ///
    /// An event was dispatched, the inner payload is provided by [`ReceiveEventData`]
    Dispatch = 0,
    /// **Send/Receive**
    ///
    /// Fired periodically by the client to keep the connection alive.
    /// Can be received by the gateway to send a heartbeat immediately.
    Heartbeat = 1,
    /// **Send**
    ///
    /// Starts a new session during the initial handshake.
    Identify = 2,
    /// **Send**
    ///
    /// Update the client's presence
    PresenceUpdate = 3,
    /// **Send**
    ///
    /// Used to join/leave or move between voice channels.
    VoiceStateUpdate = 4,
    /// **Send**
    ///
    /// Resume a previous session that was disconnected.
    Resume = 6,
    /// **Receive**
    ///
    /// The client should attempt to reconnect and resume immediately.
    Reconnect = 7,
    /// **Send**
    ///
    /// Request information about offline guild members in a large guild.
    RequestGuildMembers = 8,
    /// **Receive**
    ///
    /// The session has been invalidated. The client should reconnect and identify/resume accordingly.
    InvalidSession = 9,
    /// **Receive**
    ///
    /// Received immediately after connecting, contains the `heartbeat_interval` to use.
    /// See [`ReceiveEventData::Hello`]
    Hello = 10,
    /// **Receive**
    ///
    /// Received in response to sending a heartbeat, the gateway acknowledges the heartbeat.
    HeartbeatACK = 11,
    /// **Send**
    ///
    /// Request information about soundboard sounds in a set of guilds.
    RequestSoundboardSounds = 31,
}

impl From<OpCode> for u8 {
    fn from(value: OpCode) -> Self {
        value as u8
    }
}

impl From<u8> for OpCode {
    fn from(value: u8) -> Self {
        match value {
            0 => OpCode::Dispatch,
            1 => OpCode::Heartbeat,
            2 => OpCode::Identify,
            3 => OpCode::PresenceUpdate,
            4 => OpCode::VoiceStateUpdate,
            6 => OpCode::Resume,
            7 => OpCode::Reconnect,
            8 => OpCode::RequestGuildMembers,
            9 => OpCode::InvalidSession,
            10 => OpCode::Hello,
            11 => OpCode::HeartbeatACK,
            31 => OpCode::RequestSoundboardSounds,
            _ => panic!("Invalid OpCode value: {value}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
/// Required properties for the [`OpCode::Identify`] opcode
///
/// [Discord documentation](https://discord.com/developers/docs/events/gateway#identifying)
pub struct IdentifyProperties {
    os: String,
    browser: String,
    device: String,
}

impl Default for IdentifyProperties {
    fn default() -> Self {
        Self {
            os: std::env::consts::OS.to_string(),
            browser: "discors".to_string(),
            device: "discors".to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
#[serde(untagged)]
/// The event data when receiving a [`ReceiveEvent`] via the gateway
pub enum ReceiveEventData {
    /// Most events received are dispatched through this variant. As such, the data is
    /// contained within the inner [`DispatchEvent`] variant.
    Dispatch(DispatchEvent),
    /// Received when the client should immediately send a heartbeat.
    Heartbeat,
    /// Received when the client should reconnect and resume immediately.
    Reconnect,
    /// Received when this connection is no longer valid. The inner [bool] indicates whether
    /// the session can be resumed.
    InvalidSession(bool),
    /// Received immediately after connecting.
    Hello {
        /// The interval in milliseconds at which the client should send heartbeats.
        heartbeat_interval: u64,
    },
    /// Received to acknowledge a heartbeat.
    HeartbeatAck,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
#[serde(untagged)]
/// The event data when sending a [`SendEvent`] via the gateway
pub enum SendEventData {
    /// Sent in response to a [`OpCode::Heartbeat`] event or to keep the connection alive.
    /// Optionally contains the sequence number of the last event received.
    ///
    /// [Discord documentation](https://discord.com/developers/docs/topics/gateway-events#heartbeat)
    Heartbeat(Option<u64>),
    /// Used to trigger the initial handshake with the gateway. Provides the token and other
    /// information required to identify the client.
    ///
    /// [Discord documentation](https://discord.com/developers/docs/topics/gateway-events#identify)
    Identify {
        /// The token of the bot that the client is connecting with
        token: String,
        /// The properties of the client
        properties: IdentifyProperties,
        #[serde(skip_serializing_if = "Option::is_none")]
        /// Whether this connection supports the compression of packets
        compress: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        /// Value between 50 and 250, total number of members where the gateway will stop
        /// sending offline members in the guild member list
        large_threshold: Option<u8>,
        #[serde(skip_serializing_if = "Option::is_none")]
        /// The shard information for this connection, the first value is the current shard (based on a zero-based index),
        /// and the second value is the total number of shards.
        shard: Option<ShardInformation>,
        // presence: Option<PresenceUpdate>,
        /// The intents of the client
        intents: GatewayIntents,
    },
    /// Resume a previous session that was terminated.
    ///
    /// [Discord documentation](https://discord.com/developers/docs/topics/gateway-events#resume)
    Resume {
        /// The token that the client used when connecting to the gateway
        token: String,
        /// The session ID that the client used when connecting to the gateway
        session_id: String,
        #[serde(rename = "seq")]
        /// The last sequence received by the client.
        sequence: u64,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
/// An event received from the gateway
pub struct Event {
    /// The opcode of the event
    pub op: OpCode,
    #[serde(rename = "d")]
    #[serde(skip_serializing)]
    /// The data of the event
    pub receive_data: Option<ReceiveEventData>,
    #[serde(rename = "d")]
    #[serde(skip_deserializing)]
    /// The data of the event
    pub send_data: Option<SendEventData>,
    #[serde(rename = "s")]
    /// The sequence number of the event, which should increment by one for each event
    pub sequence: Option<u64>,
    #[serde(rename = "t")]
    /// The event name, if applicable
    pub event: Option<String>,
}

impl Default for Event {
    fn default() -> Self {
        Self {
            op: OpCode::Heartbeat,
            receive_data: None,
            send_data: None,
            sequence: None,
            event: None,
        }
    }
}

impl<'de> Deserialize<'de> for Event {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let mut event_map = serde_json::Map::deserialize(deserializer)?;
        let sequence = event_map.remove("s").and_then(|s| s.as_u64());
        let op = OpCode::from(
            event_map
                .remove("op")
                .ok_or_else(|| serde::de::Error::missing_field("op"))?
                .as_u64()
                .ok_or_else(|| serde::de::Error::custom("op is not a u64"))? as u8,
        );
        let data = match op {
            OpCode::Dispatch => Some(ReceiveEventData::Dispatch(
                DispatchEvent::deserialize(&event_map)
                    .map_err(|err| serde::de::Error::custom(err.to_string()))?,
            )),
            OpCode::Heartbeat => Some(ReceiveEventData::Heartbeat),
            OpCode::Reconnect => Some(ReceiveEventData::Reconnect),
            OpCode::InvalidSession => Some(ReceiveEventData::InvalidSession(
                event_map
                    .remove("d")
                    .ok_or_else(|| serde::de::Error::missing_field("d"))?
                    .as_bool()
                    .ok_or_else(|| serde::de::Error::custom("d is not a bool"))?,
            )),
            OpCode::Hello => {
                let inner = event_map
                    .remove("d")
                    .ok_or_else(|| serde::de::Error::missing_field("d"))?;
                Some(
                    ReceiveEventData::deserialize(inner)
                        .map_err(|err| serde::de::Error::custom(err.to_string()))?,
                )
            }
            OpCode::HeartbeatACK => Some(ReceiveEventData::HeartbeatAck),
            _ => None,
        };
        let event = event_map
            .remove("t")
            .and_then(|s| s.as_str().map(std::string::ToString::to_string));

        Ok(Self {
            op,
            receive_data: data,
            send_data: None,
            sequence,
            event,
        })
    }
}
