use crate::statics::REQWEST;
use anyhow::{bail, Result};
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
    pub region: Option<String>,
    pub timezone: Option<String>,
}

impl IPInfo {
    pub async fn get<T: ToString>(ip: T) -> Result<Self> {
        let response = REQWEST.get(format!("https://ipinfo.io/{}/json", ip.to_string().replace(['/', '?'], ""))).send().await?;

        if response.status() != 200 {
            bail!("IP address not found.");
        }

        Ok(response.json().await?)
    }

    pub fn format(&self) -> String {
        format!(
            "[{ip}](<https://whatismyipaddress.com/ip/{ip}>)\n{}",
            [
                self.hostname.as_ref().unwrap_or(&"".into()).clone(),
                [
                    self.city.as_ref().unwrap_or(&"".into()).clone(),
                    self.region.as_ref().unwrap_or(&"".into()).clone(),
                    self.country.as_ref().unwrap_or(&"".into()).clone(),
                ]
                .into_iter()
                .filter(|entry| !entry.is_empty())
                .collect::<Vec<String>>()
                .join(", "),
                self.loc.as_ref().unwrap_or(&"".into()).clone().replace(',', ", "),
                self.org.as_ref().unwrap_or(&"".into()).clone(),
            ]
            .into_iter()
            .filter(|entry| !entry.is_empty())
            .collect::<Vec<String>>()
            .join("\n"),
            ip = self.ip,
        )
    }
}
