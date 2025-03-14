use crate::{
    functions::{label_num, limit_strings},
    statics::colors::PRIMARY_COLOR,
    structs::api::vndb::{statics::VISUAL_NOVEL_FIELDS, Vndb},
};
use anyhow::{bail, Result};
use serde::Deserialize;
use serde_json::json;
use serde_repr::Deserialize_repr;
use slashook::structs::embeds::Embed;
use std::fmt::{Display, Formatter, Result as FmtResult};

// Enum reference: https://code.blicky.net/yorhel/vndb/src/branch/master/lib/VNDB/Types.pm

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct VndbTitle {
    pub lang: String,
    pub title: String,
    pub latin: Option<String>,
    pub official: bool,
    pub main: bool,
}

#[derive(Deserialize_repr, Debug)]
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
                Self::InDevelopment => "In development".into(),
                _ => format!("{self:?}"),
            },
        )
    }
}

#[derive(Deserialize, Debug)]
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
        let default = format!("{self:?}");

        write!(
            f,
            "{}",
            match self {
                Self::ScottishGaelic => "Scottish Gaelic",
                Self::PortugueseBrazil => "Portuguese (Brazil)",
                Self::PortuguesePortugal => "Portuguese (Portugal)",
                Self::ChineseSimplified => "Chinese Simplified",
                Self::ChineseTraditional => "Chinese Traditional",
                _ => default.as_str(),
            },
        )
    }
}

#[derive(Deserialize, Debug)]
pub enum VndbPlatform {
    #[serde(rename = "win")]
    Windows,

    #[serde(rename = "lin")]
    Linux,

    #[serde(rename = "mac")]
    Macos,

    #[serde(rename = "web")]
    Website,

    #[serde(rename = "tdo")]
    Tdo,

    #[serde(rename = "ios")]
    Ios,

    #[serde(rename = "and")]
    Android,

    #[serde(rename = "bdp")]
    BdPlayer,

    #[serde(rename = "dos")]
    Dos,

    #[serde(rename = "dvd")]
    DvdPlayer,

    #[serde(rename = "drc")]
    Dreamcast,

    #[serde(rename = "nes")]
    Famicom,

    #[serde(rename = "sfc")]
    SuperFamicom,

    #[serde(rename = "fm7")]
    Fm7,

    #[serde(rename = "fm8")]
    Fm8,

    #[serde(rename = "fmt")]
    FmTowns,

    #[serde(rename = "gba")]
    GameBoyAdvance,

    #[serde(rename = "gbc")]
    GameBoyColor,

    #[serde(rename = "msx")]
    Msx,

    #[serde(rename = "nds")]
    NintendoDs,

    #[serde(rename = "swi")]
    NintendoSwitch,

    #[serde(rename = "wii")]
    NintendoWii,

    #[serde(rename = "wiu")]
    NintendoWiiU,

    #[serde(rename = "n3d")]
    Nintendo3ds,

    #[serde(rename = "p88")]
    Pc88,

    #[serde(rename = "p98")]
    Pc98,

    #[serde(rename = "pce")]
    PcEngine,

    #[serde(rename = "pcf")]
    Pcfx,

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
    SegaMegaCd,

    #[serde(rename = "sat")]
    SegaSaturn,

    #[serde(rename = "vnd")]
    Vnds,

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
    XboxXs,

    #[serde(rename = "mob")]
    OtherMobile,

    #[serde(rename = "oth")]
    Other,
}

impl Display for VndbPlatform {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let original = format!("{self:?}");

        write!(
            f,
            "{}",
            match self {
                Self::Macos => "Mac OS",
                Self::Tdo => "3DO",
                Self::Ios => "Apple iProduct",
                Self::BdPlayer => "Blu-ray Player",
                Self::Dos => "DOS",
                Self::DvdPlayer => "DVD Player",
                Self::SuperFamicom => "Super Famicom",
                Self::Fm7 => "FM-7",
                Self::Fm8 => "FM-8",
                Self::FmTowns => "FM Towns",
                Self::GameBoyAdvance => "Game Boy Advance",
                Self::GameBoyColor => "Game Boy Color",
                Self::Msx => "MSX",
                Self::NintendoDs => "Nintendo DS",
                Self::NintendoSwitch => "Nintendo Switch",
                Self::NintendoWii => "Nintendo Wii",
                Self::NintendoWiiU => "Nintendo Wii U",
                Self::Nintendo3ds => "Nintendo 3DS",
                Self::Pc88 => "PC-88",
                Self::Pc98 => "PC-98",
                Self::PcEngine => "PC Engine",
                Self::Pcfx => "PC-FX",
                Self::PlayStationPortable => "PlayStation Portable",
                Self::PlayStation1 => "PlayStation 1",
                Self::PlayStation2 => "PlayStation 2",
                Self::PlayStation3 => "PlayStation 3",
                Self::PlayStation4 => "PlayStation 4",
                Self::PlayStation5 => "PlayStation 5",
                Self::PlayStationVita => "PlayStation Vita",
                Self::SegaMegaDrive => "Sega Mega Drive",
                Self::SegaMegaCd => "Sega Mega-CD",
                Self::SegaSaturn => "Sega Saturn",
                Self::Vnds => "VNDS",
                Self::SharpX1 => "Sharp X1",
                Self::SharpX68000 => "Sharp X68000",
                Self::Xbox360 => "Xbox 360",
                Self::XboxOne => "Xbox One",
                Self::XboxXs => "Xbox X/S",
                Self::OtherMobile => "Other (mobile)",
                _ => original.as_str(),
            },
        )
    }
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct VndbImage {
    pub id: String,
    pub url: String,
    pub dims: (u64, u64),
    pub sexual: f32,
    pub violence: f32,

    #[serde(rename = "votecount")]
    pub vote_count: u64,
}

