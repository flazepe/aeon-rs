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
