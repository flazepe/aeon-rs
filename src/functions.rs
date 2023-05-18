use crate::statics::{regex::MARKDOWN_REGEX, REQWEST};
use anyhow::Result;
use regex::Captures;
use serde_json::Value;
use slashook::structs::components::{SelectMenu, SelectOption};
use std::fmt::Display;
use thousands::Separable;

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
    MARKDOWN_REGEX
        .replace_all(&string.to_string(), |caps: &Captures| match caps[0].starts_with("\\") {
            true => caps[0].to_string(),
            false => format!("\\{}", caps[0].to_string()),
        })
        .to_string()
}

pub enum TimestampFormat {
    Duration,
    Simple,
    Full,
}

pub fn format_timestamp<T: Display>(timestamp: T, format: TimestampFormat) -> String {
    let duration = format!("<t:{}:R>", timestamp);
    let simple = format!("<t:{}:D>", timestamp);
    let full = format!("{simple} ({duration})");

    match format {
        TimestampFormat::Duration => duration,
        TimestampFormat::Simple => simple,
        TimestampFormat::Full => full,
    }
}

pub async fn hastebin<T: ToString>(string: T) -> Result<String> {
    let domain = "https://paste.pythondiscord.com";
    let json = REQWEST.post(format!("{domain}/documents")).body(string.to_string()).send().await?.json::<Value>().await?;
    Ok(format!("{domain}/raw/{}", json["key"].as_str().unwrap_or("")))
}

pub fn limit_string<T: ToString, U: ToString>(string: T, delimiter: U, limit: usize) -> String {
    let delimiter = delimiter.to_string();
    let mut split = string.to_string().split(&delimiter).map(|string| string.to_string()).collect::<Vec<String>>();

    while split.join(&delimiter).len() > limit {
        split.pop();
    }

    split.join(&delimiter)
}

// This does not account for irregular nouns for now
pub fn plural<T: ToString, U: ToString>(amount: T, singular: U) -> String {
    let amount = amount.to_string();
    let mut subject = singular.to_string().to_lowercase();
    let second_last_letter = subject.chars().skip(subject.len().max(2) - 2).take(1).collect::<String>();

    if amount != "1".to_string() {
        loop {
            // -s/-ch/-sh/-x/-z
            if ["s", "ch", "sh", "x", "z"].iter().any(|entry| subject.ends_with(entry))
                    // -consonant + o with some exceptions
                    || (!["a", "e", "i", "o", "u"].contains(&second_last_letter.as_str())
                        && subject.ends_with("o")
                        && !["piano", "photo"].contains(&subject.as_str()))
            {
                subject = format!("{}es", subject);
                break;
            }

            // -f/-fe with some exceptions
            if ["f", "fe"].iter().any(|entry| subject.ends_with(entry))
                && !["belief", "cliff", "relief", "roof"].contains(&subject.as_str())
            {
                subject = format!(
                    "{}ves",
                    subject
                        .chars()
                        .take(match subject.ends_with("fe") {
                            true => subject.len() - 2,
                            false => subject.len() - 1,
                        })
                        .collect::<String>()
                );

                break;
            }

            // -consonant + y
            if !["a", "e", "i", "o", "u"].contains(&second_last_letter.as_str()) && subject.ends_with("y") {
                subject = format!("{}ies", subject.chars().take(subject.len() - 1).collect::<String>());
                break;
            }

            // Regular nouns
            subject = format!("{}s", subject);
            break;
        }
    }

    format!("{} {subject}", amount.separate_with_commas())
}
