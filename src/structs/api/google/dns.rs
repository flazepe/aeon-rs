use crate::{
    statics::REQWEST,
    structs::api::google::{
        Google,
        statics::{GOOGLE_DNS_CODES, GOOGLE_DNS_RECORD_TYPES, GOOGLE_EMBED_AUTHOR_ICON_URL, GOOGLE_EMBED_AUTHOR_URL, GOOGLE_EMBED_COLOR},
    },
};
use anyhow::{Result, bail};
use serde::Deserialize;
use slashook::structs::embeds::Embed;
use std::fmt::Display;

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct GoogleDnsRecord {
    pub name: String,

    #[serde(rename = "type")]
    pub record_type: u64,

    #[serde(rename = "TTL")]
    pub ttl: u64,

    pub data: String,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct GoogleDnsQuestion {
    pub name: String,

    #[serde(rename = "type")]
    pub record_type: u64,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
#[allow(dead_code)]
pub struct GoogleDnsQuery {
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

    pub question: Vec<GoogleDnsQuestion>,
    pub answer: Option<Vec<GoogleDnsRecord>>,
    pub authority: Option<Vec<GoogleDnsRecord>>,
    pub comment: Option<String>,
}

#[derive(Debug)]
pub struct GoogleDns {
    pub domain: String,
    pub record_type: String,
    pub comment: Option<String>,
    pub records: Vec<GoogleDnsRecord>,
}

impl GoogleDns {
    pub fn format(&self) -> Embed {
        let title = format!("{} records for {}", self.record_type, self.domain);
        let records = format!(
            "{}```diff\n{}```",
            self.comment.as_deref().unwrap_or_default(),
            self.records.iter().map(|record| format!("+ {} (TTL {})", record.data.trim(), record.ttl)).collect::<Vec<String>>().join("\n"),
        );

        Embed::new()
            .set_color(GOOGLE_EMBED_COLOR)
            .unwrap_or_default()
            .set_author("Google  •  DNS", Some(GOOGLE_EMBED_AUTHOR_URL), Some(GOOGLE_EMBED_AUTHOR_ICON_URL))
            .set_title(title)
            .set_description(records)
    }
}

impl Google {
    pub async fn query_dns<T: Display, U: Display>(record_type: T, domain: U) -> Result<GoogleDns> {
        let record_type = record_type.to_string();

        if !GOOGLE_DNS_RECORD_TYPES.contains(&record_type.as_str()) {
            bail!("Invalid record type.");
        }

        let domain = domain.to_string().to_lowercase().replace("http://", "").replace("https://", "");

        let dns_response = REQWEST
            .get("https://dns.google/resolve")
            .query(&[("type", record_type.to_string()), ("name", domain.to_string())])
            .send()
            .await?
            .json::<GoogleDnsQuery>()
            .await?;

        if dns_response.status != 0 {
            bail!(GOOGLE_DNS_CODES.get(&dns_response.status).unwrap_or(&"An unknown error occurred."));
        }

        let records = dns_response.answer.or(dns_response.authority).unwrap_or_else(Vec::new);

        if records.is_empty() {
            bail!("No DNS records found.");
        }

        Ok(GoogleDns { domain: domain.to_string(), record_type, comment: dns_response.comment, records })
    }
}
