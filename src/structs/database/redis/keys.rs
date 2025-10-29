use std::fmt::{Display, Formatter, Result as FmtResult};

pub enum RedisKey {
    GuildChannelSnipes(String, String),
    GuildChannelEditSnipes(String, String),
    GuildChannelMessage(String, String, String),
    GuildChannelMessageCommandResponse(String, String, String),
    GuildChannelMessageEmbedFixResponse(String, String, String),
    GuildChannelMessageReactionSnipes(String, String, String),
    UserCooldown(String),
    UserLastPistonProgrammingLanguage(String),
}

impl Display for RedisKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::GuildChannelSnipes(guild_id, channel_id) => {
                write!(f, "guilds_{guild_id}_channels_{channel_id}_snipes")
            },
            Self::GuildChannelEditSnipes(guild_id, channel_id) => {
                write!(f, "guilds_{guild_id}_channels_{channel_id}_edit-snipes")
            },
            Self::GuildChannelMessage(guild_id, channel_id, message_id) => {
                write!(f, "guilds_{guild_id}_channels_{channel_id}_messages_{message_id}")
            },
            Self::GuildChannelMessageCommandResponse(guild_id, channel_id, message_id) => {
                write!(f, "guilds_{guild_id}_channels_{channel_id}_messages_{message_id}_command_response")
            },
            Self::GuildChannelMessageEmbedFixResponse(guild_id, channel_id, message_id) => {
                write!(f, "guilds_{guild_id}_channels_{channel_id}_messages_{message_id}_embed-fix-response")
            },
            Self::GuildChannelMessageReactionSnipes(guild_id, channel_id, message_id) => {
                write!(f, "guilds_{guild_id}_channels_{channel_id}_messages_{message_id}_reaction-snipes")
            },
            Self::UserCooldown(user_id) => {
                write!(f, "users_{user_id}_cooldown")
            },
            Self::UserLastPistonProgrammingLanguage(user_id) => {
                write!(f, "users_{user_id}_last-piston-programming-language")
            },
        }
    }
}
