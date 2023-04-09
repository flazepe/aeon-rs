use crate::{
    macros::if_else,
    statics::anilist::ANILIST_MANGA_FIELDS,
    structs::api::anilist::{
        components::{
            AniListCoverImage, AniListEdges, AniListExternalLink, AniListFormat, AniListFuzzyDate,
            AniListMangaCharacter, AniListMediaPageResponse, AniListMediaResponse, AniListRanking, AniListRelation,
            AniListSource, AniListStatus, AniListTitle,
        },
        AniList,
    },
};
use anyhow::{bail, Result};
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
    pub format: AniListFormat,
    pub synonyms: Vec<String>,
    pub is_adult: bool,
    pub start_date: AniListFuzzyDate,
    pub end_date: AniListFuzzyDate,
    pub status: AniListStatus,
    pub chapters: Option<u64>,
    pub volumes: Option<u64>,
    pub is_licensed: bool,
    pub genres: Vec<String>,
    pub source: Option<AniListSource>,
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
    pub async fn search_manga<T: ToString>(search: T) -> Result<Vec<AniListManga>> {
        let result: AniListMediaPageResponse<AniListManga> = AniList::query(
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
        .await?;

        if_else!(
            result.data.page.media.is_empty(),
            bail!("Manga not found."),
            Ok(result.data.page.media)
        )
    }

    pub async fn get_manga(id: u64) -> Result<AniListManga> {
        let result: AniListMediaResponse<AniListManga> = AniList::query(
            format!(
                "query($id: Int) {{
                    Media(id: $id) {{
                        {ANILIST_MANGA_FIELDS}
                    }}
                }}"
            ),
            json!({ "id": id }),
        )
        .await?;

        match result.data.media {
            Some(manga) => Ok(manga),
            None => bail!("Manga not found."),
        }
    }
}
