use crate::statics::{CONFIG, REQWEST};
use anyhow::{bail, Result};
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimeZoneConvertedTime {
    pub local_time: String,
    pub utc_offset_with_dst: String,
    pub time_zone_display_name: String,
    pub time_zone_display_abbr: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimeZone {
    pub generic_name: String,
    pub abbreviation: Option<String>,
    pub iana_time_zone_id: String,
    pub windows_time_zone_id: String,
    pub utc_offset: String,
    pub converted_time: TimeZoneConvertedTime,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimeZoneLocation {
    pub place_name: String,
    pub time_zone: Vec<TimeZone>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimeZoneResource {
    #[serde(rename = "__type")]
    pub __type: String,

    pub time_zone_at_location: Vec<TimeZoneLocation>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimeZoneResourceSet {
    pub estimated_total: u64,
    pub resources: Vec<TimeZoneResource>,
}

#[derive(Deserialize)]
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
    pub async fn get<T: ToString>(location: T) -> Result<Self> {
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

        if timezones.is_empty() {
            bail!("Location not found.");
        }

        Ok(timezones.remove(0))
    }

    pub fn format(&self) -> String {
        let timezone = &self.time_zone[0];
        format!("It is `{} UTC {}` in {}.", timezone.converted_time.local_time.replace('T', " "), timezone.utc_offset, self.place_name)
    }
}
