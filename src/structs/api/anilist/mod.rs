mod anime;
mod components;
mod manga;
pub mod statics;
mod user;

use crate::{
    functions::{format_timestamp, limit_strings, TimestampFormat},
    statics::REQWEST,
    structs::api::anilist::components::{AniListFuzzyDate, AniListRelation},
};
use anyhow::Result;
use nipper::Document;
use serde::de::DeserializeOwned;
use serde_json::{json, Value};
use slashook::{chrono::NaiveDateTime, structs::embeds::Embed};
use std::collections::HashMap;

pub struct AniList;

impl AniList {
    async fn query<T: ToString, U: DeserializeOwned>(query: T, variables: Value) -> Result<U> {
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
            if fuzzy_date.day.is_some() && fuzzy_date.month.is_some() && fuzzy_date.year.is_some() {
                dates.push(format_timestamp(
                    NaiveDateTime::parse_from_str(
                        &format!("{}-{}-{} 00:00", fuzzy_date.year.unwrap(), fuzzy_date.month.unwrap(), fuzzy_date.day.unwrap()),
                        "%F %R",
                    )
                    .unwrap()
                    .timestamp(),
                    TimestampFormat::Simple,
                ));
            }
        }

        match dates.is_empty() {
            true => "TBA".into(),
            false => dates.join(" - "),
        }
    }

    pub fn format_description<T: ToString>(embed: Embed, description: Option<&T>) -> Embed {
        embed.set_description(limit_strings(
            Document::from(&description.map(|description| description.to_string()).unwrap_or("N/A".into()))
                .select("body")
                .text()
                .split('\n'),
            "\n",
            4096,
        ))
    }

    fn format_relations(mut embed: Embed, relations: &Vec<AniListRelation>) -> Embed {
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
                relation.node.format.as_ref().map_or("".into(), |format| format!(" ({format})")),
            ));
        }

        for (relation_type, list) in categorized {
            embed = embed.add_field(relation_type, limit_strings(list, "\n", 1024), false);
        }

        embed
    }
}
