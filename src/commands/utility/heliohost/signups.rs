use crate::{macros::if_else, statics::colors::PRIMARY_COLOR, structs::duration::Duration};
use anyhow::Result;
use reqwest::Client;
use slashook::{
    chrono::{Datelike, Duration as ChronoDuration, TimeZone, Utc},
    commands::{CommandInput, CommandResponder},
    structs::embeds::Embed,
};
use std::time::{SystemTime, UNIX_EPOCH};

pub async fn run(_: CommandInput, res: CommandResponder) -> Result<()> {
    let now = Utc::now();

    let mut embed = Embed::new().set_color(PRIMARY_COLOR)?.set_description(format!(
        "[Signups](https://heliohost.org/signup/) will reset in: **{}**",
        Duration::new().parse(format!(
            "{}s",
            (Utc.with_ymd_and_hms(now.year(), now.month(), now.day(), 0, 0, 0)
                .unwrap()
                + ChronoDuration::days(1))
            .timestamp()
                - SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() as i64,
        ))?,
    ));

    for (server, plan) in [("Tommy", "2"), ("Ricky", "1"), ("Johnny", "9")] {
        embed = embed.add_field(
            server,
            if_else!(
                Client::new()
                    .get("https://heliohost.org/assets/monitor.php")
                    .query(&[("plan", plan)])
                    .send()
                    .await?
                    .text()
                    .await?
                    == "1",
                "Open",
                "Closed",
            ),
            true,
        );
    }

    res.send_message(embed).await?;

    Ok(())
}
