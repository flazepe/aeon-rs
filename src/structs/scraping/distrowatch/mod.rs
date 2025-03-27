pub mod statics;

use crate::statics::{REQWEST, colors::PRIMARY_EMBED_COLOR};
use anyhow::{Context, Result, bail};
use nipper::Document;
use slashook::structs::embeds::Embed;
use std::fmt::Display;

const BASE_DOMAIN: &str = "https://distrowatch.com";

#[derive(Debug)]
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
                .unwrap_or_default(),
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
        fn to_hyperlink<T: Display>(names: T, query_param: &str) -> String {
            names
                .to_string()
                .split(", ")
                .map(|name| format!("[{name}]({BASE_DOMAIN}/search.php?{query_param}={})", name.replace(' ', "+")))
                .collect::<Vec<String>>()
                .join(", ")
        }

        let thumbnail = self.logo.as_deref().unwrap_or("");
        let title = format!("{} ({})", self.name, self.status);
        let url = &self.url;
        let description = &self.description;
        let distribution_type = format!(
            "{os_type} ({architecture})",
            os_type = to_hyperlink(&self.distribution_type, "ostype"),
            architecture = to_hyperlink(&self.architecture, "architecture"),
        );
        let based_on = to_hyperlink(&self.based_on, "basedon");
        let origin = to_hyperlink(&self.origin, "origin");
        let desktop = to_hyperlink(&self.desktop, "desktop");
        let category = to_hyperlink(&self.category, "category");
        let popularity = format!("[{popularity}]({BASE_DOMAIN}/dwres.php?resource=popularity)", popularity = self.popularity);

        Embed::new()
            .set_color(PRIMARY_EMBED_COLOR)
            .unwrap_or_default()
            .set_thumbnail(thumbnail)
            .set_title(title)
            .set_url(url)
            .set_description(description)
            .add_field("Type", distribution_type, true)
            .add_field("Based on", based_on, true)
            .add_field("Origin", origin, true)
            .add_field("Desktop", desktop, true)
            .add_field("Category", category, true)
            .add_field("Popularity", popularity, true)
    }
}
