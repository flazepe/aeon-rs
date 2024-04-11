use crate::{
    statics::{regex::MARKDOWN_REGEX, REQWEST},
    traits::Commas,
};
use anyhow::Result;
use regex::Captures;
use serde_json::Value;
use slashook::structs::{
    components::{SelectMenu, SelectOption},
    utils::File,
};
use std::fmt::Display;
use tokio::process::Command;

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
        ("1 week", "1w"),
    ] {
        select_menu = select_menu.add_option(SelectOption::new(label, value));
    }

    select_menu
}

pub async fn eien(command: &str, extra_args: &[&str]) -> Result<File> {
    let mut args = vec!["../eien", command];
    args.extend_from_slice(extra_args);
    Ok(File::new("image.png", Command::new("node").args(args).output().await?.stdout))
}

pub fn escape_markdown(string: &str) -> String {
    MARKDOWN_REGEX
        .replace_all(string, |caps: &Captures| match caps[0].starts_with('\\') {
            true => caps[0].to_string(),
            false => format!("\\{}", &caps[0]),
        })
        .to_string()
}

pub enum TimestampFormat {
    // Duration,
    Simple,
    Full,
}

pub fn format_timestamp<T: Display>(timestamp: T, format: TimestampFormat) -> String {
    let duration = format!("<t:{timestamp}:R>");
    let simple = format!("<t:{timestamp}:D>");
    let full = format!("{simple} ({duration})");

    match format {
        // TimestampFormat::Duration => duration,
        TimestampFormat::Simple => simple,
        TimestampFormat::Full => full,
    }
}

pub async fn hastebin<T: ToString>(string: T) -> Result<String> {
    let domain = "https://haste.zneix.eu";
    let json = REQWEST.post(format!("{domain}/documents")).body(string.to_string()).send().await?.json::<Value>().await?;
    Ok(format!("{domain}/raw/{}", json["key"].as_str().unwrap_or("")))
}

pub fn limit_strings<T: IntoIterator<Item = U>, U: ToString, V: ToString>(iterable: T, delimiter: V, limit: usize) -> String {
    let delimiter = delimiter.to_string();
    let mut strings = iterable.into_iter().map(|stringable| stringable.to_string()).collect::<Vec<String>>();

    while strings.join(&delimiter).len() > limit {
        strings.pop();
    }

    strings.join(&delimiter)
}

pub fn label_num<T: ToString, U: ToString, V: ToString>(amount: T, singular: U, plural: V) -> String {
    let amount = amount.to_string();
    format!("{} {}", amount.commas(), if amount == "1" { singular.to_string() } else { plural.to_string() })
}
