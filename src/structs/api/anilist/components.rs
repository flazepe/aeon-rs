use serde::Deserialize;

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
#[allow(non_camel_case_types)]
pub enum AniListCharacterRole {
    MAIN,
    SUPPORTING,
    BACKGROUND,
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
#[allow(non_camel_case_types)]
pub enum AniListRelationType {
    ADAPTATION,
    PREQUEL,
    SEQUEL,
    PARENT,
    SIDE_STORY,
    CHARACTER,
    SUMMARY,
    ALTERNATIVE,
    SPIN_OFF,
    OTHER,
    SOURCE,
    COMPILATION,
    CONTAINS,
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
#[allow(non_camel_case_types)]
pub enum AniListFormat {
    TV,
    TV_SHORT,
    MOVIE,
    SPECIAL,
    OVA,
    ONA,
    MUSIC,
    MANGA,
    NOVEL,
    ONE_SHOT,
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
    POPULAR,
    RATED,
}

#[derive(Debug, Deserialize)]
pub enum AniListSeason {
    WINTER,
    SPRING,
    SUMMER,
    FALL,
}

#[derive(Debug, Deserialize)]
#[allow(non_camel_case_types)]
pub enum AniListSource {
    ORIGINAL,
    MANGA,
    LIGHT_NOVEL,
    VISUAL_NOVEL,
    VIDEO_GAME,
    OTHER,
    NOVEL,
    DOUJINSHI,
    ANIME,
    WEB_NOVEL,
    LIVE_ACTION,
    GAME,
    COMIC,
    MULTIMEDIA_PROJECT,
    PICTURE_BOOK,
}

#[derive(Debug, Deserialize)]
#[allow(non_camel_case_types)]
pub enum AniListStatus {
    FINISHED,
    RELEASING,
    NOT_YET_RELEASED,
    CANCELLED,
    HIATUS,
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
