use std::time::{SystemTime, UNIX_EPOCH};

use crate::{
    macros::{and_then_or, if_else},
    statics::{anilist::ANILIST_ANIME_FIELDS, colors::PRIMARY_COLOR},
    structs::api::anilist::{
        components::{
            AniListAiringSchedule, AniListAnimeCharacter, AniListCoverImage, AniListEdges, AniListExternalLink,
            AniListFormat, AniListFuzzyDate, AniListMediaPageResponse, AniListMediaResponse, AniListNodes,
            AniListRanking, AniListRelation, AniListSeason, AniListSource, AniListStatus, AniListStudio, AniListTitle,
            AniListTrailer,
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
pub struct AniListAnime {
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

impl AniListAnime {
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
                AniList::prettify_enum_value(&self.format)
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
                "Aired",
                format!(
                    "{} ({}){}",
                    AniList::format_airing_date(self.start_date, self.end_date),
                    AniList::prettify_enum_value(self.status),
                    and_then_or!(
                        self.airing_schedule.nodes.iter().find(|node| and_then_or!(
                            node.time_until_airing,
                            |time| Some(time > 0),
                            false
                        )),
                        |node| Some(format!(
                            "\nNext episode airs <t:{}:R>",
                            SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
                                + node.time_until_airing.unwrap() as u64
                        )),
                        "".into()
                    )
                ),
                false,
            )
            .add_field(
                "Premiered",
                format!(
                    "{}{}",
                    and_then_or!(
                        self.season,
                        |season| Some(format!(
                            "{} {}",
                            AniList::prettify_enum_value(season),
                            self.season_year.unwrap()
                        )),
                        "TBA".into()
                    ),
                    and_then_or!(
                        self.trailer,
                        |trailer| Some(format!(
                            " - [Trailer]({}{})",
                            if_else!(
                                trailer.site == "youtube",
                                "https://www.youtube.com/watch?v=",
                                "https://www.dailymotion.com/video/"
                            ),
                            trailer.id
                        )),
                        "".into()
                    )
                ),
                true,
            )
            .add_field(
                "Episodes",
                format!(
                    "{}{}",
                    and_then_or!(self.episodes, |episodes| Some(episodes.to_string()), "TBA".into()),
                    and_then_or!(
                        self.duration,
                        |duration| Some(format!(" ({duration} minutes per episode)")),
                        "".into()
                    )
                ),
                true,
            )
            .add_field(
                "Twitter Hashtag",
                and_then_or!(
                    self.hashtag,
                    |hashtag| Some(
                        hashtag
                            .split(" ")
                            .map(|hashtag| format!(
                                "[{hashtag}](https://twitter.com/hashtag/{})",
                                hashtag.chars().skip(1).collect::<String>()
                            ))
                            .collect::<Vec<String>>()
                            .join(", ")
                    ),
                    "N/A".into()
                ),
                true,
            )
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
                    |source| Some(AniList::prettify_enum_value(source)),
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
    pub async fn get_anime(id: u64) -> Result<AniListAnime> {
        let result: AniListMediaResponse<AniListAnime> = AniList::query(
            format!(
                "query($id: Int) {{
                Media(id: $id) {{
                    {ANILIST_ANIME_FIELDS}
                }}
            }}"
            ),
            json!({ "id": id }),
        )
        .await?;

        match result.data.media {
            Some(anime) => Ok(anime),
            None => bail!("Anime not found."),
        }
    }

    pub async fn search_anime<T: ToString>(search: T) -> Result<Vec<AniListAnime>> {
        let result: AniListMediaPageResponse<AniListAnime> = AniList::query(
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
        .await?;

        if_else!(
            result.data.page.media.is_empty(),
            bail!("Anime not found."),
            Ok(result.data.page.media)
        )
    }
}
