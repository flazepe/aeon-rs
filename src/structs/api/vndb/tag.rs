use crate::{
    macros::{if_else, yes_no},
    statics::{colors::PRIMARY_COLOR, vndb::TAG_FIELDS},
    structs::api::vndb::Vndb,
};
use anyhow::{bail, Result};
use serde::Deserialize;
use serde_json::json;
use slashook::structs::embeds::Embed;
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug, Deserialize)]
pub enum VndbTagCategory {
    #[serde(rename = "cont")]
    Content,

    #[serde(rename = "ero")]
    SexualContext,

    #[serde(rename = "tech")]
    Technical,
}

impl Display for VndbTagCategory {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "{}",
            if_else!(
                matches!(self, VndbTagCategory::SexualContext),
                "Sexual Content".into(),
                format!("{:?}", self)
            )
        )
    }
}

#[derive(Deserialize)]
pub struct VndbTag {
    pub id: String,
    pub name: String,
    pub aliases: Vec<String>,
    pub description: String,
    pub category: VndbTagCategory,
    pub searchable: bool,
    pub applicable: bool,
    pub vn_count: u64,
}

impl VndbTag {
    pub fn format(self) -> Embed {
        Embed::new()
            .set_color(PRIMARY_COLOR)
            .unwrap_or_default()
            .set_title(format!("{} ({})", self.name, self.category.to_string()))
            .set_url(format!("https://vndb.org/{}", self.id))
            .set_description(
                self.aliases
                    .iter()
                    .map(|alias| format!("_{alias}_"))
                    .collect::<Vec<String>>()
                    .join("\n"),
            )
            .add_field("Description", self.description, false)
            .add_field("Searchable", yes_no!(self.searchable), true)
            .add_field("Applicable", yes_no!(self.applicable), true)
            .add_field("Visual Novel Count", self.vn_count.to_string(), true)
    }
}

impl Vndb {
    pub async fn search_tag<T: ToString>(&self, query: T) -> Result<Vec<VndbTag>> {
        let query = query.to_string();

        let results = self
            .query(
                "tag",
                if_else!(
                    query.starts_with("g") && query.chars().skip(1).all(|char| char.is_numeric()),
                    json!({
                        "filters": ["id", "=", query],
                        "fields": TAG_FIELDS
                    }),
                    json!({
                        "filters": ["search", "=", query],
                        "fields": TAG_FIELDS,
                        "sort": "searchrank"
                    })
                ),
            )
            .await?
            .results;

        if_else!(results.is_empty(), bail!("Tag not found."), Ok(results))
    }
}