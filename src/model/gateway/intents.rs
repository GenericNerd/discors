use bitflags::bitflags;
use serde::Serialize;

bitflags! {
    /// [Discord documentation](https://discord.com/developers/docs/topics/gateway#gateway-intents)
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct GatewayIntents: u32 {
        /// The guilds intent allows the gateway to send the following events:
        /// - GUILD_CREATE
        /// - GUILD_UPDATE
        /// - GUILD_DELETE
        /// - GUILD_ROLE_CREATE
        /// - GUILD_ROLE_UPDATE
        /// - GUILD_ROLE_DELETE
        /// - CHANNEL_CREATE
        /// - CHANNEL_UPDATE
        /// - CHANNEL_DELETE
        /// - CHANNEL_PINS_UPDATE
        /// - THREAD_CREATE
        /// - THREAD_UPDATE
        /// - THREAD_DELETE
        /// - THREAD_LIST_SYNC
        /// - THREAD_MEMBER_UPDATE
        /// - THREAD_MEMBERS_UPDATE
        /// - STAGE_INSTANCE_CREATE
        /// - STAGE_INSTANCE_UPDATE
        /// - STAGE_INSTANCE_DELETE
        const GUILDS = 1 << 0;
        /// The guild members intent allows the gateway to send the following events:
        /// - GUILD_MEMBER_ADD
        /// - GUILD_MEMBER_UPDATE
        /// - GUILD_MEMBER_REMOVE
        /// - THREAD_MEMBERS_UPDATE
        ///
        /// **This intent is privileged.** - This means that your app requires
        /// approval from Discord to use this intent.
        const GUILD_MEMBERS = 1 << 1;
        /// The guild bans intent allows the gateway to send the following events:
        /// - GUILD_AUDIT_LOG_ENTRY_CREATE
        /// - GUILD_BAN_ADD
        /// - GUILD_BAN_REMOVE
        const GUILD_MODERATION = 1 << 2;
        /// The guild expressions intent allows the gateway to send the following events:
        /// - GUILD_EMOJIS_UPDATE
        /// - GUILD_STICKERS_UPDATE
        /// - GUILD_SOUNDBOARD_SOUND_CREATE
        /// - GUILD_SOUNDBOARD_SOUND_UPDATE
        /// - GUILD_SOUNDBOARD_SOUND_DELETE
        /// - GUILD_SOUNDBOARD_SOUNDS_UPDATE
        const GUILD_EXPRESSIONS = 1 << 3;
        /// The guild integrations intent allows the gateway to send the following events:
        /// - GUILD_INTEGRATIONS_UPDATE
        /// - INTEGRATION_CREATE
        /// - INTEGRATION_UPDATE
        /// - INTEGRATION_DELETE
        const GUILD_INTEGRATIONS = 1 << 4;
        /// The guild webhooks intent allows the gateway to send the following events:
        /// - WEBHOOKS_UPDATE
        const GUILD_WEBHOOKS = 1 << 5;
        /// The guild invites intent allows the gateway to send the following events:
        /// - INVITE_CREATE
        /// - INVITE_DELETE
        const GUILD_INVITES = 1 << 6;
        /// The guild voice states intent allows the gateway to send the following events:
        /// - VOICE_CHANNEL_EFFECT_SEND
        /// - VOICE_STATE_UPDATE
        const GUILD_VOICE_STATES = 1 << 7;
        /// The guild presences intent allows the gateway to send the following events:
        /// - PRESENCE_UPDATE
        ///
        /// **This intent is privileged.** - This means that your app requires
        /// approval from Discord to use this intent.
        const GUILD_PRESENCES = 1 << 8;
        /// The guild messages intent allows the gateway to send the following events:
        /// - MESSAGE_CREATE
        /// - MESSAGE_UPDATE
        /// - MESSAGE_DELETE
        /// - MESSAGE_DELETE_BULK
        const GUILD_MESSAGES = 1 << 9;
        /// The guild message reactions intent allows the gateway to send the following events:
        /// - MESSAGE_REACTION_ADD
        /// - MESSAGE_REACTION_REMOVE
        /// - MESSAGE_REACTION_REMOVE_ALL
        /// - MESSAGE_REACTION_REMOVE_EMOJI
        const GUILD_MESSAGE_REACTIONS = 1 << 10;
        /// The guild message typing intent allows the gateway to send the following events:
        /// - TYPING_START
        const GUILD_MESSAGE_TYPING = 1 << 11;
        /// The direct messages intent allows the gateway to send the following events:
        /// - MESSAGE_CREATE
        /// - MESSAGE_UPDATE
        /// - MESSAGE_DELETE
        /// - CHANNEL_PINS_UPDATE
        const DIRECT_MESSAGES = 1 << 12;
        /// The direct message reactions intent allows the gateway to send the following events:
        /// - MESSAGE_REACTION_ADD
        /// - MESSAGE_REACTION_REMOVE
        /// - MESSAGE_REACTION_REMOVE_ALL
        /// - MESSAGE_REACTION_REMOVE_EMOJI
        const DIRECT_MESSAGE_REACTIONS = 1 << 13;
        /// The direct message typing intent allows the gateway to send the following events:
        /// - TYPING_START
        const DIRECT_MESSAGE_TYPING = 1 << 14;
        /// The message content intent allows the gateway to send the contents of a message.
        ///
        /// **This intent is privileged.** - This means that your app requires
        /// approval from Discord to use this intent.
        const MESSAGE_CONTENT = 1 << 15;
        /// The guild scheduled events intent allows the gateway to send the following events:
        /// - GUILD_SCHEDULED_EVENT_CREATE
        /// - GUILD_SCHEDULED_EVENT_UPDATE
        /// - GUILD_SCHEDULED_EVENT_DELETE
        /// - GUILD_SCHEDULED_EVENT_USER_ADD
        /// - GUILD_SCHEDULED_EVENT_USER_REMOVE
        const GUILD_SCHEDULED_EVENTS = 1 << 16;
        /// The auto-moderation configuration intent allows the gateway to send the following events:
        /// - AUTO_MODERATION_RULE_CREATE
        /// - AUTO_MODERATION_RULE_UPDATE
        /// - AUTO_MODERATION_RULE_DELETE
        const AUTO_MODERATION_CONFIGURATION = 1 << 20;
        /// The auto-moderation execution intent allows the gateway to send the following events:
        /// - AUTO_MODERATION_ACTION_EXECUTION
        const AUTO_MODERATION_EXECUTION = 1 << 21;
        /// The guild message polls intent allows the gateway to send the following events:
        /// - MESSAGE_POLL_VOTE_ADD
        /// - MESSAGE_POLL_VOTE_REMOVE
        const GUILD_MESSAGE_POLLS = 1 << 24;
        /// The direct message polls intent allows the gateway to send the following events:
        /// - MESSAGE_POLL_VOTE_ADD
        /// - MESSAGE_POLL_VOTE_REMOVE
        const DIRECT_MESSAGE_POLLS = 1 << 25;
    }
}

impl Serialize for GatewayIntents {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.bits().serialize(serializer)
    }
}

impl GatewayIntents {
    #[must_use]
    pub fn privileged() -> GatewayIntents {
        Self::GUILD_MEMBERS | Self::GUILD_PRESENCES | Self::MESSAGE_CONTENT
    }

    #[must_use]
    pub fn non_privileged() -> GatewayIntents {
        Self::privileged().complement()
    }
}

impl Default for GatewayIntents {
    fn default() -> Self {
        Self::non_privileged()
    }
}
