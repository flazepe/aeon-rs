use serde::Deserialize;
use std::fmt::{Display, Formatter, Result as FmtResult};

// Page queries
#[derive(Deserialize)]
pub struct AniListResponse<T> {
    pub data: T,
}

#[derive(Deserialize)]
pub struct AniListMediaPageResponse<T> {
    #[serde(rename = "Page")]
    pub page: AniListMediaPage<T>,
}

#[derive(Deserialize)]
pub struct AniListMediaPage<T> {
    pub media: Vec<T>,
}

// ID queries
#[derive(Deserialize)]
pub struct AniListMediaResponse<T> {
    #[serde(rename = "Media")]
    pub media: Option<T>,
}

// Edges and nodes
#[derive(Deserialize)]
pub struct AniListEdges<T> {
    pub edges: Vec<T>,
}

#[derive(Deserialize)]
pub struct AniListNodes<T> {
    pub nodes: Vec<T>,
}

// Character
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AniListAnimeCharacter {
    pub node: AniListCharacterNode,
    pub role: AniListCharacterRole,
    pub voice_actors: Vec<AniListCharacterVoiceActor>,
}

#[derive(Deserialize)]
pub struct AniListMangaCharacter {
    pub node: AniListCharacterNode,
    pub role: AniListCharacterRole,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AniListCharacterNode {
    pub name: AniListName,
    pub image: AniListImage,
    pub site_url: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AniListCharacterRole {
    Main,
    Supporting,
    Background,
}

impl Display for AniListCharacterRole {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{self:?}")
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AniListCharacterVoiceActor {
    pub name: AniListName,
    pub language_v2: String,
    pub site_url: String,
}

// Relation
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AniListRelation {
    pub relation_type: AniListRelationType,
    pub node: AniListRelationNode,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AniListRelationNode {
    pub title: AniListTitle,
    pub format: Option<AniListFormat>,
    pub site_url: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AniListRelationType {
    Adaptation,
    Prequel,
    Sequel,
    Parent,
    SideStory,
    Character,
    Summary,
    Alternative,
    SpinOff,
    Other,
    Source,
    Compilation,
    Contains,
}

impl Display for AniListRelationType {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let original = format!("{self:?}");

        write!(
            f,
            "{}",
            match self {
                Self::SideStory => "Side Story",
                Self::SpinOff => "Spin Off",
                _ => original.as_str(),
            },
        )
    }
}

// Others
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AniListAiringSchedule {
    pub time_until_airing: Option<i64>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AniListCoverImage {
    pub extra_large: String,
}

#[derive(Deserialize)]
pub struct AniListExternalLink {
    pub site: Option<String>,
    pub url: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AniListFormat {
    Tv,
    TvShort,
    Movie,
    Special,
    Ova,
    Ona,
    Music,
    Manga,
    Novel,
    OneShot,
}

impl Display for AniListFormat {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let original = format!("{self:?}");

        write!(
            f,
            "{}",
            match self {
                Self::Tv => "TV",
                Self::TvShort => "TV Short",
                Self::Ova => "OVA",
                Self::Ona => "ONA",
                Self::OneShot => "One Shot",
                _ => original.as_str(),
            },
        )
    }
}

#[derive(Deserialize)]
pub struct AniListFuzzyDate {
    pub year: Option<u64>,
    pub month: Option<u64>,
    pub day: Option<u64>,
}

#[derive(Deserialize)]
pub struct AniListImage {
    pub large: String,
}

#[derive(Deserialize)]
pub struct AniListName {
    pub full: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AniListRanking {
    pub rank: u64,

    #[serde(rename = "type")]
    pub ranking_type: Option<AniListRankingType>,

    pub format: Option<String>,
    pub all_time: bool,
    pub season: Option<String>,
    pub year: Option<u64>,
}

#[derive(Debug, Deserialize)]
pub enum AniListRankingType {
    Popular,
    Rated,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AniListSeason {
    Winter,
    Spring,
    Summer,
    Fall,
}

impl Display for AniListSeason {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{self:?}")
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AniListSource {
    Original,
    Manga,
    LightNovel,
    VisualNovel,
    VideoGame,
    Other,
    Novel,
    Doujinshi,
    Anime,
    WebNovel,
    LiveAction,
    Game,
    Comic,
    MultimediaProject,
    PictureBook,
}

impl Display for AniListSource {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let original = format!("{self:?}");

        write!(
            f,
            "{}",
            match self {
                Self::LightNovel => "Light Novel",
                Self::VisualNovel => "Visual Novel",
                Self::VideoGame => "Video Game",
                Self::WebNovel => "Web Novel",
                Self::LiveAction => "Live Action",
                Self::MultimediaProject => "Multimedia Project",
                Self::PictureBook => "Picture Book",
                _ => original.as_str(),
            },
        )
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AniListStatus {
    Finished,
    Releasing,
    NotYetReleased,
    Cancelled,
    Hiatus,
}

impl Display for AniListStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "{}",
            match self {
                Self::NotYetReleased => "Not Yet Released".into(),
                _ => format!("{self:?}"),
            },
        )
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AniListStudio {
    pub name: String,
    pub site_url: String,
}

#[derive(Deserialize)]
pub struct AniListTitle {
    pub romaji: String,
}

#[derive(Deserialize)]
pub struct AniListTrailer {
    pub id: String,
    pub site: String,
}
