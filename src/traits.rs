use slashook::structs::users::User;
use twilight_model::user::User as TwilightUser;

pub trait AvatarUrl {
    fn avatar_url(&self, format: &str, size: u64) -> Option<String>;
    fn display_avatar_url(&self, format: &str, size: u64) -> String;
}

impl AvatarUrl for User {
    fn avatar_url(&self, format: &str, size: u64) -> Option<String> {
        self.avatar.as_ref().map(|avatar| {
            format!(
                "https://cdn.discordapp.com/avatars/{}/{}.{}?size={}",
                self.id,
                avatar,
                if format == "gif" && !avatar.starts_with("a_") { "png" } else { format },
                size,
            )
        })
    }

    fn display_avatar_url(&self, format: &str, size: u64) -> String {
        AvatarUrl::avatar_url(self, format, size).unwrap_or_else(|| format!("https://cdn.discordapp.com/embed/avatars/{}.png", (self.id.parse::<u64>().unwrap() >> 22) % 5))
    }
}

impl AvatarUrl for TwilightUser {
    fn avatar_url(&self, format: &str, size: u64) -> Option<String> {
        self.avatar.as_ref().map(|avatar| {
            format!(
                "https://cdn.discordapp.com/avatars/{}/{}.{}?size={}",
                self.id,
                avatar,
                if format == "gif" && !avatar.is_animated() { "png" } else { format },
                size,
            )
        })
    }

    fn display_avatar_url(&self, format: &str, size: u64) -> String {
        AvatarUrl::avatar_url(self, format, size).unwrap_or_else(|| format!("https://cdn.discordapp.com/embed/avatars/{}.png", (self.id.get() >> 22) % 5))
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
