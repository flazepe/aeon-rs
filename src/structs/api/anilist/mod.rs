mod anime;
mod components;
mod manga;
mod user;

use crate::{
    functions::{format_timestamp, if_else_option, limit_string, TimestampFormat},
    macros::if_else,
    structs::api::anilist::components::{AniListFuzzyDate, AniListRelation},
};
use anyhow::Result;
use nipper::Document;
use reqwest::Client;
use serde::de::DeserializeOwned;
use serde_json::{json, Value};
use slashook::{chrono::NaiveDateTime, structs::embeds::Embed};
use std::{collections::HashMap, fmt::Debug};

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

    pub fn format_enum_value<T: Debug>(value: T) -> String {
        format!("{:?}", value)
            .split("_")
            .map(|word| {
                if_else!(
                    ["ONA", "OVA", "TV"].contains(&word),
                    word.into(),
                    format!(
                        "{}{}",
                        word.chars().next().unwrap(),
                        word.chars().skip(1).collect::<String>().to_lowercase(),
                    ),
                )
            })
            .collect::<Vec<String>>()
            .join(" ")
    }

    fn format_airing_date(start: AniListFuzzyDate, end: AniListFuzzyDate) -> String {
        let mut dates = vec![];

        for fuzzy_date in [start, end] {
            if fuzzy_date.day.is_some() && fuzzy_date.month.is_some() && fuzzy_date.year.is_some() {
                dates.push(format_timestamp(
                    NaiveDateTime::parse_from_str(
                        &format!(
                            "{}-{}-{} 00:00:00",
                            fuzzy_date.year.unwrap(),
                            fuzzy_date.month.unwrap(),
                            fuzzy_date.day.unwrap()
                        ),
                        "%Y-%m-%d %H:%M:%S",
                    )
                    .unwrap()
                    .timestamp(),
                    TimestampFormat::Simple,
                ));
            }
        }

        if_else!(dates.is_empty(), "TBA".into(), dates.join(" - "))
    }

    pub fn format_description(embed: Embed, description: Option<String>) -> Embed {
        embed.set_description(limit_string(
            Document::from(&description.unwrap_or("N/A".into()))
                .select("body")
                .text()
                .split("\n")
                .map(|str| str.to_string())
                .collect::<Vec<String>>()
                .join("\n"),
            "\n",
            4096,
        ))
    }

    fn format_relations(mut embed: Embed, relations: Vec<AniListRelation>) -> Embed {
        let mut categorized = HashMap::new();

        for relation in relations {
            let relation_type = AniList::format_enum_value(relation.relation_type);

            if !categorized.contains_key(&relation_type) {
                categorized.insert(relation_type.clone(), vec![]);
            }

            categorized.get_mut(&relation_type).unwrap().push(format!(
                "[{}]({}){}",
                relation.node.title.romaji,
                relation.node.site_url,
                if_else_option(
                    relation.node.format,
                    |format| format!(" ({})", AniList::format_enum_value(format)),
                    "".into()
                )
            ));
        }

        for (relation_type, list) in categorized {
            embed = embed.add_field(relation_type, limit_string(list.join("\n"), "\n", 1024), false);
        }

        embed
    }
}
