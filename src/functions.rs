use anyhow::Result;
use regex::{Captures, Regex};
use reqwest::Client;
use serde_json::Value;
use slashook::structs::components::{SelectMenu, SelectOption};
use std::fmt::Display;
use twilight_model::user::User;

pub fn add_reminder_select_options(mut select_menu: SelectMenu) -> SelectMenu {
    for (label, value) in [
        ("5 minutes", "5m"),
        ("15 minutes", "15m"),
        ("30 minutes", "30m"),
        ("1 hour", "1h"),
        ("3 hours", "3h"),
        ("6 hours", "6h"),
        ("12 hours", "12h"),
        ("24 hours", "24h"),
    ] {
        select_menu = select_menu.add_option(SelectOption::new(label, value));
    }

    select_menu
}

pub fn escape_markdown<T: ToString>(string: T) -> String {
    Regex::new(r"\\?[*_~`]")
        .unwrap()
        .replace_all(&string.to_string(), |caps: &Captures| {
            if caps[0].starts_with("\\") {
                caps[0].to_string()
            } else {
                format!("\\{}", caps[0].to_string())
            }
        })
        .to_string()
}

pub enum TimestampType {
    Simple,
    Full,
    Duration,
}

pub fn format_timestamp<T: Display>(timestamp: T, format: TimestampType) -> String {
    let duration = format!("<t:{}:R>", timestamp);
    let simple = format!("<t:{}:D>", timestamp);
    let full = format!("{simple} ({duration})");

    match format {
        TimestampType::Simple => simple,
        TimestampType::Full => full,
        TimestampType::Duration => duration,
    }
}

pub async fn hastebin<T: ToString>(string: T) -> Result<String> {
    let domain = "https://paste.pythondiscord.com";

    let json = Client::new()
        .post(format!("{domain}/documents"))
        .body(string.to_string())
        .send()
        .await?
        .json::<Value>()
        .await?;

    Ok(format!("{domain}/raw/{}", json["key"].as_str().unwrap_or("")))
}

pub fn twilight_user_to_tag(user: &User) -> String {
    format!("{}#{}", user.name, user.discriminator())
}
