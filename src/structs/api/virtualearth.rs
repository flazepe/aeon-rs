use crate::statics::{CONFIG, REQWEST};
use anyhow::{bail, Result};
use serde::Deserialize;
use std::fmt::Display;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TimeZoneConvertedTime {
    pub local_time: String,
    pub utc_offset_with_dst: String,
    pub time_zone_display_name: String,
    pub time_zone_display_abbr: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TimeZone {
    pub generic_name: String,
    pub abbreviation: Option<String>,
    pub iana_time_zone_id: String,
    pub windows_time_zone_id: String,
    pub utc_offset: String,
    pub converted_time: TimeZoneConvertedTime,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TimeZoneLocation {
    pub place_name: String,
    pub time_zone: Vec<TimeZone>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TimeZoneResource {
    #[serde(rename = "__type")]
    pub __type: String,

    pub time_zone_at_location: Vec<TimeZoneLocation>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TimeZoneResourceSet {
    pub estimated_total: u64,
    pub resources: Vec<TimeZoneResource>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TimeZoneResponse {
    pub authentication_result_code: String,
    pub brand_logo_uri: String,
    pub copyright: String,
    pub resource_sets: Vec<TimeZoneResourceSet>,
    pub status_code: u64,
    pub status_description: String,
    pub trace_id: String,
}

impl TimeZoneLocation {
    pub async fn get<T: Display>(location: T) -> Result<Self> {
        let timezones = &mut REQWEST
            .get("https://dev.virtualearth.net/REST/v1/TimeZone/")
            .query(&[("key", &CONFIG.api.virtualearth_key), ("query", &location.to_string())])
            .send()
            .await?
            .json::<TimeZoneResponse>()
            .await?
            .resource_sets[0]
            .resources[0]
            .time_zone_at_location;

        match timezones.is_empty() {
            true => bail!("Location not found."),
            false => Ok(timezones.remove(0)),
        }
    }

    pub fn format(&self) -> String {
        let timezone = &self.time_zone[0];
        let (date, time) = timezone.converted_time.local_time.split_once('T').unwrap_or(("", ""));
        let hour = time.chars().take(2).collect::<String>().parse::<u8>().unwrap_or(0);
        let min = time.chars().skip(3).take(2).collect::<String>().parse::<u8>().unwrap_or(0);

        format!(
            "It is `{}:{min} {}` or `{hour}:{min}` in {} (`{date}, UTC {}`).",
            if hour > 12 { hour - 12 } else { hour },
            if hour > 12 { "PM" } else { "AM" },
            self.place_name,
            timezone.utc_offset,
        )
    }
}
