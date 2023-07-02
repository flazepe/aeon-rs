use crate::statics::REQWEST;
use anyhow::{bail, Result};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct IpInfoError {
    pub title: String,
    pub message: String,
}

#[derive(Deserialize)]
pub struct IpInfo {
    pub bogon: Option<bool>,
    pub city: Option<String>,
    pub country: Option<String>,
    pub error: Option<IpInfoError>,
    pub hostname: Option<String>,
    pub ip: String,
    pub loc: Option<String>,
    pub org: Option<String>,
    pub postal: Option<String>,
    pub region: Option<String>,
    pub timezone: Option<String>,
}

impl IpInfo {
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
                self.hostname.clone().unwrap_or("".into()),
                [
                    self.city.clone().unwrap_or("".into()),
                    self.region.clone().unwrap_or("".into()),
                    self.country.clone().unwrap_or("".into()),
                ]
                .into_iter()
                .filter(|entry| !entry.is_empty())
                .collect::<Vec<String>>()
                .join(", "),
                self.loc.clone().unwrap_or("".into()).replace(',', ", "),
                self.org.clone().unwrap_or("".into()),
            ]
            .into_iter()
            .filter(|entry| !entry.is_empty())
            .collect::<Vec<String>>()
            .join("\n"),
            ip = self.ip,
        )
    }
}
