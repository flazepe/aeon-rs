use crate::statics::{CONFIG, REQWEST};
use anyhow::{Context, Result};
use serde::Deserialize;
use std::fmt::Display;

#[derive(Deserialize, Debug)]
pub struct IPGeolocationTimeZone {
    pub location: IPGeolocationTimeZoneLocation,
    pub time_zone: IPGeolocationTimeZoneTz,
}

#[derive(Deserialize, Debug)]
pub struct IPGeolocationTimeZoneLocation {
    pub country_name: String,
    pub state_prov: String,
    pub city: String,
}

#[derive(Deserialize, Debug)]
pub struct IPGeolocationTimeZoneTz {
    pub offset: i64,
    pub offset_with_dst: i64,
    pub date_time_txt: String,
    pub time_24: String,
    pub time_12: String,
    pub is_dst: bool,
}

impl IPGeolocationTimeZone {
    pub async fn get<T: Display>(location: T) -> Result<Self> {
        REQWEST
            .get("https://api.ipgeolocation.io/v2/timezone")
            .query(&[("apiKey", &CONFIG.api.ipgeolocation_key), ("location", &location.to_string())])
            .send()
            .await?
            .json()
            .await
            .context("Location not found.")
    }

    pub fn format(&self) -> String {
        let offset = if self.time_zone.is_dst { self.time_zone.offset_with_dst } else { self.time_zone.offset };

        format!(
            "It is `{}` or `{}` in {} (`{} - UTC{}`).",
            self.time_zone.time_12,
            self.time_zone.time_24,
            &[self.location.city.as_str(), self.location.state_prov.as_str(), self.location.country_name.as_str(),]
                .into_iter()
                .filter(|entry| !entry.is_empty())
                .collect::<Vec<&str>>()
                .join(", "),
            self.time_zone.date_time_txt.split(" ").take(4).collect::<Vec<&str>>().join(" "),
            if offset >= 0 { format!("+{offset}") } else { offset.to_string() },
        )
    }
}
