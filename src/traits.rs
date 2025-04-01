use slashook::structs::{messages::Message as SlashookMessage, users::User as SlashookUser};
use std::fmt::Display;
use twilight_model::{channel::Message as TwilightMessage, user::User as TwilightUser};

pub trait UserExt {
    fn label(&self) -> String;
    fn avatar_url<T: Display, U: Display>(&self, format: T, size: U) -> Option<String>;
    fn display_avatar_url<T: Display, U: Display>(&self, format: T, size: U) -> String;
}

impl UserExt for SlashookUser {
    fn label(&self) -> String {
        format!("{} ({})", self.username, self.id)
    }

    fn avatar_url<T: Display, U: Display>(&self, format: T, size: U) -> Option<String> {
        let format = format.to_string();

        self.avatar.as_ref().map(|avatar| {
            format!(
                "https://cdn.discordapp.com/avatars/{user_id}/{avatar}.{format}?size={size}",
                user_id = self.id,
                format = if format == "gif" && !avatar.starts_with("a_") { "png" } else { &format },
            )
        })
    }

    fn display_avatar_url<T: Display, U: Display>(&self, format: T, size: U) -> String {
        match UserExt::avatar_url(self, format, size) {
            Some(avatar_url) => avatar_url,
            None => format!("https://cdn.discordapp.com/embed/avatars/{}.png", (self.id.parse::<u64>().unwrap() >> 22) % 5),
        }
    }
}

impl UserExt for TwilightUser {
    fn label(&self) -> String {
        format!("{} ({})", self.name, self.id)
    }

    fn avatar_url<T: Display, U: Display>(&self, format: T, size: U) -> Option<String> {
        let format = format.to_string();

        self.avatar.as_ref().map(|avatar| {
            format!(
                "https://cdn.discordapp.com/avatars/{user_id}/{avatar}.{format}?size={size}",
                user_id = self.id,
                format = if format == "gif" && !avatar.is_animated() { "png" } else { &format },
            )
        })
    }

    fn display_avatar_url<T: Display, U: Display>(&self, format: T, size: U) -> String {
        match UserExt::avatar_url(self, format, size) {
            Some(avatar_url) => avatar_url,
            None => format!("https://cdn.discordapp.com/embed/avatars/{}.png", (self.id.get() >> 22) % 5),
        }
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

pub trait LimitedVec<T> {
    fn push_limited(&mut self, value: T, limit: usize);
}

impl<T> LimitedVec<T> for Vec<T> {
    fn push_limited(&mut self, value: T, limit: usize) {
        self.push(value);

        let length = self.len();

        if length > limit {
            self.drain(..length - limit);
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
