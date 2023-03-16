use crate::constants::CURRENCIES;
use anyhow::{Context, Result};
use reqwest::get;
use serde::Deserialize;

#[derive(Deserialize)]
struct ExchangeRateConversionResponse {
    result: f64,
}

pub struct ExchangeRateConversion {
    from_currency: String,
    from_amount: f64,
    to_currency: String,
    to_amount: f64,
}

impl ExchangeRateConversion {
    pub async fn get(from_amount: &f64, from_currency: &str, to_currency: &str) -> Result<Self> {
        let from_currency = CURRENCIES
            .iter()
            .find(|[currency, _]| currency == &from_currency)
            .context("Invalid currency.")?;

        let to_currency = CURRENCIES
            .iter()
            .find(|[currency, _]| currency == &to_currency)
            .context("Invalid currency.")?;

        Ok(Self {
            from_currency: format!("{} ({})", from_currency[1], from_currency[0]),
            from_amount: from_amount.clone(),
            to_currency: format!("{} ({})", to_currency[1], to_currency[0]),
            to_amount: (get(format!(
                "https://api.exchangerate.host/convert?amount={from_amount}&from={}&to={}",
                from_currency[0], to_currency[0]
            ))
            .await?
            .json::<ExchangeRateConversionResponse>()
            .await?)
                .result,
        })
    }

    pub fn format(self) -> String {
        format!(
            "{} {} equals {:.3} {}.",
            self.from_amount, self.from_currency, self.to_amount, self.to_currency
        )
    }
}
