use crate::{
    functions::limit_strings,
    macros::yes_no,
    structs::api::vndb::{
        Vndb,
        statics::{VNDB_EMBED_AUTHOR_ICON_URL, VNDB_EMBED_AUTHOR_URL, VNDB_EMBED_COLOR, VNDB_TRAIT_FIELDS},
    },
    traits::Commas,
};
use anyhow::{Result, bail};
use serde::Deserialize;
use serde_json::json;
use slashook::structs::embeds::Embed;
use std::fmt::Display;

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct VndbTrait {
    pub id: String,
    pub name: String,
    pub aliases: Vec<String>,
    pub description: String,
    pub searchable: bool,
    pub applicable: bool,
    pub group_id: Option<String>,
    pub group_name: Option<String>,
    pub char_count: u64,
}

impl VndbTrait {
    pub fn format(&self) -> Embed {
        let group_name = self.group_name.as_ref().map(|group_name| format!("{group_name}: "));
        let title = format!("{}{}", group_name.as_deref().unwrap_or_default(), self.name);
        let url = format!("https://vndb.org/{}", self.id);
        let aliases = self.aliases.iter().map(|alias| format!("_{alias}_")).collect::<Vec<String>>().join("\n");
        let description = limit_strings(Vndb::clean_bbcode(&self.description).split('\n'), "\n", 1024);
        let searchable = yes_no!(self.searchable);
        let applicable = yes_no!(self.applicable);
        let char_count = self.char_count.commas();

        Embed::new()
            .set_color(VNDB_EMBED_COLOR)
            .unwrap_or_default()
            .set_author("vndb  •  Trait", Some(VNDB_EMBED_AUTHOR_URL), Some(VNDB_EMBED_AUTHOR_ICON_URL))
            .set_title(title)
            .set_url(url)
            .set_description(aliases)
            .add_field("Description", description, false)
            .add_field("Searchable", searchable, true)
            .add_field("Applicable", applicable, true)
            .add_field("Character Count", char_count, true)
    }
}

impl Vndb {
    pub async fn search_trait<T: Display>(query: T) -> Result<Vec<VndbTrait>> {
        let query = query.to_string();

        let results = Self::query(
            "trait",
            if query.starts_with('i') && query.chars().skip(1).all(|char| char.is_numeric()) {
                json!({
                    "filters": ["id", "=", query],
                    "fields": VNDB_TRAIT_FIELDS,
                })
            } else {
                json!({
                    "filters": ["search", "=", query],
                    "fields": VNDB_TRAIT_FIELDS,
                    "sort": "searchrank",
                })
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
