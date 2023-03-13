use serde::Deserialize;

#[derive(Deserialize)]
pub struct ExchangeRateConversion {
    pub result: f64,
}

#[derive(Deserialize)]
pub struct GoogleTranslateSentences {
    pub trans: String,
    pub orig: String,
}

#[derive(Deserialize)]
pub struct GoogleTranslateResponse {
    pub sentences: Vec<GoogleTranslateSentences>,
    pub src: String,
}

#[derive(Deserialize)]
pub struct IPInfoError {
    pub title: String,
    pub message: String,
}

#[derive(Deserialize)]
pub struct IPInfo {
    pub bogon: Option<bool>,
    pub city: Option<String>,
    pub country: Option<String>,
    pub error: Option<IPInfoError>,
    pub hostname: Option<String>,
    pub ip: String,
    pub loc: Option<String>,
    pub org: Option<String>,
    pub postal: Option<String>,
    pub readme: Option<String>,
    pub region: Option<String>,
    pub timezone: Option<String>,
}
