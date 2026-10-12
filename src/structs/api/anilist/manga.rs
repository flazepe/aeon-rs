use crate::{
    functions::paginate_strings,
    macros::yes_no,
    structs::{
        api::anilist::{
            AniList,
            components::{
                AniListCoverImage, AniListEdges, AniListExternalLink, AniListFormat, AniListFuzzyDate, AniListMangaCharacter,
                AniListMediaPageResponse, AniListRanking, AniListRelation, AniListResponse, AniListSource, AniListStatus, AniListTitle,
            },
            statics::{ANILIST_EMBED_AUTHOR_ICON_URL, ANILIST_EMBED_AUTHOR_URL, ANILIST_EMBED_COLOR, ANILIST_MANGA_FIELDS},
        },
        pagination::{Pagination, PaginationPages, PaginationSection},
    },
};
use anyhow::{Result, bail};
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
        let thumbnail = &self.cover_image.extra_large;
        let image = self.banner_image.as_deref().unwrap_or_default();
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
            .set_author("AniList  •  Manga", Some(ANILIST_EMBED_AUTHOR_URL), Some(ANILIST_EMBED_AUTHOR_ICON_URL))
            .set_title(title)
            .set_url(url)
    }

    pub fn format_pagination<T: Display>(&self, command: T) -> Pagination {
        let mut sections = vec![];

        let synonyms = self.synonyms.iter().map(|title| format!("_{title}_")).collect::<Vec<String>>().join("\n");
        let published = format!("{} ({})", AniList::format_airing_date(&self.start_date, &self.end_date), &self.status);
        let chapters = self.chapters.map(|chapters| chapters.to_string()).unwrap_or("TBA".into());
        let volumes = self.volumes.map(|volumes| volumes.to_string()).unwrap_or("TBA".into());
        let licensed = yes_no!(self.is_licensed);
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
        .flatten()
        .collect::<Vec<String>>();
        let score = if scores.is_empty() { "N/A".into() } else { scores.join("\n") };
        let timestamp = Utc.timestamp_opt(self.updated_at as i64, 0).unwrap();

        let overview_embed = self
            ._format()
            .set_description(synonyms)
            .add_field("Published", published, false)
            .add_field("Chapters", chapters, true)
            .add_field("Volumes", volumes, true)
            .add_field("Licensed", licensed, true)
            .add_field("Genre", genres, true)
            .add_field("Source", source.as_deref().unwrap_or("N/A"), true)
            .add_field("Score", score, true)
            .set_footer("Last updated", None::<String>)
            .set_timestamp(timestamp);
        sections.push(PaginationSection::new("Overview", PaginationPages::Embeds(vec![vec![overview_embed]])));

        let description_embed = AniList::format_embed_description(self._format(), self.description.as_ref());
        sections.push(PaginationSection::new("Description", PaginationPages::Embeds(vec![vec![description_embed]])));

        let characters = paginate_strings(
            self.characters
                .edges
                .iter()
                .map(|character| format!("[{}]({}) ({})", character.node.name.full, character.node.site_url, character.role)),
            "\n",
            4096,
        );
        let characters_embeds = characters.iter().map(|entry| vec![self._format().set_description(entry)]).collect();
        sections.push(PaginationSection::new("Characters", PaginationPages::Embeds(characters_embeds)));

        let relations_embed = AniList::format_embed_relations(self._format(), &self.relations.edges);
        sections.push(PaginationSection::new("Relations", PaginationPages::Embeds(vec![vec![relations_embed]])));

        Pagination::new(command, sections)
    }
}

impl AniList {
    pub async fn search_manga<T: Display>(search: T) -> Result<Vec<AniListManga>> {
        let result: AniListResponse<AniListMediaPageResponse<AniListManga>> = Self::query(
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
