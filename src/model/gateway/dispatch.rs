//! Dispatch events are sent from the gateway to the client and contain
//! updates and other information, paramount to the functionality of the
//! client.

use serde::Deserialize;

use crate::model::{guild::UnavailableGuild, user::User};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
/// `READY` is sent from the gateway
///
/// [Discord documentation](https://discord.com/developers/docs/events/gateway-events#ready)
pub struct ReadyEvent {
    /// The gateway API version. By default, this value is 6, but we should
    /// attempt to connect to the latest version, which at the time of writing
    /// is 10.
    pub v: u16,
    /// Our current user, this includes the email address.
    pub user: User,
    /// These are the guilds the user is currently in. More information about the guilds
    /// are often obtained through the `GuildCreate` and `GuildUpdate` events.
    pub guilds: Vec<UnavailableGuild>,
    /// This is given to the gateway when resuming a session and for obtaining missed events.
    pub session_id: String,
    /// The URL used to reconnect and resume the session with the gateway.
    pub resume_gateway_url: String,
    /// Shard information that is associated with this session, if sent when identifying.
    /// The first value is the current shard (based on a zero-based index), and the second
    /// value is the total number of shards.
    pub shard: Option<(u64, u64)>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
pub struct GuildCreateEvent {}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
pub struct GuildUpdateEvent {}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
pub struct GuildDeleteEvent {}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(tag = "t", content = "d")]
/// The type of event that is dispatched by the gateway.
///
/// A list of all the events can be found [here](https://discord.com/developers/docs/events/gateway-events#receive-events).
///
/// *Note: Some items in this list are not provided via a Dispatch event, but through other events.*
pub enum DispatchEvent {
    /// Contains the initial state information
    Ready(ReadyEvent),
    /// Sent when we resume a session
    Resumed,
    GuildCreate(GuildCreateEvent),
    GuildUpdate(GuildUpdateEvent),
    GuildDelete(GuildDeleteEvent),
}
