use crate::constants::*;
use anyhow::{bail, Result};
use reqwest::get;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct GoogleDNSRecord {
    pub name: String,

    #[serde(rename = "type")]
    pub record_type: u64,

    #[serde(rename = "TTL")]
    pub ttl: u64,

    pub data: String,
}

#[derive(Deserialize)]
pub struct GoogleDNSQuestion {
    pub name: String,

    #[serde(rename = "type")]
    pub record_type: u64,
}

#[derive(Deserialize)]
pub struct GoogleDNS {
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
    pub question: Vec<GoogleDNSQuestion>,

    #[serde(rename = "Answer")]
    pub answer: Option<Vec<GoogleDNSRecord>>,

    #[serde(rename = "Authority")]
    pub authority: Option<Vec<GoogleDNSRecord>>,

    #[serde(rename = "Comment")]
    pub comment: Option<String>,
}

impl GoogleDNS {
    pub async fn query(record_type: &str, domain: &str) -> Result<Self> {
        let res = get(format!(
            "https://dns.google/resolve?type={}&name={}",
            record_type,
            domain
                .to_lowercase()
                .replace("http://", "")
                .replace("https://", "")
        ))
        .await?;

        if res.status() != 200 {
            bail!("Invalid record type.");
        }

        let dns_response = res.json::<Self>().await?;

        if dns_response.status != 0 {
            bail!(DNS_CODES
                .iter()
                .enumerate()
                .find(|(index, _)| index == &(dns_response.status as usize))
                .and_then(|entry| Some(entry.1.join(": ")))
                .unwrap_or("An unknown error occurred.".into()));
        }

        Ok(dns_response)
    }

    pub fn format(self) -> String {
        let records = self.answer.or(self.authority).unwrap_or(vec![]);

        if records.is_empty() {
            String::from("No DNS records found.")
        } else {
            format!(
                "{}```diff\n{}```",
                self.comment.unwrap_or("".into()),
                records
                    .iter()
                    .map(|record| format!("+ {} (TTL {})", record.data.trim(), record.ttl))
                    .collect::<Vec<String>>()
                    .join("\n")
            )
        }
    }
}
