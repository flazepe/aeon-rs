use crate::{
    statics::anilist::ANILIST_MANGA_FIELDS,
    structs::api::anilist::{
        components::{
            AniListCoverImage, AniListEdges, AniListExternalLink, AniListFuzzyDate, AniListMangaCharacter,
            AniListMediaPageResponse, AniListMediaResponse, AniListRanking, AniListRelation, AniListTitle,
        },
        AniList,
    },
};
use anyhow::Result;
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AniListManga {
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
    pub status: String,
    pub chapters: Option<u64>,
    pub volumes: Option<u64>,
    pub is_licensed: bool,
    pub genres: Vec<String>,
    pub source: Option<String>,
    pub average_score: Option<u64>,
    pub mean_score: Option<u64>,
    pub external_links: Vec<AniListExternalLink>,
    pub rankings: Vec<AniListRanking>,
    pub popularity: u64,
    pub favourites: u64,
    pub description: Option<String>,
    pub characters: AniListEdges<AniListMangaCharacter>,
    pub relations: AniListEdges<AniListRelation>,
    pub updated_at: u64,
}

impl AniList {
    pub async fn search_manga<T: ToString>(search: T) -> Result<AniListMediaPageResponse<AniListManga>> {
        Ok(AniList::query(
            format!(
                "query($search: String) {{
                    Page(perPage: 10) {{
                        media(search: $search, type: MANGA, sort: POPULARITY_DESC) {{
                            {ANILIST_MANGA_FIELDS}
                        }}
                    }}
                }}"
            ),
            json!({ "search": search.to_string() }),
        )
        .await?)
    }

    pub async fn get_manga(id: u64) -> Result<AniListMediaResponse<AniListManga>> {
        Ok(AniList::query(
            format!(
                "query($id: Int) {{
                    Media(id: $id) {{
                        {ANILIST_MANGA_FIELDS}
                    }}
                }}"
            ),
            json!({ "id": id }),
        )
        .await?)
    }
}
