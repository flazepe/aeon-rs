use crate::{
    functions::limit_strings,
    macros::yes_no,
    structs::api::anilist::{
        components::{
            AniListCoverImage, AniListEdges, AniListExternalLink, AniListFormat, AniListFuzzyDate, AniListMangaCharacter,
            AniListMediaPageResponse, AniListMediaResponse, AniListRanking, AniListRelation, AniListResponse, AniListSource, AniListStatus,
            AniListTitle,
        },
        statics::{ANILIST_EMBED_COLOR, ANILIST_MANGA_FIELDS},
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
    pub format: Option<AniListFormat>,
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
            .set_color(ANILIST_EMBED_COLOR)
            .unwrap_or_default()
            .set_thumbnail(&self.cover_image.extra_large)
            .set_image(self.banner_image.as_deref().unwrap_or(""))
            .set_title(format!(
                ":flag_{}: {} ({})",
                self.country_of_origin.to_lowercase(),
                match self.title.romaji.len() > 230 {
                    true => format!("{}…", self.title.romaji.chars().take(229).collect::<String>().trim()),
                    false => self.title.romaji.clone(),
                },
                self.format.as_ref().map_or_else(|| "TBA".into(), |format| format.to_string()),
            ))
            .set_url(&self.site_url)
    }

    pub fn format(&self) -> Embed {
        self._format()
            .set_description(self.synonyms.iter().map(|title| format!("_{title}_")).collect::<Vec<_>>().join("\n"))
            .add_field("Published", format!("{} ({})", AniList::format_airing_date(&self.start_date, &self.end_date), &self.status), false)
            .add_field("Chapters", self.chapters.map_or_else(|| "TBA".into(), |chapters| chapters.to_string()), true)
            .add_field("Volumes", self.volumes.map_or_else(|| "TBA".into(), |volumes| volumes.to_string()), true)
            .add_field("Licensed", yes_no!(self.is_licensed), true)
            .add_field(
                "Genre",
                self.genres
                    .iter()
                    .map(|genre| format!("[{genre}](https://anilist.co/search/anime?genres={})", genre.replace(' ', "+")))
                    .collect::<Vec<_>>()
                    .join(", "),
                true,
            )
            .add_field("Source", self.source.as_ref().map_or_else(|| "N/A".into(), |source| source.to_string()), true)
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
        self._format().set_description(limit_strings(
            self.characters
                .edges
                .iter()
                .map(|character| format!("[{}]({}) ({})", character.node.name.full, character.node.site_url, &character.role)),
            "\n",
            4096,
        ))
    }

    pub fn format_relations(&self) -> Embed {
        AniList::format_relations(self._format(), &self.relations.edges)
    }
}

impl AniList {
    pub async fn get_manga(id: u64) -> Result<AniListManga> {
        match AniList::query::<_, AniListResponse<AniListMediaResponse<AniListManga>>>(
            format!(
                "query($id: Int) {{
                    Media(id: $id) {{
                        {ANILIST_MANGA_FIELDS}
                    }}
                }}",
            ),
            json!({ "id": id }),
        )
        .await?
        .data
        .media
        {
            Some(manga) => Ok(manga),
            None => bail!("Manga not found."),
        }
    }

    pub async fn search_manga<T: ToString>(search: T) -> Result<Vec<AniListManga>> {
        let result: AniListResponse<AniListMediaPageResponse<AniListManga>> = AniList::query(
            format!(
                "query($search: String) {{
                    Page(perPage: 10) {{
                        media(search: $search, type: MANGA, sort: POPULARITY_DESC) {{
                            {ANILIST_MANGA_FIELDS}
                        }}
                    }}
                }}",
            ),
            json!({ "search": search.to_string() }),
        )
        .await?;

        if result.data.page.media.is_empty() {
            bail!("Manga not found.");
        }

        Ok(result.data.page.media)
    }
}
