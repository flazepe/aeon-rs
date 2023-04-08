use crate::{
    macros::{and_then_or, if_else},
    statics::{colors::PRIMARY_COLOR, vndb::CHARACTER_FIELDS},
    structs::api::vndb::{visual_novel::VndbImage, Vndb},
};
use anyhow::{bail, Result};
use serde::Deserialize;
use serde_json::json;
use serde_repr::Deserialize_repr;
use slashook::structs::embeds::Embed;
use std::fmt::{Display, Formatter, Result as FmtResult};

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
    pub fn format(self) -> Embed {
        Embed::new()
            .set_color(PRIMARY_COLOR)
            .unwrap_or_default()
            .set_thumbnail(and_then_or!(
                self.image,
                |image| Some(if_else!(image.sexual > 1.0, "".into(), image.url)),
                "".into()
            ))
            .set_title(self.name.chars().take(256).collect::<String>())
            .set_url(format!("https://vndb.org/{}", self.id))
            .set_description(
                self.aliases
                    .iter()
                    .map(|alias| format!("_{alias}_"))
                    .collect::<Vec<String>>()
                    .join("\n"),
            )
            .add_field(
                "Sex",
                and_then_or!(
                    self.sex,
                    |(sex, spoiler_sex)| Some(format!(
                        "{}{}",
                        and_then_or!(sex, |sex| Some(sex.to_string()), "N/A".into()),
                        and_then_or!(
                            spoiler_sex,
                            |spoiler_sex| Some(format!(" ||actually {spoiler_sex}||")),
                            "".into()
                        )
                    )),
                    "N/A".into()
                ),
                true,
            )
            .add_field(
                "Age",
                and_then_or!(self.age, |age| Some(age.to_string()), "N/A".into()),
                true,
            )
            .add_field(
                "Birthday",
                and_then_or!(
                    self.birthday,
                    |birthday| Some(format!("{}/{}", birthday.0, birthday.1)),
                    "N/A".into()
                ),
                true,
            )
            .add_field(
                "Blood Type",
                and_then_or!(
                    self.blood_type,
                    |blood_type| Some(format!("{:?}", blood_type)),
                    "N/A".into()
                ),
                true,
            )
            .add_field(
                "Height",
                and_then_or!(self.height, |height| Some(format!("{height} cm")), "N/A".into()),
                true,
            )
            .add_field(
                "Weight",
                and_then_or!(self.weight, |weight| Some(format!("{weight} kg")), "N/A".into()),
                true,
            )
            .add_field(
                "Bust",
                and_then_or!(
                    self.bust,
                    |bust| Some(format!(
                        "{bust} cm{}",
                        and_then_or!(self.cup, |cup| Some(format!(" - Cup Size {cup}")), "".into())
                    )),
                    "N/A".into()
                ),
                true,
            )
            .add_field(
                "Waist",
                and_then_or!(self.waist, |waist| Some(format!("{waist} cm")), "N/A".into()),
                true,
            )
            .add_field(
                "Hips",
                and_then_or!(self.hips, |hips| Some(format!("{hips} cm")), "N/A".into()),
                true,
            )
            .add_field(
                "Traits",
                {
                    let mut traits = self
                        .traits
                        .into_iter()
                        .map(|character_trait| {
                            format!(
                                "[{}](https://vndb.org/{})",
                                if_else!(
                                    matches!(character_trait.spoiler, VndbSpoilerLevel::NonSpoiler),
                                    character_trait.name,
                                    format!("||{}||", character_trait.name)
                                ),
                                character_trait.id
                            )
                        })
                        .collect::<Vec<String>>();

                    while traits.join(", ").len() > 1024 {
                        traits.pop();
                    }

                    traits.join(", ")
                },
                false,
            )
            .add_field(
                "Visual Novels",
                {
                    let mut visual_novels = self
                        .vns
                        .into_iter()
                        .map(|visual_novel| {
                            format!(
                                "[{}](https://vndb.org/{}) ({})",
                                visual_novel.title, visual_novel.id, visual_novel.role
                            )
                        })
                        .collect::<Vec<String>>();

                    while visual_novels.join("\n").len() > 1024 {
                        visual_novels.pop();
                    }

                    visual_novels.join("\n")
                },
                false,
            )
    }
}

impl Vndb {
    pub async fn get_character<T: ToString>(&self, id: T) -> Result<VndbCharacter> {
        let mut results = self
            .query(
                "character",
                json!({
                    "filters": ["id", "=", id.to_string()],
                    "fields": CHARACTER_FIELDS
                }),
            )
            .await?
            .results;

        if_else!(results.is_empty(), bail!("Character not found."), Ok(results.remove(0)))
    }

    pub async fn search_character<T: ToString>(&self, query: T) -> Result<Vec<VndbCharacter>> {
        let results = self
            .query(
                "character",
                json!({
                    "filters": ["search", "=", query.to_string()],
                    "fields": CHARACTER_FIELDS,
                    "sort": "searchrank"
                }),
            )
            .await?
            .results;

        if_else!(results.is_empty(), bail!("Character not found."), Ok(results))
    }
}
