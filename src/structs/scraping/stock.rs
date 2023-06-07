use crate::statics::{colors::PRIMARY_COLOR, REQWEST};
use anyhow::{bail, Context, Result};
use nipper::Document;
use slashook::structs::embeds::Embed;

struct YahooFinanceLookupAttributes {
    href: String,
    title: String,
    data_symbol: String,
}

pub struct Stock {
    pub name: String,
    pub url: String,
    pub currency: String,
    pub price: String,
    pub diff: String,
}

impl Stock {
    pub async fn get<T: ToString>(ticker: T) -> Result<Self> {
        let attributes = {
            let document = Document::from(
                &REQWEST.get("https://finance.yahoo.com/lookup/equity").query(&[("s", ticker.to_string())]).send().await?.text().await?,
            );

            let selection = document.select("td a");

            if selection.nodes().is_empty() {
                bail!("Ticker not found.");
            }

            YahooFinanceLookupAttributes {
                href: selection.attr("href").context("Missing href attr.")?.to_string(),
                title: selection.attr("title").context("Missing title attr.")?.to_string(),
                data_symbol: selection.attr("data-symbol").context("Missing data-symbol attr.")?.to_string(),
            }
        };

        let document = Document::from(&REQWEST.get(format!("https://finance.yahoo.com{}", attributes.href)).send().await?.text().await?);

        Ok(Self {
            name: format!("{} ({})", attributes.title, attributes.data_symbol),
            url: format!("https://finance.yahoo.com/quote/{}", attributes.data_symbol),
            currency: document
                .select("#quote-header-info span")
                .first()
                .text()
                .split(' ')
                .last()
                .context("Could not get currency.")?
                .to_string(),
            price: document.select(r#"#quote-header-info [data-field="regularMarketPrice"]"#).first().text().to_string(),
            diff: ["regularMarketChange", "regularMarketChangePercent"]
                .map(|field| document.select(&format!(r#"#quote-header-info [data-field="{}"]"#, field)).first().text().to_string())
                .join(" "),
        })
    }

    pub fn format(&self) -> Embed {
        Embed::new()
            .set_color(PRIMARY_COLOR)
            .unwrap_or_default()
            .set_title(&self.name)
            .set_url(&self.url)
            .set_description(format!("```diff\n{} {}\n{}```", self.currency, self.price, self.diff))
    }
}
