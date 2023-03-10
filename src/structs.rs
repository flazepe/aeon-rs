use serde::Deserialize;

#[derive(Deserialize)]
pub struct ExchangeRateConversion {
    pub result: f64,
}
