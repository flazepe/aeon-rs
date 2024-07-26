use crate::{functions::limit_strings, statics::colors::PRIMARY_COLOR};
use anyhow::{bail, Result};
use chromiumoxide::browser::{Browser, BrowserConfig};
use futures::StreamExt;
use nipper::Document;
use slashook::structs::embeds::Embed;
use std::fmt::Display;
use tokio::spawn;

#[derive(Debug)]
pub struct AZLyrics {
    pub title: String,
    pub artist: String,
    pub lyrics: String,
}

impl AZLyrics {
    pub async fn get<T: Display>(query: T) -> Result<Self> {
        let (mut browser, mut handler) = Browser::launch(BrowserConfig::builder().no_sandbox().build().unwrap()).await?;

        let handle = spawn(async move {
            while let Some(handle) = handler.next().await {
                if handle.is_err() {
                    break;
                }
            }
        });

        let x = {
            let page = browser.new_page("https://www.azlyrics.com").await?;
            let content = page.content().await?;
            page.close().await?;
            content.chars().skip(content.find(r#"name="x" value=""#).unwrap_or(0) + 16).take(64).collect::<String>()
        };

        let search_content = {
            let page = browser.new_page(format!("https://search.azlyrics.com/search.php?x={x}&q={query}")).await?;
            let content = page.content().await?;
            page.close().await?;
            content
        };

        let search_url = {
            let document = Document::from(&search_content);
            let selection = document.select("td a").first();
            let Some(url) = selection.attr("href") else { bail!("Song not found.") };
            url.to_string()
        };

        let content = {
            let page = browser.new_page(search_url).await?;
            let content = page.content().await?;
            browser.close().await?;
            handle.await?;
            content
        };

        let document = Document::from(&content);
        let title = document.select("h1").first().text().chars().skip(1).collect::<String>().trim_end_matches(r#"" lyrics"#).to_string();
        let artist = document.select("h2").first().text().trim_end_matches(" Lyrics").to_string();

        for selector in ["b", ".div-share", ".lyricsh", ".noprint", ".ringtone", ".smt"] {
            for mut node in document.select(selector).iter() {
                node.remove();
            }
        }

        Ok(Self { title, artist, lyrics: document.select(".row").text().trim().to_string() })
    }

    pub fn format(&self) -> Embed {
        Embed::new()
            .set_color(PRIMARY_COLOR)
            .unwrap_or_default()
            .set_author(&self.artist, None::<String>, None::<String>)
            .set_title(&self.title)
            .set_description(limit_strings(self.lyrics.split('\n'), "\n", 4096))
    }
}
