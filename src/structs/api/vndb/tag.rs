use crate::{
    functions::limit_string,
    macros::yes_no,
    statics::colors::PRIMARY_COLOR,
    structs::api::vndb::{statics::TAG_FIELDS, Vndb},
    traits::Commas,
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
            match self {
                VndbTagCategory::SexualContext => "Sexual Content".into(),
                _ => format!("{self:?}"),
            },
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
    pub fn format(&self) -> Embed {
        Embed::new()
            .set_color(PRIMARY_COLOR)
            .unwrap_or_default()
            .set_title(format!("{} ({})", self.name, self.category))
            .set_url(format!("https://vndb.org/{}", self.id))
            .set_description(self.aliases.iter().map(|alias| format!("_{alias}_")).collect::<Vec<String>>().join("\n"))
            .add_field("Description", limit_string(Vndb::clean_bbcode(&self.description), "\n", 1024), false)
            .add_field("Searchable", yes_no!(self.searchable), true)
            .add_field("Applicable", yes_no!(self.applicable), true)
            .add_field("Visual Novel Count", self.vn_count.commas(), true)
    }
}

impl Vndb {
    pub async fn search_tag<T: ToString>(query: T) -> Result<Vec<VndbTag>> {
        let query = query.to_string();

        let results = Vndb::query(
            "tag",
            match query.starts_with('g') && query.chars().skip(1).all(|char| char.is_numeric()) {
                true => json!({
                    "filters": ["id", "=", query],
                    "fields": TAG_FIELDS,
                }),
                false => json!({
                    "filters": ["search", "=", query],
                    "fields": TAG_FIELDS,
                    "sort": "searchrank",
                }),
            },
        )
        .await?
        .results;

        if results.is_empty() {
            bail!("Tag not found.");
        }

        Ok(results)
    }
}
