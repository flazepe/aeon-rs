use crate::statics::exchange_rate::EXCHANGE_RATE_CURRENCIES;
use anyhow::{Context, Result};
use reqwest::Client;
use serde::Deserialize;

#[derive(Deserialize)]
struct ExchangeRateConversionResponse {
    result: f64,
}

pub struct ExchangeRateConversion {
    pub origin_currency: String,
    pub amount: f64,
    pub target_currency: String,
    pub conversion: f64,
}

impl ExchangeRateConversion {
    pub async fn get<T: ToString, U: ToString>(amount: f64, origin_currency: T, target_currency: U) -> Result<Self> {
        let origin_currency = EXCHANGE_RATE_CURRENCIES
            .get_key_value(origin_currency.to_string().to_uppercase().as_str())
            .context("Invalid origin currency.")?;

        let target_currency = EXCHANGE_RATE_CURRENCIES
            .get_key_value(target_currency.to_string().to_uppercase().as_str())
            .context("Invalid target currency.")?;

        Ok(Self {
            origin_currency: format!("{} ({})", origin_currency.1, origin_currency.0),
            amount: amount.clone(),
            target_currency: format!("{} ({})", target_currency.1, target_currency.0),
            conversion: (Client::new()
                .get("https://api.exchangerate.host/convert")
                .query(&[
                    ("amount", amount.to_string().as_str()),
                    ("from", origin_currency.0.to_string().as_str()),
                    ("to", target_currency.0.to_string().as_str()),
                ])
                .send()
                .await?
                .json::<ExchangeRateConversionResponse>()
                .await?)
                .result,
        })
    }

    pub fn format(&self) -> String {
        format!("{} {} = `{:.3} {}`.", self.amount, self.origin_currency, self.conversion, self.target_currency)
    }
}
