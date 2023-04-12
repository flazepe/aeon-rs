use crate::{
    functions::{if_else_option, limit_string},
    macros::if_else,
    statics::{colors::PRIMARY_COLOR, vndb::CHARACTER_FIELDS},
    structs::api::vndb::{visual_novel::VndbImage, Vndb},
};
use anyhow::{bail, Result};
use serde::Deserialize;
use serde_json::json;
use serde_repr::Deserialize_repr;
use slashook::structs::embeds::Embed;
use std::{
    collections::HashMap,
    fmt::{Display, Formatter, Result as FmtResult},
};

#[derive(Debug, Deserialize)]
pub enum VndbBloodType {
    #[serde(rename = "a")]
    A,

    #[serde(rename = "b")]
    B,

    #[serde(rename = "ab")]
    AB,

    #[serde(rename = "o")]
    O,
}

#[derive(Debug, Deserialize)]
pub enum VndbSex {
    #[serde(rename = "m")]
    Male,

    #[serde(rename = "f")]
    Female,

    #[serde(rename = "b")]
    Both,
}

impl Display for VndbSex {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{:?}", self)
    }
}

#[derive(Deserialize)]
pub enum VndbCharacterRole {
    #[serde(rename = "main")]
    Main,

    #[serde(rename = "primary")]
    Primary,

    #[serde(rename = "side")]
    Side,

    #[serde(rename = "appears")]
    Appears,
}

impl Display for VndbCharacterRole {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "{}",
            match self {
                VndbCharacterRole::Main => "Protagonist",
                VndbCharacterRole::Primary => "Main character",
                VndbCharacterRole::Side => "Side character",
                VndbCharacterRole::Appears => "Makes an appearance",
            }
        )
    }
}

#[derive(Deserialize)]
pub struct VndbCharacterVisualNovel {
    pub id: String,
    pub title: String,
    pub role: VndbCharacterRole,
}

#[derive(Deserialize_repr)]
#[repr(u8)]
pub enum VndbSpoilerLevel {
    NonSpoiler,
    AlmostSpoiler,
    Spoiler,
}

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
    pub spoiler: VndbSpoilerLevel,
    pub lie: bool,
}

#[derive(Deserialize)]
pub struct VndbCharacter {
    pub id: String,
    pub name: String,
    pub original: Option<String>,
    pub aliases: Vec<String>,
    pub description: Option<String>,
    pub image: Option<VndbImage>,
    pub blood_type: Option<VndbBloodType>,
    pub height: Option<u64>,
    pub weight: Option<u64>,
    pub bust: Option<u64>,
    pub waist: Option<u64>,
    pub hips: Option<u64>,
    pub cup: Option<String>,
    pub age: Option<u64>,
    pub birthday: Option<(u64, u64)>,
    pub sex: Option<(Option<VndbSex>, Option<VndbSex>)>,
    pub vns: Vec<VndbCharacterVisualNovel>,
    pub traits: Vec<VndbTrait>,
}

impl VndbCharacter {
    fn _format(&self) -> Embed {
        Embed::new()
            .set_color(PRIMARY_COLOR)
            .unwrap_or_default()
            .set_thumbnail(if_else_option(
                self.image.as_ref(),
                |image| if_else!(image.sexual > 1.0, "".into(), image.url.clone()),
                "".into(),
            ))
            .set_title(self.name.chars().take(256).collect::<String>())
            .set_url(format!("https://vndb.org/{}", self.id))
    }

    pub fn format(self) -> Embed {
        self._format()
            .set_description(
                self.aliases
                    .iter()
                    .map(|alias| format!("_{alias}_"))
                    .collect::<Vec<String>>()
                    .join("\n"),
            )
            .add_field(
                "Sex",
                if_else_option(
                    self.sex,
                    |(sex, spoiler_sex)| {
                        format!(
                            "{}{}",
                            if_else_option(sex, |sex| sex.to_string(), "N/A".into()),
                            if_else_option(
                                spoiler_sex,
                                |spoiler_sex| format!(" (||actually {spoiler_sex}||)"),
                                "".into()
                            )
                        )
                    },
                    "N/A".into(),
                ),
                true,
            )
            .add_field(
                "Age",
                if_else_option(self.age, |age| age.to_string(), "N/A".into()),
                true,
            )
            .add_field(
                "Birthday",
                if_else_option(
                    self.birthday,
                    |birthday| format!("{}/{}", birthday.0, birthday.1),
                    "N/A".into(),
                ),
                true,
            )
            .add_field(
                "Blood Type",
                if_else_option(self.blood_type, |blood_type| format!("{:?}", blood_type), "N/A".into()),
                true,
            )
            .add_field(
                "Height",
                if_else_option(self.height, |height| format!("{height} cm"), "N/A".into()),
                true,
            )
            .add_field(
                "Weight",
                if_else_option(self.weight, |weight| format!("{weight} kg"), "N/A".into()),
                true,
            )
            .add_field(
                "Bust",
                if_else_option(
                    self.bust,
                    |bust| {
                        format!(
                            "{bust} cm{}",
                            if_else_option(self.cup, |cup| format!(" - Cup Size {cup}"), "".into())
                        )
                    },
                    "N/A".into(),
                ),
                true,
            )
            .add_field(
                "Waist",
                if_else_option(self.waist, |waist| format!("{waist} cm"), "N/A".into()),
                true,
            )
            .add_field(
                "Hips",
                if_else_option(self.hips, |hips| format!("{hips} cm"), "N/A".into()),
                true,
            )
    }

    pub fn format_traits(self) -> Embed {
        let mut groups = HashMap::new();

        for character_trait in &self.traits {
            if !groups.contains_key(&character_trait.group_name) {
                groups.insert(character_trait.group_name.clone(), vec![]);
            }

            groups.get_mut(&character_trait.group_name).unwrap().push(if_else!(
                matches!(character_trait.spoiler, VndbSpoilerLevel::NonSpoiler),
                format!(
                    "||[{}](https://vndb.org/{})||",
                    character_trait.name.clone(),
                    character_trait.id,
                ),
                format!(
                    "[{}](https://vndb.org/{})",
                    character_trait.name.clone(),
                    character_trait.id,
                ),
            ));
        }

        let mut embed = self._format();

        for (group_name, traits) in groups {
            embed = embed.add_field(group_name, limit_string(traits.join(", "), ", ", 1024), false);
        }

        embed
    }

    pub fn format_visual_novels(self) -> Embed {
        self._format().set_description(limit_string(
            self.vns
                .into_iter()
                .map(|visual_novel| {
                    format!(
                        "[{}](https://vndb.org/{}) ({})",
                        visual_novel.title, visual_novel.id, visual_novel.role
                    )
                })
                .collect::<Vec<String>>()
                .join("\n"),
            "\n",
            4096,
        ))
    }
}

impl Vndb {
    pub async fn search_character<T: ToString>(&self, query: T) -> Result<Vec<VndbCharacter>> {
        let query = query.to_string();

        let results = self
            .query(
                "character",
                if_else!(
                    query.starts_with("c") && query.chars().skip(1).all(|char| char.is_numeric()),
                    json!({
                        "filters": ["id", "=", query],
                        "fields": CHARACTER_FIELDS,
                    }),
                    json!({
                        "filters": ["search", "=", query],
                        "fields": CHARACTER_FIELDS,
                        "sort": "searchrank",
                    }),
                ),
            )
            .await?
            .results;

        if_else!(results.is_empty(), bail!("Character not found."), Ok(results))
    }
}
