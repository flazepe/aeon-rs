use crate::{
    functions::if_else_option,
    macros::{if_else, plural},
    statics::{colors::PRIMARY_COLOR, vndb::VISUAL_NOVEL_FIELDS},
    structs::api::vndb::Vndb,
};
use anyhow::{bail, Result};
use serde::Deserialize;
use serde_json::json;
use serde_repr::Deserialize_repr;
use slashook::structs::embeds::Embed;
use std::fmt::{Display, Formatter, Result as FmtResult};

// Enum reference: https://code.blicky.net/yorhel/vndb/src/branch/master/lib/VNDB/Types.pm

#[derive(Deserialize)]
pub struct VndbTitle {
    pub lang: String,
    pub title: String,
    pub latin: Option<String>,
    pub official: bool,
    pub main: bool,
}

#[derive(Debug, Deserialize_repr)]
#[repr(u8)]
pub enum VndbDevStatus {
    Finished,
    InDevelopment,
    Cancelled,
}

impl Display for VndbDevStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "{}",
            match self {
                VndbDevStatus::InDevelopment => "In development".into(),
                _ => format!("{:?}", self),
            }
        )
    }
}

#[derive(Debug, Deserialize)]
pub enum VndbLanguage {
    #[serde(rename = "ar")]
    Arabic,

    #[serde(rename = "bg")]
    Bulgarian,

    #[serde(rename = "ca")]
    Catalan,

    #[serde(rename = "cs")]
    Czech,

    #[serde(rename = "ck")]
    Cherokee,

    #[serde(rename = "da")]
    Danish,

    #[serde(rename = "de")]
    German,

    #[serde(rename = "el")]
    Greek,

    #[serde(rename = "en")]
    English,

    #[serde(rename = "eo")]
    Esperanto,

    #[serde(rename = "es")]
    Spanish,

    #[serde(rename = "eu")]
    Basque,

    #[serde(rename = "fa")]
    Persian,

    #[serde(rename = "fi")]
    Finnish,

    #[serde(rename = "fr")]
    French,

    #[serde(rename = "ga")]
    Irish,

    #[serde(rename = "gd")]
    ScottishGaelic,

    #[serde(rename = "he")]
    Hebrew,

    #[serde(rename = "hi")]
    Hindi,

    #[serde(rename = "hr")]
    Croatian,

    #[serde(rename = "hu")]
    Hungarian,

    #[serde(rename = "id")]
    Indonesian,

    #[serde(rename = "it")]
    Italian,

    #[serde(rename = "iu")]
    Inuktitut,

    #[serde(rename = "ja")]
    Japanese,

    #[serde(rename = "ko")]
    Korean,

    #[serde(rename = "mk")]
    Macedonian,

    #[serde(rename = "ms")]
    Malay,

    #[serde(rename = "la")]
    Latin,

    #[serde(rename = "lt")]
    Lithuanian,

    #[serde(rename = "lv")]
    Latvian,

    #[serde(rename = "nl")]
    Dutch,

    #[serde(rename = "no")]
    Norwegian,

    #[serde(rename = "pl")]
    Polish,

    #[serde(rename = "pt-br")]
    PortugueseBrazil,

    #[serde(rename = "pt-pt")]
    PortuguesePortugal,

    #[serde(rename = "ro")]
    Romanian,

    #[serde(rename = "ru")]
    Russian,

    #[serde(rename = "sk")]
    Slovak,

    #[serde(rename = "sl")]
    Slovene,

    #[serde(rename = "sr")]
    Serbian,

    #[serde(rename = "sv")]
    Swedish,

    #[serde(rename = "ta")]
    Tagalog,

    #[serde(rename = "th")]
    Thai,

    #[serde(rename = "tr")]
    Turkish,

    #[serde(rename = "uk")]
    Ukrainian,

    #[serde(rename = "ur")]
    Urdu,

    #[serde(rename = "vi")]
    Vietnamese,

