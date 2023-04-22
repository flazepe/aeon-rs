use crate::{macros::if_else, statics::colors::PRIMARY_COLOR};
use anyhow::{bail, Context, Result};
use nipper::Document;
use reqwest::{get, Client};
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
                &Client::new()
                    .get("https://finance.yahoo.com/lookup/equity")
                    .query(&[("s", ticker.to_string().as_str())])
                    .send()
                    .await?
                    .text()
                    .await?,
            );

            let selection = &document.select("td a");

            if_else!(
                selection.nodes().is_empty(),
                bail!("Ticker not found."),
                YahooFinanceLookupAttributes {
                    href: selection.attr("href").context("Missing href attr.")?.to_string(),
                    title: selection.attr("title").context("Missing title attr.")?.to_string(),
                    data_symbol: selection
                        .attr("data-symbol")
                        .context("Missing data-symbol attr.")?
                        .to_string(),
                },
            )
        };

        let document = Document::from(
            &get(format!("https://finance.yahoo.com{}", attributes.href))
                .await?
                .text()
                .await?,
        );

        Ok(Self {
            name: format!("{} ({})", attributes.title, attributes.data_symbol),
            url: format!("https://finance.yahoo.com/quote/{}", attributes.data_symbol),
            currency: document
                .select("#quote-header-info span")
                .first()
                .text()
                .split(" ")
                .last()
                .context("Could not get currency.")?
                .to_string(),
            price: document
                .select("#quote-header-info [data-field=\"regularMarketPrice\"]")
                .first()
                .text()
                .to_string(),
            diff: ["regularMarketChange", "regularMarketChangePercent"]
                .map(|field| {
                    document
                        .select(&format!("#quote-header-info [data-field=\"{}\"]", field))
                        .first()
                        .text()
                        .to_string()
                })
                .join(" "),
        })
    }

    pub fn format(self) -> Embed {
        Embed::new()
            .set_color(PRIMARY_COLOR)
            .unwrap_or_default()
            .set_title(self.name)
            .set_url(self.url)
            .set_description(format!("```diff\n{} {}\n{}```", self.currency, self.price, self.diff))
    }
}
