use crate::{
    statics::anilist::ANILIST_ANIME_FIELDS,
    structs::api::anilist::{
        components::{
            AniListAiringSchedule, AniListAnimeCharacter, AniListCoverImage, AniListEdges, AniListExternalLink,
            AniListFuzzyDate, AniListMediaPageResponse, AniListMediaResponse, AniListNodes, AniListRanking,
            AniListRelation, AniListSeason, AniListSource, AniListStatus, AniListStudio, AniListTitle, AniListTrailer,
        },
        AniList,
    },
};
use anyhow::Result;
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AniListAnime {
    pub id: u64,
    pub site_url: String,
    pub cover_image: AniListCoverImage,
    pub banner_image: Option<String>,
    pub country_of_origin: String,
    pub title: AniListTitle,
    pub format: String,
    pub synonyms: Vec<String>,
    pub is_adult: bool,
    pub start_date: AniListFuzzyDate,
    pub end_date: AniListFuzzyDate,
    pub status: AniListStatus,
    pub airing_schedule: AniListNodes<AniListAiringSchedule>,
    pub season: Option<AniListSeason>,
    pub season_year: Option<u64>,
    pub trailer: Option<AniListTrailer>,
    pub episodes: Option<u64>,
    pub duration: Option<u64>,
    pub hashtag: Option<String>,
    pub genres: Vec<String>,
    pub source: Option<AniListSource>,
    pub average_score: Option<u64>,
    pub mean_score: Option<u64>,
    pub external_links: Vec<AniListExternalLink>,
    pub rankings: Vec<AniListRanking>,
    pub popularity: u64,
    pub favourites: u64,
    pub description: Option<String>,
    pub studios: AniListNodes<AniListStudio>,
    pub characters: AniListEdges<AniListAnimeCharacter>,
    pub relations: AniListEdges<AniListRelation>,
    pub updated_at: u64,
}

impl AniList {
    pub async fn search_anime<T: ToString>(search: T) -> Result<AniListMediaPageResponse<AniListAnime>> {
        Ok(AniList::query(
            format!(
                "query($search: String) {{
                    Page(perPage: 10) {{
                        media(search: $search, type: ANIME, sort: POPULARITY_DESC) {{
                            {ANILIST_ANIME_FIELDS}
                        }}
                    }}
                }}"
            ),
            json!({ "search": search.to_string() }),
        )
        .await?)
    }

    pub async fn get_anime(id: u64) -> Result<AniListMediaResponse<AniListAnime>> {
        Ok(AniList::query(
            format!(
                "query($id: Int) {{
                    Media(id: $id) {{
                        {ANILIST_ANIME_FIELDS}
                    }}
                }}"
            ),
            json!({ "id": id }),
        )
        .await?)
    }
}
