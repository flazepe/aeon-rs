use crate::{
    macros::if_else,
    statics::{
        colors::PRIMARY_COLOR,
        google::{GOOGLE_DNS_CODES, GOOGLE_DNS_RECORD_TYPES},
    },
    structs::api::google::Google,
};
use anyhow::{bail, Result};
use reqwest::Client;
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
#[serde(rename_all = "PascalCase")]
pub struct GoogleDNSQuery {
    pub status: u8,

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

    pub question: Vec<GoogleDNSQuestion>,
    pub answer: Option<Vec<GoogleDNSRecord>>,
    pub authority: Option<Vec<GoogleDNSRecord>>,
    pub comment: Option<String>,
}

pub struct GoogleDNS {
    pub domain: String,
    pub record_type: String,
    pub comment: Option<String>,
    pub records: Vec<GoogleDNSRecord>,
}

impl GoogleDNS {
    pub fn format(self) -> Embed {
        Embed::new()
            .set_color(PRIMARY_COLOR)
            .unwrap_or_default()
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

impl Google {
    pub async fn query_dns<T: ToString, U: ToString>(record_type: T, domain: U) -> Result<GoogleDNS> {
        let record_type = record_type.to_string();

        if !GOOGLE_DNS_RECORD_TYPES.contains(&record_type.as_str()) {
            bail!("Invalid record type.");
        }

        let domain = domain
            .to_string()
            .to_lowercase()
            .replace("http://", "")
            .replace("https://", "");

        let res = Client::new()
            .get(format!("https://dns.google/resolve"))
            .query(&[
                ("type", record_type.to_string().as_str()),
                ("name", domain.to_string().as_str()),
            ])
            .send()
            .await?;

        let dns_response = res.json::<GoogleDNSQuery>().await?;

        if dns_response.status != 0 {
            bail!(GOOGLE_DNS_CODES
                .get(&dns_response.status)
                .unwrap_or(&"An unknown error occurred."));
        }

        let records = dns_response.answer.or(dns_response.authority).unwrap_or(vec![]);

        if_else!(
            records.is_empty(),
            bail!("No DNS records found."),
            Ok(GoogleDNS {
                domain: domain.to_string(),
                record_type,
                comment: dns_response.comment,
                records,
            }),
        )
    }
}
