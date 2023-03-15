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

#[derive(Deserialize)]
pub struct DNSRecord {
    pub name: String,

    #[serde(rename = "type")]
    pub record_type: u64,

    #[serde(rename = "TTL")]
    pub ttl: u64,

    pub data: String,
}

#[derive(Deserialize)]
pub struct DNSQuestion {
    pub name: String,

    #[serde(rename = "type")]
    pub record_type: u64,
}

#[derive(Deserialize)]
pub struct DNSResponse {
    #[serde(rename = "Status")]
    pub status: u64,

    #[serde(rename = "TC")]
    pub tc: bool,

    #[serde(rename = "RD")]
    pub rd: bool,

    #[serde(rename = "RA")]
    pub ra: bool,

    #[serde(rename = "AD")]
    pub ad: bool,

    #[serde(rename = "CD")]
    pub cd: bool,

    #[serde(rename = "Question")]
    pub question: Vec<DNSQuestion>,

    #[serde(rename = "Answer")]
    pub answer: Option<Vec<DNSRecord>>,

    #[serde(rename = "Authority")]
    pub authority: Option<Vec<DNSRecord>>,

    #[serde(rename = "Comment")]
    pub comment: Option<String>,
}
