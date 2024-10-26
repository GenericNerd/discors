//! The user module contains all the user-related structs and enums.

use bitflags::bitflags;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
/// The avatar decoration of a user
///
/// [Discord documentation](https://discord.com/developers/docs/resources/user#avatar-decoration-data-object)
pub struct AvatarDecoration {
    /// The [avatar decoration hash](https://discord.com/developers/docs/reference#image-formatting)
    // TODO: Investigate whether a ImageHash type should be used here
    pub asset: String,
    /// A snowflake for the ID of the decoration's SKU
    pub sku_id: u64,
}

bitflags! {
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    #[non_exhaustive]
    /// The public flags of a user
    ///
    /// [Discord documentation](https://discord.com/developers/docs/resources/user#user-object-user-flags)
    pub struct UserFlags: u64 {
        /// Discord Employee
        const STAFF = 1 << 0;
        /// Partnered Server Owner
        const PARTNER = 1 << 1;
        /// HypeSquad Events Member
        const HYPESQUAD = 1 << 2;
        /// Bug Hunter Level 1
        const BUG_HUNTER_LEVEL_1 = 1 << 3;
        /// HypeSquad Bravery Member
        const HYPESQUAD_BRAVERY = 1 << 6;
        /// HypeSquad Brilliance Member
        const HYPESQUAD_BRILLIANCE = 1 << 7;
        /// HypeSquad Balance Member
        const HYPESQUAD_BALANCE = 1 << 8;
        /// Early Supporter
        const EARLY_SUPPORTER = 1 << 9;
        /// Team User
        const TEAM_PSEUDO_USER = 1 << 10;
        /// Bug Hunter Level 2
        const BUG_HUNTER_LEVEL_2 = 1 << 14;
        /// Verified Bot
        const VERIFIED_BOT = 1 << 16;
        /// Early Verified Bot Developer
        const VERIFIED_DEVELOPER = 1 << 17;
        /// Discord Certified Moderator
        const CERTIFICATED_MODERATOR = 1 << 18;
        /// Bot uses only HTTP interactions and is shown in the online member list
        const BOT_HTTP_INTERACTIONS = 1 << 19;
        /// Active developer
        const ACTIVE_DEVELOPER = 1 << 22;
    }
}

impl<'de> Deserialize<'de> for UserFlags {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let bits = u64::deserialize(deserializer)?;
        Ok(Self::from_bits_truncate(bits))
    }
}

impl Serialize for UserFlags {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.bits().serialize(serializer)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u8)]
#[serde(into = "u8", from = "u8")]
#[non_exhaustive]
/// The subscription type of a user
///
/// [Discord documentation](https://discord.com/developers/docs/resources/user#user-object-premium-types)
pub enum PremiumType {
    /// User is not subscribed to Nitro
    None = 0,
    /// User is a Nitro Classic subscriber
    NitroClassic = 1,
    /// User is a Nitro subscriber
    Nitro = 2,
    /// User is a Nitro Basic subscriber
    NitroBasic = 3,
}

impl From<PremiumType> for u8 {
    fn from(value: PremiumType) -> Self {
        value as u8
    }
}

impl From<u8> for PremiumType {
    fn from(value: u8) -> Self {
        match value {
            0 => PremiumType::None,
            1 => PremiumType::NitroClassic,
            2 => PremiumType::Nitro,
            3 => PremiumType::NitroBasic,
            _ => panic!("Invalid PremiumType value: {value}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
/// The user object
///
/// [Discord documentation](https://discord.com/developers/docs/resources/user#user-object)
pub struct User {
    /// The user's ID
    // TODO: User IDs come in as Strings but should be u64s
    pub id: String,
    /// The user's username, not unique across the platform (although this is becoming
    /// increasingly uncommon due to the discontinuation of the discriminator field)
    pub username: String,
    #[deprecated(
        note = "Although some users may still have a discriminator, it is becoming increasingly uncommon. You should use the username field instead."
    )]
    /// The user's 4-digit discord-tag
    ///
    /// *Note: This field is deprecated and may be removed in the future.*
    pub discriminator: String,
    /// The user's display name
    pub global_name: Option<String>,
    /// The user's [avatar hash](https://discord.com/developers/docs/reference#image-formatting)
    // TODO: Investigate whether a ImageHash type should be used here
    pub avatar: Option<String>,
    /// Whether the user belongs to an `OAuth2` application
    pub bot: Option<bool>,
    /// Whether the user is an Official Discord System user (part of the urgent message system)
    pub system: Option<bool>,
    /// Whether the user has two factor enabled on their account
    pub mfa_enabled: Option<bool>,
    /// The user's [banner hash](https://discord.com/developers/docs/reference#image-formatting)
    // TODO: Investigate whether a ImageHash type should be used here
    pub banner: Option<String>,
    /// The user's banner color encoded as an integer representation of hexadecimal color code
    pub accent_color: Option<u32>,
    /// The user's two letter language code, as defined [here](https://discord.com/developers/docs/reference#locales)
    pub locale: Option<String>,
    /// Whether the email on this account has been verified
    ///
    /// *Note: This requires an extra scope of `email` when authenticating*
    pub verified: Option<bool>,
    /// The user's email
    ///
    /// *Note: This requires an extra scope of `email` when authenticating*
    pub email: Option<String>,
    /// The flags on this account
    pub flags: Option<UserFlags>,
    /// The type of Nitro subscription on this account
    pub premium_type: Option<PremiumType>,
    /// The public flags on this account
    pub public_flags: Option<UserFlags>,
    /// The user's avatar decoration data
    pub avatar_decoration_data: Option<AvatarDecoration>,
}
