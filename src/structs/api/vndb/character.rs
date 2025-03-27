use crate::{
    functions::limit_strings,
    structs::api::vndb::{
        Vndb,
        statics::{VNDB_CHARACTER_FIELDS, VNDB_EMBED_AUTHOR_ICON_URL, VNDB_EMBED_AUTHOR_URL, VNDB_EMBED_COLOR},
        visual_novel::VndbImage,
    },
    traits::Commas,
};
use anyhow::{Result, bail};
use serde::Deserialize;
use serde_json::json;
use serde_repr::Deserialize_repr;
use slashook::structs::embeds::Embed;
use std::{
    collections::HashMap,
    fmt::{Display, Formatter, Result as FmtResult},
};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum VndbBloodType {
    A,
    B,
    AB,
    O,
}

#[derive(Deserialize, Debug)]
pub enum VndbSex {
    #[serde(rename = "m")]
    Male,

    #[serde(rename = "f")]
    Female,

    #[serde(rename = "b")]
    Both,
}

#[derive(Deserialize, Debug)]
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
                Self::Main => "Protagonist",
                Self::Primary => "Main character",
                Self::Side => "Side character",
                Self::Appears => "Makes an appearance",
            },
        )
    }
}

#[derive(Deserialize, Debug)]
pub struct VndbCharacterVisualNovel {
    pub id: String,
    pub title: String,
    pub role: VndbCharacterRole,
}

#[derive(Deserialize_repr, Debug)]
#[repr(u8)]
pub enum VndbSpoilerLevel {
    NonSpoiler,
    AlmostSpoiler,
    Spoiler,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
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

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
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
        let thumbnail = self.image.as_ref().map_or("", |image| if image.sexual > 1.0 { "" } else { image.url.as_str() });
        let title = self.name.chars().take(256).collect::<String>();
        let url = format!("https://vndb.org/{}", self.id);

        Embed::new()
            .set_color(VNDB_EMBED_COLOR)
            .unwrap_or_default()
            .set_thumbnail(thumbnail)
            .set_author("vndb  •  Character", Some(VNDB_EMBED_AUTHOR_URL), Some(VNDB_EMBED_AUTHOR_ICON_URL))
            .set_title(title)
            .set_url(url)
    }

    pub fn format(&self) -> Embed {
        let aliases = self.aliases.iter().map(|alias| format!("_{alias}_")).collect::<Vec<String>>().join("\n");
        let sex = self.sex.as_ref().map(|(sex, spoiler_sex)| {
            format!(
                "{}{}",
                sex.as_ref().map(|sex| format!("{sex:?}")).as_deref().unwrap_or("N/A"),
                spoiler_sex.as_ref().map(|spoiler_sex| format!(" (||actually {spoiler_sex:?}||)")).as_deref().unwrap_or(""),
            )
        });
        let age = self.age.map(|age| age.commas());
        let birthday = self.birthday.map(|birthday| format!("{}/{}", birthday.0, birthday.1));
        let blood_type = self.blood_type.as_ref().map(|blood_type| format!("{blood_type:?}"));
        let height = self.height.map(|height| format!("{} cm", height.commas()));
        let weight = self.weight.map(|weight| format!("{} kg", weight.commas()));
        let bust =
            self.bust.map(|bust| format!("{bust} cm{}", self.cup.as_ref().map(|cup| format!(" - Cup Size {cup}")).unwrap_or_default()));
        let waist = self.waist.map(|waist| format!("{waist} cm"));
        let hips = self.hips.map(|hips| format!("{hips} cm"));

        self._format()
            .set_description(aliases)
            .add_field("Sex", sex.as_deref().unwrap_or("N/A"), true)
            .add_field("Age", age.as_deref().unwrap_or("N/A"), true)
            .add_field("Birthday", birthday.as_deref().unwrap_or("N/A"), true)
            .add_field("Blood Type", blood_type.as_deref().unwrap_or("N/A"), true)
            .add_field("Height", height.as_deref().unwrap_or("N/A"), true)
            .add_field("Weight", weight.as_deref().unwrap_or("N/A"), true)
            .add_field("Bust", bust.as_deref().unwrap_or("N/A"), true)
            .add_field("Waist", waist.as_deref().unwrap_or("N/A"), true)
            .add_field("Hips", hips.as_deref().unwrap_or("N/A"), true)
    }

    pub fn format_traits(&self) -> Embed {
        let mut groups = HashMap::new();

        for character_trait in &self.traits {
            if !groups.contains_key(&character_trait.group_name) {
                groups.insert(&character_trait.group_name, vec![]);
            }

            let mut text = format!("[{}](https://vndb.org/{})", character_trait.name, character_trait.id);

            if !matches!(character_trait.spoiler, VndbSpoilerLevel::NonSpoiler) {
                text = format!("||{text}||");
            }

            groups.get_mut(&character_trait.group_name).unwrap().push(text);
        }

        let mut embed = self._format();

        for (group_name, traits) in groups {
            embed = embed.add_field(group_name, limit_strings(traits, ", ", 1024), false);
        }

        embed
    }

    pub fn format_visual_novels(&self) -> Embed {
        self._format().set_description(limit_strings(
            self.vns
                .iter()
                .map(|visual_novel| format!("[{}](https://vndb.org/{}) ({})", visual_novel.title, visual_novel.id, visual_novel.role)),
            "\n",
            4096,
        ))
    }
}

impl Vndb {
    pub async fn search_character<T: Display>(query: T) -> Result<Vec<VndbCharacter>> {
        let query = query.to_string();

        let results = Self::query(
            "character",
            if query.starts_with('c') && query.chars().skip(1).all(|char| char.is_numeric()) {
                json!({
                    "filters": ["id", "=", query],
                    "fields": VNDB_CHARACTER_FIELDS,
                })
            } else {
                json!({
                    "filters": ["search", "=", query],
                    "fields": VNDB_CHARACTER_FIELDS,
                    "sort": "searchrank",
                })
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
