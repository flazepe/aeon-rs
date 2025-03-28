use crate::{functions::limit_strings, statics::REQWEST};
use anyhow::{Context, Result, bail};
use nipper::{Document, Selection};
use slashook::structs::embeds::Embed;
use std::fmt::{Display, Formatter, Result as FmtResult};

static NOVEL_UPDATES_URL: &str = "https://www.novelupdates.com";
static NOVEL_UPDATES_EMBED_COLOR: &str = "#2c3e50";
static NOVEL_UPDATES_EMBED_AUTHOR_ICON_URL: &str = "https://i.ibb.co/CKnK0jrK/novelupdates.png";

#[derive(Debug)]
pub struct NovelUpdates {
    pub title: String,
    pub url: String,
    pub cover_image: String,
    pub year: String,
    pub novel_type: NovelUpdatesField,
    pub language: NovelUpdatesField,
    pub status: String,
    pub translated: NovelUpdatesField,
    pub genres: Vec<NovelUpdatesField>,
    pub tags: Vec<NovelUpdatesField>,
    pub authors: Vec<NovelUpdatesField>,
    pub artists: Vec<NovelUpdatesField>,
    pub licensed: String,
    pub original_publisher: NovelUpdatesField,
    pub english_publisher: NovelUpdatesField,
    pub description: String,
}

#[derive(Debug)]
pub struct NovelUpdatesField {
    pub name: String,
    pub url: String,
}

impl Display for NovelUpdatesField {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "[{}]({})", self.name, self.url)
    }
}

#[derive(Debug)]
pub struct NovelUpdatesSearchResult {
    pub title: String,
    pub id: String,
}

impl NovelUpdates {
    pub async fn search<T: Display>(query: T) -> Result<Vec<NovelUpdatesSearchResult>> {
        let text = REQWEST
            .get(format!("{NOVEL_UPDATES_URL}/series-finder/"))
            .header("user-agent", "yes")
            .query(&[("sf", "1"), ("sh", &query.to_string())])
            .send()
            .await?
            .text()
            .await?;

        let results = Document::from(&text)
            .select(".search_title")
            .iter()
            .map(|div| {
                let a_text = div.select("a").text();
                let span_id = div.select("span").attr_or("id", "");
                NovelUpdatesSearchResult { title: a_text.to_string(), id: span_id.trim_start_matches("sid").to_string() }
            })
            .collect::<Vec<NovelUpdatesSearchResult>>();

        if results.is_empty() {
            bail!("No results found.");
        }

        Ok(results)
    }

    async fn get_url_from_id<T: Display>(id: T) -> Result<String> {
        let text = REQWEST
            .get("https://www.novelupdates.com/rank-graph/")
            .query(&[("pid", &id.to_string())])
            .header("user-agent", "yes")
            .send()
            .await?
            .text()
            .await?;

        Document::from(&text)
            .select("dd a")
            .iter()
            .nth(1)
            .and_then(|a| a.attr("href").map(|href| href.to_string()))
            .context("Could not get novel URL from ID.")
    }

    pub async fn get<T: Display>(id: T) -> Result<Self> {
        let url = Self::get_url_from_id(id).await?;
        let text = REQWEST.get(&url).header("user-agent", "yes").send().await?.text().await?;
        let document = Document::from(&text);

        let get_field = |a: Selection<'_>| NovelUpdatesField {
            name: if a.text().is_empty() { "N/A".into() } else { a.text().to_string() },
            url: a
                .attr("href")
                .map(|href| if href.starts_with("//") { format!("https:{}", href) } else { href.to_string() })
                .unwrap_or(NOVEL_UPDATES_URL.into()),
        };

        let title = document.select(".seriestitlenu").text().to_string();
        let cover_image = document.select(".seriesimg img").attr("src").map(|attr| attr.to_string()).unwrap_or_default();
        let year = document.select("#edityear").text().trim().to_string();
        let novel_type = get_field(document.select("#showtype a"));
        let language = get_field(document.select("#showlang a"));
        let status = document.select("#editstatus").text().trim().to_string();
        let translated = get_field(document.select("#showtranslated a"));
        let genres = document.select("#seriesgenre a").iter().map(|a| get_field(a)).collect();
        let tags = document.select("#showtags a").iter().map(|a| get_field(a)).collect();
        let authors = document.select("#showauthors a").iter().map(|a| get_field(a)).collect();
        let artists = document.select("#showartists a").iter().map(|a| get_field(a)).collect();
        let licensed = document.select("#showlicensed").text().trim().to_string();
        let original_publisher = get_field(document.select("#showopublisher a"));
        let english_publisher = get_field(document.select("#showepublisher a"));
        let description = document
            .select("#editdescription p")
            .iter()
            .map(|selection| selection.text().to_string())
            .collect::<Vec<String>>()
            .join("\n\n");

        Ok(Self {
            title,
            url,
            cover_image,
            year,
            novel_type,
            language,
            status,
            translated,
            genres,
            tags,
            authors,
            artists,
            licensed,
            original_publisher,
            english_publisher,
            description,
        })
    }

    pub fn format(&self) -> Embed {
        Embed::new()
            .set_color(NOVEL_UPDATES_EMBED_COLOR)
            .unwrap_or_default()
            .set_thumbnail(&self.cover_image)
            .set_author("NovelUpdates", Some(NOVEL_UPDATES_URL), Some(NOVEL_UPDATES_EMBED_AUTHOR_ICON_URL))
            .set_title(format!("{} ({})", self.title.chars().take(121).collect::<String>(), self.year))
            .set_url(&self.url)
            .set_description(&self.description)
            .add_field("Type", format!("{} ({})", self.novel_type, self.language), true)
            .add_field("Status", &self.status, true)
            .add_field("Translated", self.translated.to_string(), true)
            .add_field("Licensed", &self.licensed, true)
            .add_field("Original Publisher", self.original_publisher.to_string(), true)
            .add_field("English Publisher", self.english_publisher.to_string(), true)
            .add_field("Genres", limit_strings(self.genres.iter().map(|field| field.to_string()), ", ", 1024), false)
            .add_field("Tags", format!("||{}||", limit_strings(self.tags.iter().map(|field| field.to_string()), ", ", 1020)), false)
            .add_field("Authors", limit_strings(self.authors.iter().map(|field| field.to_string()), ", ", 1024), false)
            .add_field("Artists", limit_strings(self.artists.iter().map(|field| field.to_string()), ", ", 1024), false)
    }
}
