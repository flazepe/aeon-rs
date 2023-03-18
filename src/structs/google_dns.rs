use crate::{and_then_or, constants::*};
use anyhow::{bail, Result};
use reqwest::get;
use serde::Deserialize;
use slashook::structs::embeds::Embed;

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
pub struct GoogleDNSQuery {
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

pub struct GoogleDNS {
    pub domain: String,
    pub record_type: String,
    pub comment: Option<String>,
    pub records: Vec<GoogleDNSRecord>,
}

impl GoogleDNS {
    pub async fn query(record_type: &str, domain: &str) -> Result<Self> {
        let record_type = record_type.to_uppercase();
        let domain = domain
            .to_lowercase()
            .replace("http://", "")
            .replace("https://", "");

        let res = get(format!(
            "https://dns.google/resolve?type={record_type}&name={domain}"
        ))
        .await?;

        if res.status() != 200 {
            bail!("Invalid record type.");
        }

        let dns_response = res.json::<GoogleDNSQuery>().await?;

        if dns_response.status != 0 {
            bail!(and_then_or!(
                DNS_CODES
                    .iter()
                    .enumerate()
                    .find(|(index, _)| index == &(dns_response.status as usize)),
                |entry| Some(entry.1.join(": ")),
                "An unknown error occurred.".into()
            ));
        }

        let records = dns_response
            .answer
            .or(dns_response.authority)
            .unwrap_or(vec![]);

        if records.is_empty() {
            bail!("No DNS records found.")
        }

        Ok(Self {
            domain: domain.to_string(),
            record_type,
            comment: dns_response.comment,
            records,
        })
    }

    pub fn format(self) -> Embed {
        Embed::new()
            .set_title(format!("{} records for {}", self.record_type, self.domain))
            .set_description(format!(
                "{}```diff\n{}```",
                self.comment.unwrap_or("".into()),
                self.records
                    .iter()
                    .map(|record| format!("+ {} (TTL {})", record.data.trim(), record.ttl))
                    .collect::<Vec<String>>()
                    .join("\n")
            ))
    }
}
