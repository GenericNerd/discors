//! The guild module contains all the guild-related structs and enums.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
/// An unavailable guild is a partial guild object that is considered either:
/// 1. Offline (due to an outage or other temporary issue); or
/// 2. Further information will be provided in the future (such as through [`GuildCreate`] events)
pub struct UnavailableGuild {
    /// The guild ID
    // TODO: Guild IDs come in as Strings but should be u64s
    pub id: String,
    /// Whether the guild is unavailable, this should always be true
    pub unavailable: bool,
}
