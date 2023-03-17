use anyhow::{bail, Result};
use reqwest::get;
use serde::Deserialize;

#[derive(Deserialize)]
struct IPInfoError {
    title: String,
    message: String,
}

#[derive(Deserialize)]
pub struct IPInfo {
    bogon: Option<bool>,
    city: Option<String>,
    country: Option<String>,
    error: Option<IPInfoError>,
    hostname: Option<String>,
    ip: String,
    loc: Option<String>,
    org: Option<String>,
    postal: Option<String>,
    region: Option<String>,
    timezone: Option<String>,
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

    pub fn format(self) -> String {
        format!(
            "[{ip}](<https://whatismyipaddress.com/ip/{ip}>)\n{}",
            [
                self.hostname.unwrap_or("".into()),
                [
                    self.city.unwrap_or("".into()),
                    self.region.unwrap_or("".into()),
                    self.country.unwrap_or("".into()),
                ]
                .into_iter()
                .filter(|entry| !entry.is_empty())
                .collect::<Vec<String>>()
                .join(", "),
                self.loc.unwrap_or("".into()).replace(',', ", "),
                self.org.unwrap_or("".into()),
            ]
            .into_iter()
            .filter(|entry| !entry.is_empty())
            .collect::<Vec<String>>()
            .join("\n"),
            ip = self.ip
        )
    }
}
