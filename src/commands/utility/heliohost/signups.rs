use crate::{
    statics::colors::PRIMARY_COLOR,
    structs::{duration::Duration, interaction::Interaction},
};
use anyhow::Result;
use reqwest::Client;
use slashook::{
    chrono::{Datelike, Duration as ChronoDuration, TimeZone, Utc},
    commands::{CommandInput, CommandResponder},
    structs::embeds::Embed,
};
use std::time::{SystemTime, UNIX_EPOCH};

pub async fn run(input: CommandInput, res: CommandResponder) -> Result<()> {
    let Ok(interaction) = Interaction::new(&input, &res).verify().await else { return Ok(()); };
    let now = Utc::now();

    let mut embed = Embed::new().set_color(PRIMARY_COLOR)?.set_description(format!(
        "[Signups](https://heliohost.org/signup/) will reset in: **{}**",
        Duration::new().parse(
            (Utc.with_ymd_and_hms(now.year(), now.month(), now.day(), 0, 0, 0).unwrap() + ChronoDuration::days(1)).timestamp()
                - SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() as i64,
        )?,
    ));

    for (server, plan) in [("Tommy", "2"), ("Ricky", "1"), ("Johnny", "9")] {
        embed = embed.add_field(
            server,
            match Client::new().get("https://heliohost.org/assets/monitor.php").query(&[("plan", plan)]).send().await?.text().await? == "1"
            {
                true => "Open",
                false => "Closed",
            },
            true,
        );
    }

    interaction.respond(embed, false).await
}
