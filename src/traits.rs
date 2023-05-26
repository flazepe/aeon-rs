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
                match format == "gif" && !avatar.starts_with("a_") {
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
                match format == "gif" && !avatar.is_animated() {
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

pub trait Commas {
    fn commas(&self) -> String;
}

impl<T: ToString> Commas for T {
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

        match fraction.is_empty() {
            true => formatted_integer,
            false => format!("{formatted_integer}.{fraction}"),
        }
    }
}
