use crate::{
    functions::{limit_strings, now},
    structs::api::anilist::{
        components::{
            AniListAiringSchedule, AniListAnimeCharacter, AniListCoverImage, AniListEdges, AniListExternalLink, AniListFormat,
            AniListFuzzyDate, AniListMediaPageResponse, AniListMediaResponse, AniListNodes, AniListRanking, AniListRelation,
            AniListResponse, AniListSeason, AniListSource, AniListStatus, AniListStudio, AniListTitle, AniListTrailer,
        },
        statics::{ANILIST_ANIME_FIELDS, ANILIST_EMBED_COLOR},
        AniList,
    },
};
use anyhow::{bail, Context, Result};
use serde::Deserialize;
use serde_json::json;
use slashook::{
    chrono::{TimeZone, Utc},
    structs::embeds::Embed,
};
use std::fmt::Display;

#[derive(Deserialize, Debug)]
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
        let thumbnail = &self.cover_image.extra_large;
        let image = self.banner_image.as_deref().unwrap_or("");
        let title = format!(
            ":flag_{}: {} ({})",
            self.country_of_origin.to_lowercase(),
            match self.title.romaji.len() > 230 {
                true => format!("{}…", self.title.romaji.chars().take(229).collect::<String>().trim()),
                false => self.title.romaji.clone(),
            },
            self.format.as_ref().map(|format| format.to_string()).as_deref().unwrap_or("TBA"),
        );
        let url = &self.site_url;

        Embed::new()
            .set_color(ANILIST_EMBED_COLOR)
            .unwrap_or_default()
            .set_thumbnail(thumbnail)
            .set_image(image)
            .set_title(title)
            .set_url(url)
    }

    pub fn format(&self) -> Embed {
        let synonyms = self.synonyms.iter().map(|title| format!("_{title}_")).collect::<Vec<String>>().join("\n");

        let trailer = self
            .trailer
            .as_ref()
            .map(|trailer| {
                format!(
                    " - [Trailer]({}{})",
                    match trailer.site.as_str() {
                        "youtube" => "https://www.youtube.com/watch?v=".into(),
                        "dailymotion" => "https://www.dailymotion.com/video/".into(),
                        site => format!("https://www.google.com/search?q={site}+"),
                    },
                    trailer.id,
                )
            })
            .unwrap_or_else(|| "".into());
        let season = self
            .season
            .as_ref()
            .map(|season| format!("Premiered {season} {}{trailer}\n", self.season_year.unwrap()))
            .unwrap_or_else(|| "".into());
        let airing_date = AniList::format_airing_date(&self.start_date, &self.end_date);
        let status = &self.status;
        let airing_in = self
            .airing_schedule
            .nodes
            .iter()
            .find(|node| node.time_until_airing.map_or(false, |time| time > 0))
            .map(|node| format!("\nNext episode airs <t:{}:R>", now() + node.time_until_airing.unwrap() as u64,))
            .unwrap_or_else(|| "".into());
        let aired = format!("{season}{airing_date} ({status}){airing_in}");

        let studios =
            self.studios.nodes.iter().map(|studio| format!("[{}]({})", studio.name, studio.site_url)).collect::<Vec<String>>().join(", ");
        let episodes = format!(
            "{}{}",
            self.episodes.map(|episodes| episodes.to_string()).as_deref().unwrap_or("TBA"),
            self.duration.map(|duration| format!(" ({duration} minutes per episode)")).as_deref().unwrap_or(""),
        );
        let hashtags = self
            .hashtag
            .as_ref()
            .map(|hashtag| {
                hashtag
                    .split(' ')
                    .map(|hashtag| format!("[{hashtag}](https://twitter.com/hashtag/{})", hashtag.trim_start_matches('#')))
                    .collect::<Vec<String>>()
                    .join(", ")
            })
            .unwrap_or_else(|| "N/A".into());
        let genres = self
            .genres
            .iter()
            .map(|genre| format!("[{genre}](https://anilist.co/search/anime?genres={})", genre.replace(' ', "+")))
            .collect::<Vec<String>>()
            .join(", ");
        let source = self.source.as_ref().map(|source| source.to_string()).unwrap_or_else(|| "N/A".into());
        let scores = {
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
        };
        let timestamp = Utc.timestamp_opt(self.updated_at as i64, 0).unwrap();

        self._format()
            .set_description(synonyms)
            .add_field("Aired", aired, false)
            .add_field("Studio", studios, true)
            .add_field("Episodes", episodes, true)
            .add_field("Twitter Hashtag", hashtags, true)
            .add_field("Genre", genres, true)
            .add_field("Source", source, true)
            .add_field("Score", scores, true)
            .set_footer("Last updated", None::<String>)
            .set_timestamp(timestamp)
    }

    pub fn format_description(&self) -> Embed {
        AniList::format_embed_description(self._format(), self.description.as_ref())
    }

    pub fn format_characters(&self) -> Embed {
        let characters = limit_strings(
            self.characters.edges.iter().map(|character| {
                format!(
                    "[{}]({}) ({}){}",
                    character.node.name.full,
                    character.node.site_url,
                    character.role,
                    character.voice_actors.first().map_or_else(
                        || "".into(),
                        |voice_actor| format!("\nVoiced by [{}]({})", voice_actor.name.full, voice_actor.site_url)
                    ),
                )
            }),
            "\n\n",
            4096,
        );
        self._format().set_description(characters)
    }

    pub fn format_relations(&self) -> Embed {
        AniList::format_embed_relations(self._format(), &self.relations.edges)
    }
}

impl AniList {
    pub async fn get_anime(id: u64) -> Result<AniListAnime> {
        AniList::query::<_, AniListResponse<AniListMediaResponse<AniListAnime>>>(
            format!(
                "query($id: Int) {{
                    Media(id: $id) {{
                        {ANILIST_ANIME_FIELDS}
                    }}
                }}",
            ),
            json!({ "id": id }),
        )
        .await?
        .data
        .media
        .context("Anime not found.")
    }

    pub async fn search_anime<T: Display>(search: T) -> Result<Vec<AniListAnime>> {
        let result: AniListResponse<AniListMediaPageResponse<AniListAnime>> = AniList::query(
            format!(
                "query($search: String) {{
                    Page(perPage: 10) {{
                        media(search: $search, type: ANIME, sort: POPULARITY_DESC) {{
                            {ANILIST_ANIME_FIELDS}
                        }}
                    }}
                }}",
            ),
            json!({ "search": search.to_string() }),
        )
        .await?;

        match result.data.page.media.is_empty() {
            true => bail!("Anime not found."),
            false => Ok(result.data.page.media),
        }
    }
}
