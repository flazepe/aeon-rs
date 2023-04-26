use crate::{
    functions::limit_string,
    statics::colors::PRIMARY_COLOR,
    structs::api::vndb::{statics::CHARACTER_FIELDS, visual_novel::VndbImage, Vndb},
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
#[serde(rename_all = "lowercase")]
pub enum VndbBloodType {
    A,
    B,
    AB,
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

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum VndbCharacterRole {
    Main,
    Primary,
    Side,
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
            .set_thumbnail(self.image.as_ref().map_or("".into(), |image| match image.sexual > 1.0 {
                true => "".into(),
                false => image.url.clone(),
            }))
            .set_title(self.name.chars().take(256).collect::<String>())
            .set_url(format!("https://vndb.org/{}", self.id))
    }

    pub fn format(&self) -> Embed {
        self._format()
            .set_description(self.aliases.iter().map(|alias| format!("_{alias}_")).collect::<Vec<String>>().join("\n"))
            .add_field(
                "Sex",
                self.sex.as_ref().map_or("N/A".into(), |(sex, spoiler_sex)| {
                    format!(
                        "{}{}",
                        sex.as_ref().map_or("N/A".into(), |sex| format!("{sex:?}")),
                        spoiler_sex.as_ref().map_or("".into(), |spoiler_sex| format!(" (||actually {spoiler_sex:?}||)"))
                    )
                }),
                true,
            )
            .add_field("Age", self.age.map_or("N/A".into(), |age| age.to_string()), true)
            .add_field("Birthday", self.birthday.map_or("N/A".into(), |birthday| format!("{}/{}", birthday.0, birthday.1)), true)
            .add_field("Blood Type", self.blood_type.as_ref().map_or("N/A".into(), |blood_type| format!("{blood_type:?}")), true)
            .add_field("Height", self.height.map_or("N/A".into(), |height| format!("{height} cm")), true)
            .add_field("Weight", self.weight.map_or("N/A".into(), |weight| format!("{weight} kg")), true)
            .add_field(
                "Bust",
                self.bust.map_or("N/A".into(), |bust| {
                    format!("{bust} cm{}", self.cup.as_ref().map_or("".into(), |cup| format!(" - Cup Size {cup}")))
                }),
                true,
            )
            .add_field("Waist", self.waist.map_or("N/A".into(), |waist| format!("{waist} cm")), true)
            .add_field("Hips", self.hips.map_or("N/A".into(), |hips| format!("{hips} cm")), true)
    }

    pub fn format_traits(&self) -> Embed {
        let mut groups = HashMap::new();

        for character_trait in &self.traits {
            if !groups.contains_key(&character_trait.group_name) {
                groups.insert(character_trait.group_name.clone(), vec![]);
            }

            groups.get_mut(&character_trait.group_name).unwrap().push(match character_trait.spoiler {
                VndbSpoilerLevel::NonSpoiler => format!("||[{}](https://vndb.org/{})||", character_trait.name.clone(), character_trait.id),
                _ => format!("[{}](https://vndb.org/{})", character_trait.name.clone(), character_trait.id),
            });
        }

        let mut embed = self._format();

        for (group_name, traits) in groups {
            embed = embed.add_field(group_name, limit_string(traits.join(", "), ", ", 1024), false);
        }

        embed
    }

    pub fn format_visual_novels(&self) -> Embed {
        self._format().set_description(limit_string(
            self.vns
                .iter()
                .map(|visual_novel| format!("[{}](https://vndb.org/{}) ({})", visual_novel.title, visual_novel.id, visual_novel.role))
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
                match query.starts_with("c") && query.chars().skip(1).all(|char| char.is_numeric()) {
                    true => json!({
                        "filters": ["id", "=", query],
                        "fields": CHARACTER_FIELDS,
                    }),
                    false => json!({
                        "filters": ["search", "=", query],
                        "fields": CHARACTER_FIELDS,
                        "sort": "searchrank",
                    }),
                },
            )
            .await?
            .results;

        if results.is_empty() {
            bail!("Character not found.");
        }

        Ok(results)
    }
}
