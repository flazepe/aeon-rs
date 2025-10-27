use slashook::structs::{messages::Message as SlashookMessage, users::User as SlashookUser};
use std::fmt::Display;
use twilight_model::{
    channel::{Message as TwilightMessage, message::EmojiReactionType},
    user::User as TwilightUser,
};

pub trait UserExt {
    fn label(&self) -> String;
}

impl UserExt for SlashookUser {
    fn label(&self) -> String {
        format!("@{} ({})", self.username, self.id)
    }
}

impl UserExt for TwilightUser {
    fn label(&self) -> String {
        format!("@{} ({})", self.name, self.id)
    }
}

pub trait UserAvatarExt {
    fn avatar_url<T: Display, U: Display>(&self, format: T, size: U) -> Option<String>;
    fn display_avatar_url<T: Display, U: Display>(&self, format: T, size: U) -> String;
}

impl UserAvatarExt for TwilightUser {
    fn avatar_url<T: Display, U: Display>(&self, format: T, size: U) -> Option<String> {
        let format = format.to_string();

        self.avatar.as_ref().map(|avatar| {
            format!(
                "https://cdn.discordapp.com/avatars/{user_id}/{avatar}.{format}?size={size}",
                user_id = self.id,
                format = if !avatar.is_animated() && format == "gif" { "png" } else { &format },
            )
        })
    }

    fn display_avatar_url<T: Display, U: Display>(&self, format: T, size: U) -> String {
        self.avatar_url(format, size).unwrap_or_else(|| {
            format!(
                "https://cdn.discordapp.com/embed/avatars/{}.png",
                if self.discriminator == 0 { ((self.id.get() >> 22) % 6) as u8 } else { (self.discriminator % 5) as u8 },
            )
        })
    }
}

pub trait MessageExt {
    fn reply_text(&self) -> Option<String>;
}

macro_rules! format_reply_text {
    ($user_label:expr, $guild_id:expr, $channel_id:expr, $message_id:expr $(,)?) => {
        format!(
            "[Replying to {}](https://discord.com/channels/{}/{}/{})",
            crate::functions::escape_markdown($user_label),
            $guild_id,
            $channel_id,
            $message_id,
        )
    };
    () => {
        "Replying to a message".into()
    };
}

impl MessageExt for SlashookMessage {
    fn reply_text(&self) -> Option<String> {
        self.message_reference.as_ref().map(|_| match &self.referenced_message {
            Some(referenced_message) => format_reply_text!(
                referenced_message.author.as_ref().map(|author| author.label()).as_deref().unwrap_or_default(),
                referenced_message.guild_id.as_ref().map(|guild_id| guild_id.to_string()).as_deref().unwrap_or("@me"),
                referenced_message.channel_id.as_deref().unwrap_or_default(),
                referenced_message.id.as_deref().unwrap_or_default(),
            ),
            None => format_reply_text!(),
        })
    }
}

impl MessageExt for TwilightMessage {
    fn reply_text(&self) -> Option<String> {
        self.reference.as_ref().map(|_| match &self.referenced_message {
            Some(referenced_message) => format_reply_text!(
                referenced_message.author.label(),
                referenced_message.guild_id.map(|guild_id| guild_id.to_string()).as_deref().unwrap_or("@me"),
                referenced_message.channel_id,
                referenced_message.id,
            ),
            None => format_reply_text!(),
        })
    }
}

pub trait EmojiReactionExt {
    fn get_image_url(&self) -> String;
    fn label(&self) -> String;
}

impl EmojiReactionExt for EmojiReactionType {
    fn get_image_url(&self) -> String {
        match self {
            Self::Custom { id, animated, .. } => {
                // Usually not needed but gifs don't animate inside embed thumbnails if you don't include the proper extension
                let ext = if *animated { "gif" } else { "png" };
                format!("https://cdn.discordapp.com/emojis/{id}.{ext}")
            },
            Self::Unicode { name } => {
                let mut hexcodes = name.chars().map(|entry| format!("{:x}", entry as u32)).collect::<Vec<String>>();

                // Trim fe0f (variant selector) if total hexcodes is just 2 (for emojis like :heart: and :heart_exclamation:)
                // The length is limited to 2 because a hexcode like :face_in_clouds: with 4 codepoints (:no_mouth: + zero width joiner + :cloud: + fe0f) exists and it's valid
                // Not sure if this is reliable. I seriously don't want to hardcode all valid Twemojis just for an asset URL
                if hexcodes.len() == 2 && hexcodes[1] == "fe0f" {
                    hexcodes.pop();
                }

                format!("https://raw.githubusercontent.com/jdecked/twemoji/refs/heads/main/assets/72x72/{}.png", hexcodes.join("-"))
            },
        }
    }

    fn label(&self) -> String {
        let emoji_name = match self {
            Self::Custom { name, .. } => name.as_deref().unwrap_or("<unknown>"),
            Self::Unicode { name } => name,
        };

        if let Self::Custom { .. } = self {
            let image_url = self.get_image_url();
            format!("[{emoji_name}]({image_url})")
        } else {
            // Can't make this a hyperlink because it'll mess up the Twemoji/Unicode emoji display
            emoji_name.to_string()
        }
    }
}

pub trait Commas {
    fn commas(&self) -> String;
}

impl<T: Display> Commas for T {
    fn commas(&self) -> String {
        let string = self.to_string();
        let (integer, fraction) = string.split_once('.').unwrap_or((&string, ""));
        let mut formatted_integer = String::new();

        for (index, char) in integer.chars().enumerate() {
            if (integer.len() - index) % 3 == 0 && index != 0 {
                formatted_integer += ",";
            }

            formatted_integer += &char.to_string();
        }

        if fraction.is_empty() { formatted_integer } else { format!("{formatted_integer}.{fraction}") }
    }
}
