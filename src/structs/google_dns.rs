use crate::constants::*;
use anyhow::{bail, Result};
use reqwest::get;
use serde::Deserialize;

#[derive(Deserialize)]
struct GoogleDNSRecord {
    name: String,

    #[serde(rename = "type")]
    record_type: u64,

    #[serde(rename = "TTL")]
    ttl: u64,

    data: String,
}

#[derive(Deserialize)]
struct GoogleDNSQuestion {
    name: String,

    #[serde(rename = "type")]
    record_type: u64,
}

#[derive(Deserialize)]
pub struct GoogleDNS {
    #[serde(rename = "Status")]
    status: u64,

    #[serde(rename = "TC")]
    tc: bool,

    #[serde(rename = "RD")]
    rd: bool,

    #[serde(rename = "RA")]
    ra: bool,

    #[serde(rename = "AD")]
    ad: bool,

    #[serde(rename = "CD")]
    cd: bool,

    #[serde(rename = "Question")]
    question: Vec<GoogleDNSQuestion>,

    #[serde(rename = "Answer")]
    answer: Option<Vec<GoogleDNSRecord>>,

    #[serde(rename = "Authority")]
    authority: Option<Vec<GoogleDNSRecord>>,

    #[serde(rename = "Comment")]
    comment: Option<String>,
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
