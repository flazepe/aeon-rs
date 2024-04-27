use crate::statics::colors::PRIMARY_COLOR;
use anyhow::{bail, Result};
use chromiumoxide::{
    browser::{Browser, BrowserConfig},
    handler::viewport::Viewport,
    page::ScreenshotParams,
};
use futures::StreamExt;
use nipper::Document;
use reqwest::Client;
use slashook::{
    commands::MessageResponse,
    structs::{embeds::Embed, utils::File},
};
use std::fmt::Display;
use tokio::spawn;

pub struct Ufret;

impl Ufret {
    pub async fn search<T: Display>(query: T) -> Result<Vec<UfretSong>> {
        let document = Document::from(
            &Client::new().get("https://www.ufret.jp/search.php").query(&[("key", &query.to_string())]).send().await?.text().await?,
        );

        let results = document
            .select(".col-lg-8 .list-group-item.list-group-item-action")
            .iter()
            .map(|node| {
                UfretSong::from(format!(
                    "{}|{}",
                    node.attr("href").unwrap_or("".into()).chars().filter(|char| char.is_numeric()).collect::<String>(),
                    node.text(),
                ))
            })
            .collect::<Vec<UfretSong>>();

        if results.is_empty() {
            bail!("No results found.");
        }

        Ok(results)
    }
}

pub struct UfretSong {
    pub id: String,
    pub name: String,
    pub url: String,
    pub screenshot: Option<Vec<u8>>,
}

impl UfretSong {
    pub fn from<T: Display>(string: T) -> Self {
        let string = string.to_string();
        let (id, name) = string.split_once('|').unwrap_or(("", ""));
        Self { id: id.to_string(), name: name.to_string(), url: format!("https://www.ufret.jp/song.php?data={id}"), screenshot: None }
    }

    pub async fn screenshot(mut self) -> Result<Self> {
        let (mut browser, mut handler) = Browser::launch(
            BrowserConfig::builder()
                .no_sandbox()
                .viewport(Viewport {
                    width: 414,
                    height: 816,
                    device_scale_factor: None,
                    emulating_mobile: true,
                    is_landscape: false,
                    has_touch: false,
                })
                .build()
                .unwrap(),
        )
        .await?;

        let handle = spawn(async move {
            while let Some(handle) = handler.next().await {
                if handle.is_err() {
                    break;
                }
            }
        });

        let page = browser.new_page(format!("https://www.ufret.jp/song.php?data={}", self.id)).await?;

        page.evaluate(
            r#"
                [...document.querySelectorAll("button")].forEach(x => x.remove()); // Remove buttons
                [...document.querySelectorAll('div[spottype="dynamic_mc"]')].forEach(x => x.remove()); // Remove ads
            "#,
        )
        .await?;

        page.set_content(format!(
            r#"
                <head>
                    <style>
                        .m-5 {{ margin: 3rem; }}

                        .row {{
                            display: flex;
                            flex-wrap: wrap;
                            margin-right: -15px;
                            margin-left: -15px;
                        }}

                        {}
                    </style>
                </head>
                {}
            "#,
            // Add CSS
            page.evaluate(r#"[...document.querySelectorAll("style")][3].innerText"#)
                .await?
                .into_value::<String>()
                .unwrap(), 
            // Add chords
            page.evaluate(
                r##"
                    const h1 = document.createElement("h1");
                    h1.style = "text-align: center";
                    h1.innerText = `${document.querySelector(".show_name").innerText} - ${document.querySelector(".show_artist").innerText}`;

                    const chordData = document.querySelector("#my-chord-data");
                    chordData.prepend(h1);
                    chordData.outerHTML.replace("mt-3", "m-5");
                "##,
            )
            .await?
            .into_value::<String>()
            .unwrap(),
        ))
        .await?;

        self.screenshot = Some(page.screenshot(ScreenshotParams::builder().full_page(true).build()).await?);

        browser.close().await?;
        handle.await?;

        Ok(self)
    }

    pub fn format(&self) -> MessageResponse {
        let mut response = MessageResponse::from(
            Embed::new()
                .set_color(PRIMARY_COLOR)
                .unwrap_or_default()
                .set_title(&self.name)
                .set_url(&self.url)
                .set_image("attachment://image.png"),
        );

        if let Some(screenshot) = self.screenshot.as_ref() {
            response = response.add_file(File::new("image.png", screenshot.clone()));
        }

        response
    }
}
