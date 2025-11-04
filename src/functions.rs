use crate::{
    statics::{REQWEST, regex::MARKDOWN_REGEX},
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
        ("1 day", "1d"),
        ("1 week", "1w"),
    ] {
        select_menu = select_menu.add_option(SelectOption::new(label, value));
    }

    select_menu
}

pub async fn eien<T: Display>(command: T, extra_args: &[&str]) -> Result<File> {
    let command = command.to_string();
    let mut args = vec!["../eien", &command];
    args.extend_from_slice(extra_args);
    Ok(File::new("image.png", Command::new("node").args(args).output().await?.stdout))
}

pub fn escape_markdown<T: Display>(string: T) -> String {
    MARKDOWN_REGEX
        .replace_all(&string.to_string(), |captures: &Captures| {
            if captures[0].starts_with('\\') { captures[0].to_string() } else { format!("\\{}", &captures[0]) }
        })
        .trim()
        .to_string()
}

pub fn format_timestamp<T: Display>(timestamp: T, full: bool) -> String {
    if full { format!("<t:{timestamp}:F> (<t:{timestamp}:R>)") } else { format!("<t:{timestamp}:D>") }
}

pub async fn hastebin<T: Display>(string: T) -> Result<String> {
    let domain = "https://haste.zneix.eu";
    let json = REQWEST.post(format!("{domain}/documents")).body(string.to_string()).send().await?.json::<Value>().await?;
    Ok(format!("{domain}/raw/{}", json["key"].as_str().unwrap_or_default()))
}

pub fn limit_strings<T: IntoIterator<Item = U>, U: Display, V: Display>(iterable: T, delimiter: V, limit: usize) -> String {
    let delimiter = delimiter.to_string();
    let mut strings = iterable.into_iter().map(|to_string| to_string.to_string()).collect::<Vec<String>>();

    while strings.join(&delimiter).len() > limit {
        strings.pop();
    }

    strings.join(&delimiter)
}

pub fn label_num<T: Display, U: Display, V: Display>(amount: T, singular: U, plural: V) -> String {
    let amount = amount.to_string().commas();
    format!("{amount} {}", if amount == "1" { singular.to_string() } else { plural.to_string() })
}
