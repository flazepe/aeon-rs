use crate::{
    functions::{TimestampFormat, format_timestamp},
    statics::colors::PRIMARY_EMBED_COLOR,
};
use anyhow::{Context, Result};
use slashook::{
    chrono::{DateTime, Utc},
    structs::embeds::Embed,
};
use std::fmt::Display;

pub struct Snowflake {
    pub snowflake: String,
    pub binary: String,
    pub timestamp: DateTime<Utc>,
    pub internal_worker_id: isize,
    pub internal_process_id: isize,
    pub increment: isize,
}

impl Snowflake {
    pub fn new<T: Display>(snowflake: T) -> Result<Self> {
        let snowflake = snowflake.to_string();
        let parsed = snowflake.parse::<u64>().context("Could not convert snowflake to `u64`.")?;
        let binary = format!("{parsed:064b}");

        let mut binary_chars = binary.chars();
        let parts = [
            binary_chars.by_ref().take(42).collect::<String>(),
            binary_chars.by_ref().take(5).collect::<String>(),
            binary_chars.by_ref().take(5).collect::<String>(),
            binary_chars.by_ref().take(12).collect::<String>(),
        ];

        let timestamp = i64::from_str_radix(&parts[0], 2)? + 1420070400000;
        let internal_worker_id = isize::from_str_radix(&parts[1], 2)?;
        let internal_process_id = isize::from_str_radix(&parts[2], 2)?;
        let increment = isize::from_str_radix(&parts[3], 2)?;

        Ok(Self {
            snowflake,
            binary: format!("{} {} {} {}", parts[0], parts[1], parts[2], parts[3]),
            timestamp: DateTime::from_timestamp_millis(timestamp).context("Invalid timestamp.")?,
            internal_worker_id,
            internal_process_id,
            increment,
        })
    }

    pub fn format(&self) -> Embed {
        Embed::new()
            .set_color(PRIMARY_EMBED_COLOR)
            .unwrap_or_default()
            .set_title(&self.snowflake)
            .set_description(format!("```{}```", self.binary))
            .add_field(
                "Timestamp",
                format!(
                    "{}\n`{}` (seconds)\n`{}` (milliseconds)",
                    format_timestamp(self.timestamp.timestamp(), TimestampFormat::Full),
                    self.timestamp.timestamp(),
                    self.timestamp.timestamp_millis(),
                ),
                false,
            )
            .add_field("Internal Worker ID", self.internal_worker_id, false)
            .add_field("Internal Process ID", self.internal_process_id, false)
            .add_field("Increment", self.increment, false)
    }
}
