use crate::{
    functions::limit_strings,
    macros::yes_no,
    statics::colors::PRIMARY_COLOR,
    structs::api::vndb::{statics::TRAIT_FIELDS, Vndb},
    traits::Commas,
};
use anyhow::{bail, Result};
use serde::Deserialize;
use serde_json::json;
use slashook::structs::embeds::Embed;
use std::fmt::Display;

#[derive(Deserialize)]
pub struct VndbTrait {
    pub id: String,
    pub name: String,
    pub aliases: Vec<String>,
    pub description: String,
    pub searchable: bool,
    pub applicable: bool,
    pub group_id: String,
    pub group_name: String,
    pub char_count: u64,
}

impl VndbTrait {
    pub fn format(&self) -> Embed {
        Embed::new()
            .set_color(PRIMARY_COLOR)
            .unwrap_or_default()
            .set_title(format!("{}: {}", self.group_name, self.name))
            .set_url(format!("https://vndb.org/{}", self.id))
            .set_description(self.aliases.iter().map(|alias| format!("_{alias}_")).collect::<Vec<String>>().join("\n"))
            .add_field("Description", limit_strings(Vndb::clean_bbcode(&self.description).split('\n'), "\n", 1024), false)
            .add_field("Searchable", yes_no!(self.searchable), true)
            .add_field("Applicable", yes_no!(self.applicable), true)
            .add_field("Character Count", self.char_count.commas(), true)
    }
}

impl Vndb {
    pub async fn search_trait<T: Display>(query: T) -> Result<Vec<VndbTrait>> {
        let query = query.to_string();

        let results = Vndb::query(
            "trait",
            match query.starts_with('i') && query.chars().skip(1).all(|char| char.is_numeric()) {
                true => json!({
                    "filters": ["id", "=", query],
                    "fields": TRAIT_FIELDS,
                }),
                false => json!({
                    "filters": ["search", "=", query],
                    "fields": TRAIT_FIELDS,
                    "sort": "searchrank",
                }),
            },
        )
        .await?
        .results;

        if results.is_empty() {
            bail!("Trait not found.");
        }

        Ok(results)
    }
}
