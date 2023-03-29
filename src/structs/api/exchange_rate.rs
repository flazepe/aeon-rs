use crate::statics::currencies::CURRENCIES;
use anyhow::{Context, Result};
use reqwest::get;
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
        let origin_currency = CURRENCIES
            .get_key_value(origin_currency.to_string().as_str())
            .context("Invalid origin currency.")?;

        let target_currency = CURRENCIES
            .get_key_value(target_currency.to_string().as_str())
            .context("Invalid target currency.")?;

        Ok(Self {
            origin_currency: format!("{} ({})", origin_currency.1, origin_currency.0),
            amount: amount.clone(),
            target_currency: format!("{} ({})", target_currency.1, target_currency.0),
            conversion: (get(format!(
                "https://api.exchangerate.host/convert?amount={amount}&from={}&to={}",
                origin_currency.0, target_currency.0
            ))
            .await?
            .json::<ExchangeRateConversionResponse>()
            .await?)
                .result,
        })
    }

    pub fn format(self) -> String {
        format!(
            "`{} {}` = `{:.3} {}`.",
            self.amount, self.origin_currency, self.conversion, self.target_currency
        )
    }
}
