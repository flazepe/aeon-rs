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
        let mut split = string.split(".");

        let integral = split.next().unwrap().to_string();
        let fractional = split.collect::<String>();

        let mut formatted_integral = String::new();

        for (index, char) in integral.chars().enumerate() {
            if (integral.len() - index) % 3 == 0 && index != 0 {
                formatted_integral += ",";
            }

            formatted_integral += &char.to_string();
        }

        match fractional.is_empty() {
            true => formatted_integral,
            false => format!("{formatted_integral}.{fractional}"),
        }
    }
}
