use crate::statics::colors::PRIMARY_COLOR;
use anyhow::{bail, Context, Result};
use nipper::Document;
use reqwest::Client;
use slashook::structs::embeds::Embed;

pub struct Distro {
    pub name: String,
    pub distro_type: String,
    pub architecture: String,
    pub based_on: String,
    pub origin: String,
    pub status: String,
    pub category: String,
    pub desktop: String,
    pub popularity: String,
}

impl Distro {
    pub async fn get<T: ToString>(name: T) -> Result<Self> {
        let document = Document::from(
            &Client::new()
                .get("https://distrowatch.com/table.php")
                .query(&[("distribution", name.to_string().as_str())])
                .send()
                .await?
                .text()
                .await?,
        );

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
                .to_string())
        };

        Ok(Self {
            name: name.to_string(),
            distro_type: get_table_nth_child(1)?,
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
        Embed::new()
            .set_color(PRIMARY_COLOR)
            .unwrap_or_default()
            .add_field("Name", &self.name, true)
            .add_field("Type", &self.distro_type, true)
            .add_field("Architecture", &self.architecture, true)
            .add_field("Based on", &self.based_on, true)
            .add_field("Origin", &self.origin, true)
            .add_field("Status", &self.status, true)
            .add_field("Category", &self.category, true)
            .add_field("Desktop", &self.desktop, true)
            .add_field("Popularity", &self.popularity, true)
    }
}
