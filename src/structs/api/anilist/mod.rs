mod anime;
mod components;
mod manga;

use crate::macros::{format_timestamp, if_else};
use crate::structs::api::anilist::components::AniListFuzzyDate;
use anyhow::Result;
use reqwest::Client;
use serde::de::DeserializeOwned;
use serde_json::{json, Value};
use slashook::chrono::NaiveDateTime;
use std::fmt::Debug;

pub struct AniList {}

impl AniList {
    async fn query<T: ToString, U: DeserializeOwned>(query: T, variables: Value) -> Result<U> {
        Ok(Client::new()
            .post("https://graphql.anilist.co")
            .header("content-type", "application/json")
            .body(
                json!({
                    "query": query.to_string(),
                    "variables": variables
                })
                .to_string(),
            )
            .send()
            .await?
            .json::<U>()
            .await?)
    }

    fn prettify_enum_value<T: Debug>(value: T) -> String {
        format!("{:?}", value)
            .split("_")
            .map(|word| {
                if_else!(
                    ["ONA", "OVA", "TV"].contains(&word),
                    word.into(),
                    format!(
                        "{}{}",
                        word.chars().next().unwrap(),
                        word.chars().skip(1).collect::<String>().to_lowercase()
                    )
                )
            })
            .collect::<Vec<String>>()
            .join(" ")
    }

    fn format_airing_date(start: AniListFuzzyDate, end: AniListFuzzyDate) -> String {
        let mut dates = vec![];

        for fuzzy_date in [start, end] {
            if let Some(date) = if_else!(
                fuzzy_date.day.is_none() || fuzzy_date.month.is_none() || fuzzy_date.year.is_none(),
                None,
                Some(format_timestamp!(
                    NaiveDateTime::parse_from_str(
                        &format!(
                            "{}-{}-{} 00:00:00",
                            fuzzy_date.year.unwrap(),
                            fuzzy_date.month.unwrap(),
                            fuzzy_date.day.unwrap()
                        ),
                        "%Y-%m-%d %H:%M:%S"
                    )
                    .unwrap()
                    .timestamp(),
                    "simple"
                ))
            ) {
                dates.push(date);
            }
        }

        if_else!(dates.is_empty(), "TBA".into(), dates.join(" - "))
    }
}