    #[serde(rename = "zh")]
    Chinese,

    #[serde(rename = "zh-Hans")]
    ChineseSimplified,

    #[serde(rename = "zh-Hant")]
    ChineseTraditional,
}

impl Display for VndbLanguage {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let default = format!("{:?}", self);

        write!(
            f,
            "{}",
            match self {
                VndbLanguage::ScottishGaelic => "Scottish Gaelic",
                VndbLanguage::PortugueseBrazil => "Portuguese (Brazil)",
                VndbLanguage::PortuguesePortugal => "Portuguese (Portugal)",
                VndbLanguage::ChineseSimplified => "Chinese Simplified",
                VndbLanguage::ChineseTraditional => "Chinese Traditional",
                _ => default.as_str(),
            }
        )
    }
}

#[derive(Debug, Deserialize)]
pub enum VndbPlatform {
    #[serde(rename = "win")]
    Windows,

    #[serde(rename = "lin")]
    Linux,

    #[serde(rename = "mac")]
    MacOS,

    #[serde(rename = "web")]
    Website,

    #[serde(rename = "tdo")]
    TDO,

    #[serde(rename = "ios")]
    IOS,

    #[serde(rename = "and")]
    Android,

    #[serde(rename = "bdp")]
    BDPlayer,

    #[serde(rename = "dos")]
    DOS,

    #[serde(rename = "dvd")]
    DVDPlayer,

    #[serde(rename = "drc")]
    Dreamcast,

    #[serde(rename = "nes")]
    Famicom,

    #[serde(rename = "sfc")]
    SuperFamicom,

    #[serde(rename = "fm7")]
    FM7,

    #[serde(rename = "fm8")]
    FM8,

    #[serde(rename = "fmt")]
    FMTowns,

    #[serde(rename = "gba")]
    GameBoyAdvance,

    #[serde(rename = "gbc")]
    GameBoyColor,

    #[serde(rename = "msx")]
    MSX,

    #[serde(rename = "nds")]
    NintendoDS,

    #[serde(rename = "swi")]
    NintendoSwitch,

    #[serde(rename = "wii")]
    NintendoWii,

    #[serde(rename = "wiu")]
    NintendoWiiU,

    #[serde(rename = "n3d")]
    Nintendo3DS,

    #[serde(rename = "p88")]
    PC88,

    #[serde(rename = "p98")]
    PC98,

    #[serde(rename = "pce")]
    PCEngine,

    #[serde(rename = "pcf")]
    PCFX,

    #[serde(rename = "psp")]
    PlayStationPortable,

    #[serde(rename = "ps1")]
    PlayStation1,

    #[serde(rename = "ps2")]
    PlayStation2,

    #[serde(rename = "ps3")]
    PlayStation3,

    #[serde(rename = "ps4")]
    PlayStation4,

    #[serde(rename = "ps5")]
    PlayStation5,

    #[serde(rename = "psv")]
    PlayStationVita,

    #[serde(rename = "smd")]
    SegaMegaDrive,

    #[serde(rename = "scd")]
    SegaMegaCD,

    #[serde(rename = "sat")]
    SegaSaturn,

    #[serde(rename = "vnd")]
    VNDS,

    #[serde(rename = "x1s")]
    SharpX1,

    #[serde(rename = "x68")]
    SharpX68000,

    #[serde(rename = "xb1")]
    Xbox,

    #[serde(rename = "xb3")]
    Xbox360,

    #[serde(rename = "xbo")]
    XboxOne,

    #[serde(rename = "xxs")]
    XboxXS,

    #[serde(rename = "mob")]
    OtherMobile,

    #[serde(rename = "oth")]
    Other,
}

impl Display for VndbPlatform {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let original = format!("{:?}", self);

