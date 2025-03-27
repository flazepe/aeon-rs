use crate::{
    functions::limit_strings,
    macros::yes_no,
    structs::api::vndb::{
        Vndb,
        statics::{VNDB_EMBED_AUTHOR_ICON_URL, VNDB_EMBED_AUTHOR_URL, VNDB_EMBED_COLOR, VNDB_TAG_FIELDS},
    },
    traits::Commas,
};
use anyhow::{Result, bail};
use serde::Deserialize;
use serde_json::json;
use slashook::structs::embeds::Embed;
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Deserialize, Debug)]
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
                Self::SexualContext => "Sexual Content".into(),
                _ => format!("{self:?}"),
            },
        )
    }
}

#[derive(Deserialize, Debug)]
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
        let title = format!("{} ({})", self.name, self.category);
        let url = format!("https://vndb.org/{}", self.id);
        let aliases = self.aliases.iter().map(|alias| format!("_{alias}_")).collect::<Vec<String>>().join("\n");
        let description = limit_strings(Vndb::clean_bbcode(&self.description).split('\n'), "\n", 1024);
        let searchable = yes_no!(self.searchable);
        let applicable = yes_no!(self.applicable);
        let vn_count = self.vn_count.commas();

        Embed::new()
            .set_color(VNDB_EMBED_COLOR)
            .unwrap_or_default()
            .set_author("vndb  •  Tag", Some(VNDB_EMBED_AUTHOR_URL), Some(VNDB_EMBED_AUTHOR_ICON_URL))
            .set_title(title)
            .set_url(url)
            .set_description(aliases)
            .add_field("Description", description, false)
            .add_field("Searchable", searchable, true)
            .add_field("Applicable", applicable, true)
            .add_field("Visual Novel Count", vn_count, true)
    }
}

impl Vndb {
    pub async fn search_tag<T: Display>(query: T) -> Result<Vec<VndbTag>> {
        let query = query.to_string();

        let results = Self::query(
            "tag",
            if query.starts_with('g') && query.chars().skip(1).all(|char| char.is_numeric()) {
                json!({
                    "filters": ["id", "=", query],
                    "fields": VNDB_TAG_FIELDS,
                })
            } else {
                json!({
                    "filters": ["search", "=", query],
                    "fields": VNDB_TAG_FIELDS,
                    "sort": "searchrank",
                })
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
