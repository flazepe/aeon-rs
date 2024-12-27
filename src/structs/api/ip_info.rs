use crate::statics::REQWEST;
use anyhow::{bail, Result};
use serde::Deserialize;
use std::fmt::Display;

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct IpInfoError {
    pub title: String,
    pub message: String,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
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
    pub async fn get<T: Display>(ip: T) -> Result<Self> {
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
                self.hostname.as_deref().unwrap_or(""),
                &[self.city.as_deref().unwrap_or(""), self.region.as_deref().unwrap_or(""), self.country.as_deref().unwrap_or("")]
                    .into_iter()
                    .filter(|entry| !entry.is_empty())
                    .collect::<Vec<&str>>()
                    .join(", "),
                &self.loc.as_deref().unwrap_or("").replace(',', ", "),
                self.org.as_deref().unwrap_or(""),
            ]
            .into_iter()
            .filter(|entry| !entry.is_empty())
            .collect::<Vec<&str>>()
            .join("\n"),
            ip = self.ip,
        )
    }
}
