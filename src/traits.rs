use slashook::structs::users::User;
use twilight_model::user::User as TwilightUser;

pub trait AvatarURL {
    fn avatar_url<T: ToString, U: ToString>(&self, format: T, size: U) -> Option<String>;
    fn display_avatar_url<T: ToString, U: ToString>(&self, format: T, size: U) -> String;
}

impl AvatarURL for User {
    fn avatar_url<T: ToString, U: ToString>(&self, format: T, size: U) -> Option<String> {
        let format = format.to_string();

        self.avatar.as_ref().map(|avatar| {
            format!(
                "https://cdn.discordapp.com/avatars/{}/{}.{}?size={}",
                self.id,
                avatar,
                match format == "gif".to_string() && !avatar.starts_with("a_") {
                    true => "png".into(),
                    false => format,
                },
                size.to_string(),
            )
        })
    }

    fn display_avatar_url<T: ToString, U: ToString>(&self, format: T, size: U) -> String {
        match AvatarURL::avatar_url(self, format, size) {
            Some(avatar_url) => avatar_url,
            None => format!("https://cdn.discordapp.com/embed/avatars/{}.png", (self.id.parse::<u64>().unwrap() >> 22) % 5),
        }
    }
}

impl AvatarURL for TwilightUser {
    fn avatar_url<T: ToString, U: ToString>(&self, format: T, size: U) -> Option<String> {
        let format = format.to_string();

        self.avatar.as_ref().map(|avatar| {
            format!(
                "https://cdn.discordapp.com/avatars/{}/{}.{}?size={}",
                self.id,
                avatar,
                match format == "gif".to_string() && !avatar.is_animated() {
                    true => "png".into(),
                    false => format,
                },
                size.to_string(),
            )
        })
    }

    fn display_avatar_url<T: ToString, U: ToString>(&self, format: T, size: U) -> String {
        match AvatarURL::avatar_url(self, format, size) {
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