        write!(
            f,
            "{}",
            match self {
                VndbPlatform::MacOS => "Mac OS",
                VndbPlatform::TDO => "3DO",
                VndbPlatform::IOS => "Apple iProduct",
                VndbPlatform::BDPlayer => "Blu-ray Player",
                VndbPlatform::DVDPlayer => "DVD Player",
                VndbPlatform::SuperFamicom => "Super Famicom",
                VndbPlatform::FM7 => "FM-7",
                VndbPlatform::FM8 => "FM-8",
                VndbPlatform::FMTowns => "FM Towns",
                VndbPlatform::GameBoyAdvance => "Game Boy Advance",
                VndbPlatform::GameBoyColor => "Game Boy Color",
                VndbPlatform::NintendoDS => "Nintendo DS",
                VndbPlatform::NintendoSwitch => "Nintendo Switch",
                VndbPlatform::NintendoWii => "Nintendo Wii",
                VndbPlatform::NintendoWiiU => "Nintendo Wii U",
                VndbPlatform::Nintendo3DS => "Nintendo 3DS",
                VndbPlatform::PC88 => "PC-88",
                VndbPlatform::PC98 => "PC-98",
                VndbPlatform::PCEngine => "PC Engine",
                VndbPlatform::PCFX => "PC-FX",
                VndbPlatform::PlayStationPortable => "PlayStation Portable",
                VndbPlatform::PlayStation1 => "PlayStation 1",
                VndbPlatform::PlayStation2 => "PlayStation 2",
                VndbPlatform::PlayStation3 => "PlayStation 3",
                VndbPlatform::PlayStation4 => "PlayStation 4",
                VndbPlatform::PlayStation5 => "PlayStation 5",
                VndbPlatform::PlayStationVita => "PlayStation Vita",
                VndbPlatform::SegaMegaDrive => "Sega Mega Drive",
                VndbPlatform::SegaMegaCD => "Sega Mega-CD",
                VndbPlatform::SegaSaturn => "Sega Saturn",
                VndbPlatform::SharpX1 => "Sharp X1",
                VndbPlatform::SharpX68000 => "Sharp X68000",
                VndbPlatform::Xbox360 => "Xbox 360",
                VndbPlatform::XboxOne => "Xbox One",
                VndbPlatform::XboxXS => "Xbox X/S",
                VndbPlatform::OtherMobile => "Other (mobile)",
                _ => original.as_str(),
            }
        )
    }
}

#[derive(Deserialize)]
pub struct VndbImage {
    pub id: String,
    pub url: String,
    pub dims: (u64, u64),
    pub sexual: f32,
    pub violence: f32,

    #[serde(rename = "votecount")]
    pub vote_count: u64,
}

#[derive(Deserialize_repr)]
#[repr(u8)]
pub enum VndbLength {
    Unknown,
    VeryShort,
    Short,
    Medium,
    Long,
    VeryLong,
}

impl Display for VndbLength {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "{}",
            match self {
                VndbLength::Unknown => "Unknown",
                VndbLength::VeryShort => "Very short: < 2 hours",
                VndbLength::Short => "Short: 2 - 10 hours",
                VndbLength::Medium => "Medium: 10 - 30 hours",
                VndbLength::Long => "Long: 30 - 50 hours",
                VndbLength::VeryLong => "Very long: > 50 hours",
            }
        )
    }
}

#[derive(Deserialize)]
pub enum VndbTagCategory {
    #[serde(rename = "cont")]
    Content,

    #[serde(rename = "ero")]
    SexualContext,

    #[serde(rename = "tech")]
    Technical,
}

#[derive(Deserialize)]
pub struct VndbTag {
    pub rating: f64,
    pub spoiler: f32,
    pub lie: bool,
    pub id: String,
    pub name: String,
    pub aliases: Vec<String>,
    pub description: String,
    pub category: VndbTagCategory,
    pub searchable: bool,
    pub applicable: bool,
    pub vn_count: u64,
}

#[derive(Deserialize)]
pub struct VndbVisualNovel {
    pub id: String,
    pub title: String,

    #[serde(rename = "alttitle")]
    pub alt_title: Option<String>,

