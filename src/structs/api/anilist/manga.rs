use crate::{
    macros::{and_then_or, if_else, yes_no},
    statics::{anilist::ANILIST_MANGA_FIELDS, colors::PRIMARY_COLOR},
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
use slashook::{
    chrono::{TimeZone, Utc},
    structs::embeds::Embed,
};

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

impl AniListManga {
    fn _format(&self) -> Embed {
        Embed::new()
            .set_color(PRIMARY_COLOR)
            .unwrap_or_default()
            .set_thumbnail(&self.cover_image.extra_large)
            .set_image(self.banner_image.as_ref().unwrap_or(&"".into()))
            .set_title(format!(
                ":flag_{}:  {} ({})",
                self.country_of_origin.to_lowercase(),
                self.title.romaji,
                AniList::format_enum_value(&self.format)
            ))
            .set_url(&self.site_url)
    }

    pub fn format(self) -> Embed {
        self._format()
            .set_description(
                self.synonyms
                    .iter()
                    .map(|title| format!("_{title}_"))
                    .collect::<Vec<String>>()
                    .join("\n"),
            )
            .add_field(
                "Published",
                format!(
                    "{} ({})",
                    AniList::format_airing_date(self.start_date, self.end_date),
                    AniList::format_enum_value(self.status)
                ),
                false,
            )
            .add_field(
                "Chapters",
                format!(
                    "{}",
                    and_then_or!(self.chapters, |chapters| Some(chapters.to_string()), "TBA".into()),
                ),
                true,
            )
            .add_field(
                "Volumes",
                format!(
                    "{}",
                    and_then_or!(self.volumes, |volumes| Some(volumes.to_string()), "TBA".into()),
                ),
                true,
            )
            .add_field("Licensed", yes_no!(self.is_licensed), true)
            .add_field(
                "Genre",
                self.genres
                    .iter()
                    .map(|genre| {
                        format!(
                            "[{genre}](https://anilist.co/search/anime?genres={})",
                            genre.replace(" ", "+")
                        )
                    })
                    .collect::<Vec<String>>()
                    .join(", "),
                true,
            )
            .add_field(
                "Source",
                and_then_or!(
                    self.source,
                    |source| Some(AniList::format_enum_value(source)),
                    "N/A".into()
                ),
                true,
            )
            .add_field(
                "Score",
                {
                    let mut scores = vec![];

                    if let Some(average_score) = self.average_score {
                        scores.push(format!("Average {average_score}%"))
                    }

                    if let Some(mean_score) = self.mean_score {
                        scores.push(format!("Mean {mean_score}%"))
                    }

                    if_else!(scores.is_empty(), "N/A".into(), scores.join("\n"))
                },
                true,
            )
            .set_footer("Last updated", None::<String>)
            .set_timestamp(Utc.timestamp_opt(self.updated_at as i64, 0).unwrap())
    }

    pub fn format_description(self) -> Embed {
        self._format().set_description({
            let mut description = self
                .description
                .unwrap_or("N/A".into())
                .split("\n")
                .map(|string| string.to_string())
                .collect::<Vec<String>>();

            while description.join("\n").len() > 4096 {
                description.pop();
            }

            description.join("\n")
        })
    }

    pub fn format_relations(self) -> Embed {
        AniList::format_relations(self._format(), self.relations.edges)
    }
}

impl AniList {
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
}
