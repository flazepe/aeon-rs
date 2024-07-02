pub mod statics;

use crate::statics::{colors::PRIMARY_COLOR, REQWEST};
use anyhow::{bail, Context, Result};
use nipper::Document;
use slashook::structs::embeds::Embed;
use std::fmt::Display;

const BASE_DOMAIN: &str = "https://distrowatch.com";

pub struct Distribution {
    pub name: String,
    pub url: String,
    pub logo: Option<String>,
    pub description: String,
    pub distribution_type: String,
    pub architecture: String,
    pub based_on: String,
    pub origin: String,
    pub status: String,
    pub category: String,
    pub desktop: String,
    pub popularity: String,
}

impl Distribution {
    pub async fn get<T: Display>(name: T) -> Result<Self> {
        let res = REQWEST
            .get(format!("{BASE_DOMAIN}/table.php"))
            .query(&[("distribution", name.to_string())])
            .header("user-agent", "yes")
            .send()
            .await?;
        let url = res.url().to_string();
        let document = Document::from(&res.text().await?);
        let name = document.select("td.TablesTitle h1").text();

        if name.is_empty() {
            bail!("Distribution not found.");
        }

        let get_table_nth_child = |n: u8| -> Result<String> {
            Ok(document
                .select(&format!("td.TablesTitle li:nth-child({n})"))
                .text()
                .split(':')
                .last()
                .context(format!("Could not get table nth child value for {n}."))?
                .trim()
                .to_string())
        };

        Ok(Self {
            name: name.to_string(),
            url,
            logo: document.select("td.TablesTitle img").attr("src").map(|src| format!("{BASE_DOMAIN}/{src}")),
            description: document
                .select(".TablesTitle")
                .text()
                .split('\n')
                .map(|line| line.trim().to_string())
                .filter(|line| !line.is_empty())
                .nth(3)
                .unwrap_or_else(|| "".into()),
            distribution_type: get_table_nth_child(1)?,
            architecture: get_table_nth_child(4)?,
            based_on: get_table_nth_child(2)?,
            origin: get_table_nth_child(3)?,
            status: get_table_nth_child(7)?,
            category: get_table_nth_child(6)?,
            desktop: get_table_nth_child(5)?,
            popularity: get_table_nth_child(8)?,
        })
    }

    pub fn format(&self) -> Embed {
        fn to_urls<T: Display>(names: T, url_type: &str) -> String {
            names
                .to_string()
                .split(", ")
                .map(|name| format!("[{name}]({BASE_DOMAIN}/search.php?{url_type}={})", name.replace(' ', "+")))
                .collect::<Vec<String>>()
                .join(", ")
        }

        Embed::new()
            .set_color(PRIMARY_COLOR)
            .unwrap_or_default()
            .set_thumbnail(self.logo.as_deref().unwrap_or(""))
            .set_title(format!("{} ({})", self.name, self.status))
            .set_url(&self.url)
            .set_description(&self.description)
            .add_field(
                "Type",
                format!("{} ({})", to_urls(&self.distribution_type, "ostype"), to_urls(&self.architecture, "architecture")),
                true,
            )
            .add_field("Based on", to_urls(&self.based_on, "basedon"), true)
            .add_field("Origin", to_urls(&self.origin, "origin"), true)
            .add_field("Desktop", to_urls(&self.desktop, "desktop"), true)
            .add_field("Category", to_urls(&self.category, "category"), true)
            .add_field("Popularity", format!("[{}]({BASE_DOMAIN}/dwres.php?resource=popularity)", self.popularity), true)
    }
}
