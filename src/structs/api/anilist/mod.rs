mod anime;
mod components;
mod manga;
pub mod statics;
mod user;

use crate::{
    functions::{TimestampFormat, format_timestamp, limit_strings},
    statics::REQWEST,
    structs::api::anilist::components::{AniListFuzzyDate, AniListRelation},
};
use anyhow::Result;
use nipper::Document;
use serde::de::DeserializeOwned;
use serde_json::{Value, json};
use slashook::{chrono::NaiveDateTime, structs::embeds::Embed};
use std::{collections::HashMap, fmt::Display};

pub struct AniList;

impl AniList {
    async fn query<T: Display, U: DeserializeOwned>(query: T, variables: Value) -> Result<U> {
        Ok(REQWEST
            .post("https://graphql.anilist.co")
            .json(&json!({
                "query": query.to_string(),
                "variables": variables,
            }))
            .send()
            .await?
            .json::<U>()
            .await?)
    }

    fn format_airing_date(start: &AniListFuzzyDate, end: &AniListFuzzyDate) -> String {
        let mut dates = vec![];

        for fuzzy_date in [start, end] {
            if let (Some(year), Some(month), Some(day)) = (fuzzy_date.year, fuzzy_date.month, fuzzy_date.day) {
                dates.push(format_timestamp(
                    NaiveDateTime::parse_from_str(&format!("{year}-{month}-{day} 00:00"), "%F %R").unwrap().and_utc().timestamp(),
                    TimestampFormat::Simple,
                ));
            }
        }

        if dates.is_empty() { "TBA".into() } else { dates.join(" - ") }
    }

    pub fn format_embed_description<T: Display>(embed: Embed, description: Option<&T>) -> Embed {
        let description = description.map(|description| description.to_string());

        embed.set_description(limit_strings(
            Document::from(description.as_deref().unwrap_or("N/A")).select("body").text().split('\n'),
            "\n",
            4096,
        ))
    }

    fn format_embed_relations(mut embed: Embed, relations: &Vec<AniListRelation>) -> Embed {
        let mut categorized = HashMap::new();

        for relation in relations {
            let relation_type = relation.relation_type.to_string();

            if !categorized.contains_key(&relation_type) {
                categorized.insert(relation_type.clone(), vec![]);
            }

            categorized.get_mut(&relation_type).unwrap().push(format!(
                "[{}]({}){}",
                relation.node.title.romaji,
                relation.node.site_url,
                relation.node.format.as_ref().map(|format| format!(" ({format})")).as_deref().unwrap_or(""),
            ));
        }

        for (relation_type, list) in categorized {
            embed = embed.add_field(relation_type, limit_strings(list, "\n", 1024), false);
        }

        embed
    }
}
