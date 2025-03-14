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
#[allow(dead_code)]
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
            if self.title.romaji.len() > 230 {
                format!("{}…", self.title.romaji.chars().take(229).collect::<String>().trim())
            } else {
                self.title.romaji.clone()
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

        let trailer = self.trailer.as_ref().map(|trailer| {
            format!(
                " - [Trailer]({}{})",
                match trailer.site.as_str() {
                    "youtube" => "https://www.youtube.com/watch?v=".into(),
                    "dailymotion" => "https://www.dailymotion.com/video/".into(),
                    site => format!("https://www.google.com/search?q={site}+"),
                },
                trailer.id,
            )
        });
        let season = self
            .season
            .as_ref()
            .map(|season| format!("Premiered {season} {}{}\n", self.season_year.unwrap(), trailer.as_deref().unwrap_or("")));
        let airing_date = AniList::format_airing_date(&self.start_date, &self.end_date);
        let status = &self.status;
        let airing_in = self
            .airing_schedule
            .nodes
            .iter()
            .find(|node| node.time_until_airing.map_or(false, |time| time > 0))
            .map(|node| format!("\nNext episode airs <t:{}:R>", now() + node.time_until_airing.unwrap() as u64));
        let aired = format!(
            "{season}{airing_date} ({status}){airing_in}",
            season = season.as_deref().unwrap_or(""),
            airing_in = airing_in.as_deref().unwrap_or(""),
        );

        let studios =
            self.studios.nodes.iter().map(|studio| format!("[{}]({})", studio.name, studio.site_url)).collect::<Vec<String>>().join(", ");
        let episodes = format!(
            "{}{}",
            self.episodes.map(|episodes| episodes.to_string()).as_deref().unwrap_or("TBA"),
            self.duration.map(|duration| format!(" ({duration} minutes per episode)")).as_deref().unwrap_or(""),
        );
        let hashtags = self.hashtag.as_ref().map(|hashtag| {
            hashtag
                .split(' ')
                .map(|hashtag| format!("[{hashtag}](https://x.com/hashtag/{})", hashtag.trim_start_matches('#')))
                .collect::<Vec<String>>()
                .join(", ")
        });
        let genres = self
            .genres
            .iter()
            .map(|genre| format!("[{genre}](https://anilist.co/search/anime?genres={})", genre.replace(' ', "+")))
            .collect::<Vec<String>>()
            .join(", ");
        let source = self.source.as_ref().map(|source| source.to_string());
        let scores = [
            self.average_score.map(|average_score| format!("Average {average_score}%")),
            self.mean_score.map(|mean_score| format!("Mean {mean_score}%")),
        ]
        .into_iter()
        .fold(vec![], |mut acc, cur| {
            if let Some(cur) = cur {
                acc.push(cur);
            }
            acc
        });
        let score = if scores.is_empty() { "N/A".into() } else { scores.join("\n") };
        let timestamp = Utc.timestamp_opt(self.updated_at as i64, 0).unwrap();

        self._format()
            .set_description(synonyms)
            .add_field("Aired", aired, false)
            .add_field("Studio", studios, true)
            .add_field("Episodes", episodes, true)
            .add_field("X Hashtag", hashtags.as_deref().unwrap_or("N/A"), true)
            .add_field("Genre", genres, true)
            .add_field("Source", source.as_deref().unwrap_or("N/A"), true)
            .add_field("Score", score, true)
            .set_footer("Last updated", None::<String>)
            .set_timestamp(timestamp)
    }

    pub fn format_description(&self) -> Embed {
        AniList::format_embed_description(self._format(), self.description.as_ref())
    }

    pub fn format_characters(&self) -> Embed {
        let characters = limit_strings(
            self.characters.edges.iter().map(|character| {
                let voice_actor = character
                    .voice_actors
                    .first()
                    .map(|voice_actor| format!("\nVoiced by [{}]({})", voice_actor.name.full, voice_actor.site_url));

                format!(
                    "[{}]({}) ({}){}",
                    character.node.name.full,
                    character.node.site_url,
                    character.role,
                    voice_actor.as_deref().unwrap_or(""),
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
        Self::query::<_, AniListResponse<AniListMediaResponse<AniListAnime>>>(
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
        let result: AniListResponse<AniListMediaPageResponse<AniListAnime>> = Self::query(
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

        if result.data.page.media.is_empty() {
            bail!("Anime not found.");
        }

        Ok(result.data.page.media)
    }
}
