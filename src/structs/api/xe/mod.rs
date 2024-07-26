pub mod statics;

use crate::{statics::REQWEST, structs::api::xe::statics::XE_CURRENCIES, traits::Commas};
use anyhow::{Context, Result};
use serde::Deserialize;
use std::fmt::Display;

#[derive(Deserialize, Debug)]
struct XeResponse {
    to: Vec<XeQuote>,
}

#[derive(Deserialize, Debug)]
struct XeQuote {
    mid: f64,
}

#[derive(Debug)]
pub struct Xe {
    pub origin_currency: String,
    pub amount: f64,
    pub target_currency: String,
    pub conversion: f64,
}

impl Xe {
    pub async fn convert<T: Display, U: Display>(amount: f64, origin_currency: T, target_currency: U) -> Result<Self> {
        let origin_currency =
            XE_CURRENCIES.get_key_value(origin_currency.to_string().to_uppercase().as_str()).context("Invalid origin currency.")?;

        let target_currency =
            XE_CURRENCIES.get_key_value(target_currency.to_string().to_uppercase().as_str()).context("Invalid target currency.")?;

        Ok(Self {
            origin_currency: format!("{} ({})", origin_currency.1, origin_currency.0),
            amount,
            target_currency: format!("{} ({})", target_currency.1, target_currency.0),
            conversion: (REQWEST
                .get(format!("https://duckduckgo.com/js/spice/currency_convert/{amount}/{}/{}", origin_currency.0, target_currency.0))
                .send()
                .await?
                .json::<XeResponse>()
                .await?)
                .to[0]
                .mid,
        })
    }

    pub fn format(&self) -> String {
        format!(
            "{} {} = `{} {}`.",
            format!("{:.2}", self.amount).commas(),
            self.origin_currency,
            format!("{:.2}", self.conversion).commas(),
            self.target_currency,
        )
    }
}
