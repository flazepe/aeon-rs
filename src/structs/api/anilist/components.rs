use serde::Deserialize;

// Page queries
#[derive(Deserialize)]
pub struct AniListMediaPageResponse<T> {
    pub data: AniListMediaPageData<T>,
}

#[derive(Deserialize)]
pub struct AniListMediaPageData<T> {
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
    pub data: AniListMediaData<T>,
}

#[derive(Deserialize)]
pub struct AniListMediaData<T> {
    #[serde(rename = "Media")]
    pub media: Option<T>,
}

// Others
#[derive(Deserialize)]
pub struct AniListNodes<T> {
    pub nodes: Vec<T>,
}

#[derive(Deserialize)]
pub struct AniListEdges<T> {
    pub edges: Vec<T>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AniListCoverImage {
    pub extra_large: String,
}

#[derive(Deserialize)]
pub struct AniListTitle {
    pub romaji: String,
    pub native: Option<String>,
    pub english: Option<String>,
}

#[derive(Deserialize)]
pub struct AniListFuzzyDate {
    pub year: Option<u64>,
    pub month: Option<u64>,
    pub day: Option<u64>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AniListAiringSchedule {
    pub time_until_airing: Option<i64>,
}

#[derive(Deserialize)]
pub struct AniListTrailer {
    pub id: String,
    pub site: String,
}

#[derive(Deserialize)]
pub struct AniListExternalLink {
    pub site: Option<String>,
    pub url: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AniListRanking {
    pub rank: u64,

    #[serde(rename = "type")]
    pub ranking_type: Option<String>,

    pub format: Option<String>,
    pub all_time: bool,
    pub season: Option<String>,
    pub year: Option<u64>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AniListStudio {
    pub name: String,
    pub site_url: String,
}

#[derive(Deserialize)]
pub struct AniListName {
    pub full: String,
}

#[derive(Deserialize)]
pub struct AniListImage {
    pub large: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AniListCharacterNode {
    pub name: AniListName,
    pub image: AniListImage,
    pub site_url: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AniListCharacterVoiceActor {
    pub name: AniListName,
    pub language_v2: String,
    pub site_url: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AniListAnimeCharacter {
    pub node: AniListCharacterNode,
    pub role: String,
    pub voice_actors: Vec<AniListCharacterVoiceActor>,
}

#[derive(Deserialize)]
pub struct AniListMangaCharacter {
    pub node: AniListCharacterNode,
    pub role: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AniListRelationNode {
    pub title: AniListTitle,
    pub format: String,
    pub site_url: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AniListRelation {
    pub relation_type: String,
    pub node: AniListRelationNode,
}