    pub titles: Vec<VndbTitle>,
    pub aliases: Vec<String>,

    #[serde(rename = "olang")]
    pub original_language: String,

    #[serde(rename = "devstatus")]
    pub dev_status: VndbDevStatus,

    pub released: Option<String>,
    pub languages: Vec<VndbLanguage>,
    pub platforms: Vec<VndbPlatform>,
    pub image: Option<VndbImage>,
    pub length: Option<VndbLength>,
    pub length_minutes: Option<u64>,
    pub length_votes: u64,
    pub description: Option<String>,
    pub rating: Option<f64>,
    pub popularity: f64,

    #[serde(rename = "votecount")]
    pub vote_count: u64,

    pub tags: Vec<VndbTag>,
}

impl VndbVisualNovel {
    fn _format(&self) -> Embed {
        Embed::new()
            .set_color(PRIMARY_COLOR)
            .unwrap_or_default()
            .set_thumbnail(if_else_option(
                self.image.as_ref(),
                |image| if_else!(image.sexual > 1.0, "".into(), image.url.to_string()),
                "".into(),
            ))
            .set_title(format!(
                "{} ({})",
                self.title.chars().take(240).collect::<String>(),
                self.dev_status.to_string(),
            ))
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
            .add_field("Popularity", format!("{:.0}%", self.popularity), true)
            .add_field(
                "Rating",
                format!(
                    "{} ({})",
                    if_else_option(self.rating, |rating| format!("{:.0}%", rating), "N/A".into()),
                    plural!(self.vote_count, "vote")
                ),
                true,
            )
            .add_field(
                "Length",
                format!(
                    "{} ({})",
                    if_else_option(self.length, |length| length.to_string(), "N/A".into()),
                    plural!(self.length_votes, "vote")
                ),
                true,
            )
            .add_field(
                "Languages",
                self.languages
                    .iter()
                    .map(|language| language.to_string())
                    .collect::<Vec<String>>()
                    .join(", "),
                false,
            )
            .add_field(
                "Platforms",
                self.platforms
                    .iter()
                    .map(|platform| platform.to_string())
                    .collect::<Vec<String>>()
                    .join(", "),
                false,
            )
            .set_footer(
                if_else_option(self.released, |released| format!("Released {released}"), "".into()),
                None::<String>,
            )
    }

    pub fn format_description(self) -> Embed {
        self._format().set_description({
            let mut description = Vndb::clean_bbcode(self.description.as_ref().unwrap_or(&"N/A".into()))
                .split("\n")
                .map(|str| str.to_string())
                .collect::<Vec<String>>();

            while description.join("\n").len() > 4096 {
                description.pop();
            }

            description.join("\n")
        })
    }

    pub fn format_tags(self) -> Embed {
        self._format().set_description({
            let mut tags = self
                .tags
                .into_iter()
                .map(|tag| {
                    format!(
                        "[{}](https://vndb.org/{})",
                        if_else!(tag.spoiler > 1.0, format!("||{}||", tag.name), tag.name),
                        tag.id
                    )
                })
                .collect::<Vec<String>>();

            while tags.join(", ").len() > 4096 {
                tags.pop();
            }

            tags.join(", ")
        })
    }
}

impl Vndb {
    pub async fn search_visual_novel<T: ToString>(&self, query: T) -> Result<Vec<VndbVisualNovel>> {
        let query = query.to_string();

        let results = self
            .query(
                "vn",
                if_else!(
                    query.starts_with("v") && query.chars().skip(1).all(|char| char.is_numeric()),
                    json!({
                        "filters": ["id", "=", query],
                        "fields": VISUAL_NOVEL_FIELDS,
                    }),
                    json!({
                        "filters": ["search", "=", query],
                        "fields": VISUAL_NOVEL_FIELDS,
                        "sort": "searchrank",
                    }),
                ),
            )
            .await?
            .results;

        if_else!(results.is_empty(), bail!("Visual novel not found."), Ok(results))
    }
}
