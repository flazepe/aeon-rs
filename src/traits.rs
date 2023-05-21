use slashook::structs::users::User;
use std::fmt::Display;
use twilight_model::user::User as TwilightUser;

pub trait AvatarURL {
    fn avatar_url<T: Display, U: Display>(&self, format: T, size: U) -> Option<String>;
    fn display_avatar_url<T: Display, U: Display>(&self, format: T, size: U) -> String;
}

impl AvatarURL for User {
    fn avatar_url<T: Display, U: Display>(&self, format: T, size: U) -> Option<String> {
        let mut format = format.to_string();

        if let Some(avatar) = self.avatar.as_ref() {
            if format.as_str() == "gif" && !avatar.starts_with("a_") {
                format = "png".into();
            }
        }

        self.avatar_url(format, size)
    }

    fn display_avatar_url<T: Display, U: Display>(&self, format: T, size: U) -> String {
        match self.avatar_url(format, size) {
            Some(avatar_url) => avatar_url,
            None => format!("https://cdn.discordapp.com/embed/avatars/{}.png", (self.id.parse::<u64>().unwrap() >> 22) % 5),
        }
    }
}

impl AvatarURL for TwilightUser {
    fn avatar_url<T: Display, U: Display>(&self, format: T, size: U) -> Option<String> {
        let mut format = format.to_string();

        if let Some(avatar) = self.avatar.as_ref() {
            if format.as_str() == "gif" && !avatar.is_animated() {
                format = "png".into();
            }
        }

        self.avatar.as_ref().map(|a| format!("https://cdn.discordapp.com/avatars/{}/{a}.{format}?size={size}", self.id))
    }

    fn display_avatar_url<T: Display, U: Display>(&self, format: T, size: U) -> String {
        match self.avatar_url(format, size) {
            Some(avatar_url) => avatar_url,
            None => format!("https://cdn.discordapp.com/embed/avatars/{}.png", (self.id.to_string().parse::<u64>().unwrap() >> 22) % 5),
        }
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

pub trait Tag {
    fn tag(&self) -> String;
}

impl Tag for User {
    fn tag(&self) -> String {
        format!("{}#{}", self.username, self.discriminator)
    }
}

impl Tag for TwilightUser {
    fn tag(&self) -> String {
        format!("{}#{}", self.name, self.discriminator())
    }
}
