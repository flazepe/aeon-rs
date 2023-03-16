use anyhow::{bail, Result};
use reqwest::get;
use serde::Deserialize;

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

impl IPInfo {
    pub async fn get(ip: &str) -> Result<Self> {
        let res = get(format!(
            "https://ipinfo.io/{}/json",
            ip.replace(['/', '?'], "")
        ))
        .await?;

        if res.status() != 200 {
            bail!("IP address not found.");
        }

        Ok(res.json().await?)
    }
}
