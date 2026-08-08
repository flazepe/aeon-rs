use crate::{
    functions::limit_strings,
    structs::{
        api::vndb::{
            Vndb,
            statics::{VNDB_CHARACTER_FIELDS, VNDB_EMBED_COLOR},
            visual_novel::VndbImage,
        },
        components_v2::ComponentsV2Embed,
    },
    traits::Commas,
};
use anyhow::{Result, bail};
use serde::Deserialize;
use serde_json::json;
use serde_repr::Deserialize_repr;
use slashook::structs::components::{Components, Separator, TextDisplay};
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
    fn _format(&self) -> ComponentsV2Embed {
        let thumbnail = self.image.as_ref().map_or("", |image| if image.sexual > 1.0 { "" } else { image.url.as_str() });
        let title = self.name.chars().take(256).collect::<String>();
        let url = format!("https://vndb.org/{}", self.id);
        let aliases = self.aliases.iter().map(|alias| format!("_{alias}_")).collect::<Vec<String>>();

        let mut embed = ComponentsV2Embed::new().set_color(VNDB_EMBED_COLOR).set_thumbnail(thumbnail).set_title(title).set_url(url);

        if !aliases.is_empty() {
            embed = embed.set_description(aliases.join("\n"));
        }

        embed
    }

    pub fn format(&self) -> ComponentsV2Embed {
        let mut text_displays = vec![];

        let mut group = vec![];
        if let Some((sex, spoiler_sex)) = &self.sex {
            group.push(format!(
                "**Sex**: {}{}",
                sex.as_ref().map(|sex| format!("{sex:?}")).as_deref().unwrap_or("N/A"),
                spoiler_sex.as_ref().map(|spoiler_sex| format!(" (||actually {spoiler_sex:?}||)")).as_deref().unwrap_or_default(),
            ));
        }
        if let Some(blood_type) = &self.blood_type {
            group.push(format!("**Blood Type**: {blood_type:?}"));
        }
        if !group.is_empty() {
            text_displays.push(TextDisplay::new(group.join("\n")));
        }

        let mut group = vec![];
        if let Some(age) = self.age {
            group.push(format!("**Age**: {}", age.commas()));
        }
        if let Some(birthday) = self.birthday {
            group.push(format!("**Birthday**: {}/{}", birthday.0, birthday.1));
        }
        if !group.is_empty() {
            text_displays.push(TextDisplay::new(group.join("\n")));
        }

        let mut group = vec![];
        if let Some(height) = self.height {
            group.push(format!("**Height**: {} cm", height.commas()));
        }
        if let Some(weight) = self.weight {
            group.push(format!("**Weight**: {} kg", weight.commas()));
        }
        if !group.is_empty() {
            text_displays.push(TextDisplay::new(group.join("\n")));
        }

        let mut group = vec![];
        if let Some(bust) = &self.bust {
            group.push(format!("**Bust**: {bust} cm"));
        }
        if let Some(waist) = self.waist {
            group.push(format!("**Waist**: {waist} cm"));
        }
        if let Some(hips) = self.hips {
            group.push(format!("**Hips**: {hips} cm"));
        }
        if let Some(cup_size) = &self.cup {
            group.push(format!("**Cup Size**: {cup_size}"));
        }
        if !group.is_empty() {
            text_displays.push(TextDisplay::new(group.join("\n")));
        }

        let total = text_displays.len();
        let mut components = Components::empty();

        for (i, text_display) in text_displays.into_iter().enumerate() {
            components = components.add_component(text_display);

            if i != total - 1 {
                let separator = Separator::new();
                components = components.add_component(separator);
            }
        }

        self._format().set_components(components)
    }

    pub fn format_traits(&self) -> ComponentsV2Embed {
        let mut groups = HashMap::new();

        for character_trait in &self.traits {
            if !groups.contains_key(&character_trait.group_name) {
                groups.insert(&character_trait.group_name, vec![]);
            }

            let mut text = format!("[{}](https://vndb.org/{})", character_trait.name, character_trait.id);

            if !matches!(character_trait.spoiler, VndbSpoilerLevel::NonSpoiler) {
                text = format!("||{text}||");
            }

            if let Some(traits) = groups.get_mut(&character_trait.group_name) {
                traits.push(text);
            }
        }

        let mut components = Components::empty();

        for (i, (group_name, traits)) in groups.iter().enumerate() {
            let text_display = TextDisplay::new(format!("### {group_name}\n{}", limit_strings(traits, ", ", 512)));
            components = components.add_component(text_display);

            if i != groups.len() - 1 {
                let separator = Separator::new();
                components = components.add_component(separator);
            }
        }

        self._format().set_components(components)
    }

    pub fn format_visual_novels(&self) -> ComponentsV2Embed {
        let mut components = Components::empty();

        let iter = self.vns.iter().take(20);
        let total = iter.len();

        for (i, vn) in iter.enumerate() {
            let text_display = TextDisplay::new(format!("### [{}](https://vndb.org/{})\n-# {}", vn.title, vn.id, vn.role));
            components = components.add_component(text_display);

            if i != total - 1 {
                let separator = Separator::new();
                components = components.add_component(separator);
            }
        }

        self._format().set_components(components)
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
