use crate::{
    functions::limit_string,
    statics::anilist::{ANILIST_ANIME_FIELDS, ANILIST_EMBED_COLOR},
    structs::api::anilist::{
        components::{
            AniListAiringSchedule, AniListAnimeCharacter, AniListCoverImage, AniListEdges, AniListExternalLink, AniListFormat,
            AniListFuzzyDate, AniListMediaPageResponse, AniListMediaResponse, AniListNodes, AniListRanking, AniListRelation,
            AniListResponse, AniListSeason, AniListSource, AniListStatus, AniListStudio, AniListTitle, AniListTrailer,
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
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AniListAnime {
    pub id: u64,
    pub site_url: String,
    pub cover_image: AniListCoverImage,
    pub banner_image: Option<String>,
    pub country_of_origin: String,
    pub title: AniListTitle,
    pub format: Option<AniListFormat>,
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
            .set_color(ANILIST_EMBED_COLOR)
            .unwrap_or_default()
            .set_thumbnail(&self.cover_image.extra_large)
            .set_image(self.banner_image.as_ref().unwrap_or(&"".into()))
            .set_title(format!(
                ":flag_{}: {} ({})",
                self.country_of_origin.to_lowercase(),
                self.title.romaji,
                self.format.as_ref().map_or("TBA".into(), |format| AniList::format_enum_value(format))
            ))
            .set_url(&self.site_url)
    }

    pub fn format(&self) -> Embed {
        self._format()
            .set_description(self.synonyms.iter().map(|title| format!("_{title}_")).collect::<Vec<String>>().join("\n"))
            .add_field(
                "Aired",
                format!(
                    "{}{} ({}){}",
                    self.season.as_ref().map_or("".into(), |season| format!(
                        "Premiered {} {}{}\n",
                        AniList::format_enum_value(season),
                        self.season_year.unwrap(),
                        self.trailer.as_ref().map_or("".into(), |trailer| format!(
                            " - [Trailer]({}{})",
                            match trailer.site.as_str() {
                                "youtube" => "https://www.youtube.com/watch?v=".into(),
                                "dailymotion" => "https://www.dailymotion.com/video/".into(),
                                _ => format!("https://www.google.com/search?q={}+", trailer.site),
                            },
                            trailer.id,
                        ))
                    )),
                    AniList::format_airing_date(&self.start_date, &self.end_date),
                    AniList::format_enum_value(&self.status),
                    self.airing_schedule.nodes.iter().find(|node| node.time_until_airing.map_or(false, |time| time > 0)).map_or(
                        "".into(),
                        |node| format!(
                            "\nNext episode airs <t:{}:R>",
                            SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() + node.time_until_airing.unwrap() as u64
                        ),
                    )
                ),
                false,
            )
            .add_field(
                "Studio",
                self.studios
                    .nodes
                    .iter()
                    .map(|studio| format!("[{}]({})", studio.name, studio.site_url))
                    .collect::<Vec<String>>()
                    .join(", "),
                true,
            )
            .add_field(
                "Episodes",
                format!(
                    "{}{}",
                    self.episodes.map_or("TBA".into(), |episodes| episodes.to_string()),
                    self.duration.map_or("".into(), |duration| format!(" ({duration} minutes per episode)"))
                ),
                true,
            )
            .add_field(
                "Twitter Hashtag",
                self.hashtag.as_ref().map_or("N/A".into(), |hashtag| {
                    hashtag
                        .split(" ")
                        .map(|hashtag| format!("[{hashtag}](https://twitter.com/hashtag/{})", hashtag.chars().skip(1).collect::<String>()))
                        .collect::<Vec<String>>()
                        .join(", ")
                }),
                true,
            )
            .add_field(
                "Genre",
                self.genres
                    .iter()
                    .map(|genre| format!("[{genre}](https://anilist.co/search/anime?genres={})", genre.replace(" ", "+")))
                    .collect::<Vec<String>>()
                    .join(", "),
                true,
            )
            .add_field("Source", self.source.as_ref().map_or("N/A".into(), |source| AniList::format_enum_value(source)), true)
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

                    match scores.is_empty() {
                        true => "N/A".into(),
                        false => scores.join("\n"),
                    }
                },
                true,
            )
            .set_footer("Last updated", None::<String>)
            .set_timestamp(Utc.timestamp_opt(self.updated_at as i64, 0).unwrap())
    }

    pub fn format_description(&self) -> Embed {
        AniList::format_description(self._format(), self.description.as_ref())
    }

    pub fn format_characters(&self) -> Embed {
        self._format().set_description(limit_string(
            self.characters
                .edges
                .iter()
                .map(|character| {
                    format!(
                        "[{}]({}) ({}){}",
                        character.node.name.full,
                        character.node.site_url,
                        AniList::format_enum_value(&character.role),
                        match character.voice_actors.get(0) {
                            Some(voice_actor) => format!("\nVoiced by [{}]({})", voice_actor.name.full, voice_actor.site_url),
                            None => "".into(),
                        }
                    )
                })
                .collect::<Vec<String>>()
                .join("\n\n"),
            "\n\n",
            4096,
        ))
    }

    pub fn format_relations(&self) -> Embed {
        AniList::format_relations(self._format(), &self.relations.edges)
    }
}

impl AniList {
    pub async fn get_anime(id: u64) -> Result<AniListAnime> {
        match AniList::query::<_, AniListResponse<AniListMediaResponse<AniListAnime>>>(
            format!(
                "query($id: Int) {{
                    Media(id: $id) {{
                        {ANILIST_ANIME_FIELDS}
                    }}
                }}"
            ),
            json!({ "id": id }),
        )
        .await?
        .data
        .media
        {
            Some(anime) => Ok(anime),
            None => bail!("Anime not found."),
        }
    }

    pub async fn search_anime<T: ToString>(search: T) -> Result<Vec<AniListAnime>> {
        let result: AniListResponse<AniListMediaPageResponse<AniListAnime>> = AniList::query(
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

        if result.data.page.media.is_empty() {
            bail!("Anime not found.");
        }

        Ok(result.data.page.media)
    }
}
