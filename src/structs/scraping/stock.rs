use crate::statics::{colors::PRIMARY_COLOR, REQWEST};
use anyhow::{bail, Context, Result};
use nipper::Document;
use serde::Deserialize;
use slashook::structs::embeds::Embed;
use std::fmt::Display;

#[derive(Deserialize, Debug)]
struct YahooFinanceSearchResult {
    quotes: Vec<YahooFinanceQuote>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct YahooFinanceQuote {
    shortname: String,
    symbol: String,
    is_yahoo_finance: bool,
}

#[derive(Debug)]
pub struct Stock {
    pub name: String,
    pub url: String,
    pub currency: String,
    pub price: String,
    pub diff: String,
}

impl Stock {
    pub async fn get<T: Display>(ticker: T) -> Result<Self> {
        let Some(quote) = REQWEST
            .get("https://query2.finance.yahoo.com/v1/finance/search")
            .query(&[("q", ticker.to_string())])
            .header("user-agent", "yes")
            .send()
            .await?
            .json::<YahooFinanceSearchResult>()
            .await?
            .quotes
            .into_iter()
            .find(|quote| quote.is_yahoo_finance)
        else {
            bail!("Ticker not found.")
        };

        let url = format!("https://finance.yahoo.com/quote/{}/", quote.symbol);
        let document = Document::from(&REQWEST.get(&url).header("user-agent", "yes").send().await?.text().await?);
        let mut price_change = document.select(".priceChange").iter();

        Ok(Self {
            name: format!("{} ({})", quote.shortname, quote.symbol),
            url,
            currency: document.select(".exchange").text().trim().split(' ').last().context("Could not get currency.")?.to_string(),
            price: document.select(".livePrice").text().trim().to_string(),
            diff: format!(
                "{} {}",
                price_change.next().map(|node| node.text().trim().to_string()).as_deref().unwrap_or("N/A"),
                price_change.next().map(|node| node.text().trim().to_string()).as_deref().unwrap_or("(N/A)"),
            ),
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
