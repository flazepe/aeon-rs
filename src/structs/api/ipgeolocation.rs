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
    pub offset: f64,
    pub offset_with_dst: f64,
    pub date_time_txt: String,
    pub time_24: String,
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
        let location = &[self.location.city.as_str(), self.location.state_prov.as_str(), self.location.country_name.as_str()]
            .into_iter()
            .filter(|entry| !entry.is_empty())
            .collect::<Vec<&str>>()
            .join(", ");

        let mut time_split = self.time_zone.time_24.split(":");
        let hour_24 = time_split.next().unwrap_or_default().parse::<u8>().unwrap_or_default();
        let hour_12 = if hour_24 == 0 || hour_24 == 12 { 12 } else { hour_24 % 12 };
        let min = time_split.next().unwrap_or_default();
        let am_pm = if hour_24 < 12 { "AM" } else { "PM" };

        let date = self.time_zone.date_time_txt.split(" ").take(4).collect::<Vec<&str>>().join(" ");

        let offset = if self.time_zone.is_dst { self.time_zone.offset_with_dst } else { self.time_zone.offset };
        let formatted_offset = format!(
            "UTC{}{:02}:{:02}",
            if offset >= 0.0 { "+" } else { "-" },
            offset.abs().trunc() as i32,
            (offset.abs().fract() * 60.0).round() as i32,
        );

        format!("It is `{hour_12}:{min} {am_pm}` or `{hour_24}:{min}` in {location} (`{date} / {formatted_offset}`).")
    }
}