#[derive(Deserialize_repr, Debug)]
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
                Self::Unknown => "Unknown",
                Self::VeryShort => "Very short: < 2 hours",
                Self::Short => "Short: 2 - 10 hours",
                Self::Medium => "Medium: 10 - 30 hours",
                Self::Long => "Long: 30 - 50 hours",
                Self::VeryLong => "Very long: > 50 hours",
            },
        )
    }
}

#[derive(Deserialize, Debug)]
pub enum VndbTagCategory {
    #[serde(rename = "cont")]
    Content,

    #[serde(rename = "ero")]
    SexualContext,

    #[serde(rename = "tech")]
    Technical,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
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

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
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
        let thumbnail = self.image.as_ref().map_or("", |image| if image.sexual > 1.0 { "" } else { image.url.as_str() });
        let title = format!(
            "{} ({})",
            if self.title.len() > 230 {
                format!("{}…", self.title.chars().take(229).collect::<String>().trim())
            } else {
                self.title.clone()
            },
            self.dev_status,
        );
        let url = format!("https://vndb.org/{}", self.id);

        Embed::new().set_color(PRIMARY_COLOR).unwrap_or_default().set_thumbnail(thumbnail).set_title(title).set_url(url)
    }

    pub fn format(&self) -> Embed {
        let aliases = self.aliases.iter().map(|alias| format!("_{alias}_")).collect::<Vec<String>>().join("\n");
        let popularity = format!("{:.0}%", self.popularity);
        let rating = format!(
            "{} ({})",
            self.rating.map(|rating| format!("{rating:.0}%")).as_deref().unwrap_or("N/A"),
            label_num(self.vote_count, "vote", "votes"),
        );
        let length = format!(
            "{} ({})",
            self.length.as_ref().map(|length| length.to_string()).as_deref().unwrap_or("N/A"),
            label_num(self.length_votes, "vote", "votes"),
        );
        let languages = self.languages.iter().map(|language| language.to_string()).collect::<Vec<String>>().join(", ");
        let platforms = self.platforms.iter().map(|platform| platform.to_string()).collect::<Vec<String>>().join(", ");
        let release_date = self.released.as_ref().map(|released| format!("Released {released}"));

        self._format()
            .set_description(aliases)
            .add_field("Popularity", popularity, true)
            .add_field("Rating", rating, true)
            .add_field("Length", length, true)
            .add_field("Languages", languages, false)
            .add_field("Platforms", platforms, false)
            .set_footer(release_date.as_deref().unwrap_or(""), None::<String>)
    }

    pub fn format_description(&self) -> Embed {
        let description = limit_strings(Vndb::clean_bbcode(self.description.as_deref().unwrap_or("N/A")).split('\n'), "\n", 4096);
        self._format().set_description(description)
    }

    pub fn format_tags(&self) -> Embed {
        let tags = limit_strings(
            self.tags.iter().map(|tag| {
                let mut text = format!("[{}](https://vndb.org/{})", tag.name, tag.id);

                if tag.spoiler > 1.0 {
                    text = format!("||{text}||");
                }

                text
            }),
            ", ",
            4096,
        );
        self._format().set_description(tags)
    }
}

impl Vndb {
    pub async fn search_visual_novel<T: Display>(query: T) -> Result<Vec<VndbVisualNovel>> {
        let query = query.to_string();

        let results = Self::query(
            "vn",
            if query.starts_with('v') && query.chars().skip(1).all(|char| char.is_numeric()) {
                json!({
                    "filters": ["id", "=", query],
                    "fields": VISUAL_NOVEL_FIELDS,
                })
            } else {
                json!({
                    "filters": ["search", "=", query],
                    "fields": VISUAL_NOVEL_FIELDS,
                    "sort": "searchrank",
                })
            },
        )
        .await?
        .results;

        if results.is_empty() {
            bail!("Visual novel not found.");
        }

        Ok(results)
    }
}
