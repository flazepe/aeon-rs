use crate::statics::{colors::*, CONFIG};
use anyhow::{bail, Result};
use reqwest::get;
use serde::Deserialize;
use slashook::structs::embeds::Embed;
use std::fmt::Display;

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
    pub async fn get<T: Display>(location: T) -> Result<Self> {
        let timezones = &mut get(format!(
            "https://dev.virtualearth.net/REST/v1/TimeZone/?query={location}&key={}",
            CONFIG.api.virtualearth_key
        ))
        .await?
        .json::<TimeZoneResponse>()
        .await?
        .resource_sets[0]
            .resources[0]
            .time_zone_at_location;

        if timezones.len() > 0 {
            Ok(timezones.remove(0))
        } else {
            bail!("Location not found.");
        }
    }

    pub fn format(mut self) -> String {
        let timezone = self.time_zone.remove(0);

        format!(
            "It is `{} UTC {}` in `{}`.",
            timezone.converted_time.local_time.replace('T', " "),
            timezone.utc_offset,
            self.place_name
        )

        /*
        let mut entries = self
            .time_zone
            .iter()
            .map(|timezone| {
                format!(
                    "`{} UTC {}` - {}",
                    timezone.converted_time.local_time.replace('T', " "),
                    timezone.utc_offset,
                    timezone.converted_time.time_zone_display_name
                )
            })
            .collect::<Vec<String>>();

        while entries.join("\n").len() > 4000 {
            entries.pop();
        }

        Embed::new()
            .set_color(PRIMARY_COLOR)
            .unwrap_or_default()
            .set_title(self.place_name)
            .set_description(entries.join("\n"))
        */
    }
}
