use crate::constants::CURRENCIES;
use anyhow::{Context, Result};
use reqwest::get;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ExchangeRateConversion {
    pub result: f64,
}

pub struct FormattedExchangeRateConversion {
    pub from_currency: String,
    pub to_currency: String,
    pub converted_amount: f64,
}

impl ExchangeRateConversion {
    pub async fn get(
        amount: &f64,
        from_currency: &str,
        to_currency: &str,
    ) -> Result<FormattedExchangeRateConversion> {
        let from_currency = CURRENCIES
            .iter()
            .find(|[_, currency]| currency == &from_currency)
            .context("Invalid currency.")?;

        let to_currency = CURRENCIES
            .iter()
            .find(|[_, currency]| currency == &to_currency)
            .context("Invalid currency.")?;

        Ok(FormattedExchangeRateConversion {
            from_currency: format!("{} ({})", from_currency[1], from_currency[0]),
            to_currency: format!("{} ({})", to_currency[1], to_currency[0]),
            converted_amount: (get(format!(
                "https://api.exchangerate.host/convert?amount={amount}&from={}&to={}",
                from_currency[0], to_currency[0]
            ))
            .await?
            .json::<ExchangeRateConversion>()
            .await?)
                .result,
        })
    }
}
