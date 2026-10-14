mod anime;
mod components;
mod manga;
pub mod statics;
mod user;

use crate::{
    functions::{format_timestamp, limit_strings, paginate_strings},
    statics::REQWEST,
    structs::api::anilist::components::{AniListCharacter, AniListFuzzyDate, AniListRelation},
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
                    false,
                ));
            }
        }

        if dates.is_empty() { "TBA".into() } else { dates.join(" - ") }
    }

    pub fn format_embed_description<T: Display>(embed: Embed, description: &Option<T>) -> Vec<Vec<Embed>> {
        let description = description.as_ref().map(|description| description.to_string());

        paginate_strings(Document::from(description.as_deref().unwrap_or("N/A")).select("body").text().split(' '), " ", 4096)
            .iter()
            .map(|entry| vec![embed.clone().set_description(entry)])
            .collect()
    }

    fn format_embed_characters(embed: Embed, characters: &[AniListCharacter]) -> Vec<Vec<Embed>> {
        characters
            .iter()
            .map(|character| {
                let mut lines =
                    vec![format!("[**{}**]({})", character.node.name.full, character.node.site_url), format!("-# {}", character.role)];

                let voice_actor = character.voice_actors.as_ref().and_then(|voice_actors| {
                    voice_actors.first().map(|voice_actor| format!("[{}]({})", voice_actor.name.full, voice_actor.site_url))
                });

                if let Some(voice_actor) = voice_actor {
                    lines.push(format!("Voiced by {voice_actor}"));
                }

                vec![embed.clone().set_thumbnail(&character.node.image.large).set_description(lines.join("\n"))]
            })
            .collect()
    }

    fn format_embed_relations(embed: Embed, relations: &[AniListRelation]) -> Vec<Vec<Embed>> {
        let mut categorized = HashMap::new();

        for relation in relations {
            let relation_type = relation.relation_type.to_string();

            if !categorized.contains_key(&relation_type) {
                categorized.insert(relation_type.clone(), vec![format!("**{relation_type}**")]);
            }

            categorized.get_mut(&relation_type).unwrap().push(format!(
                "[{}]({}){}",
                relation.node.title.romaji,
                relation.node.site_url,
                relation.node.format.as_ref().map(|format| format!(" ({format})")).as_deref().unwrap_or_default(),
            ));
        }

        categorized.values().map(|list| vec![embed.clone().set_description(limit_strings(list, "\n", 4096))]).collect()
    }
}
