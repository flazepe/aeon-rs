use crate::{
    functions::now,
    statics::{colors::PRIMARY_COLOR, REQWEST},
    structs::{command_context::CommandContext, duration::Duration},
};
use anyhow::Result;
use slashook::{
    chrono::{Datelike, Duration as ChronoDuration, TimeZone, Utc},
    structs::embeds::Embed,
};

pub async fn run(ctx: CommandContext) -> Result<()> {
    let utc = Utc::now();

    let mut embed = Embed::new().set_color(PRIMARY_COLOR)?.set_description(format!(
        "[Signups](https://heliohost.org/signup/) will reset in: **{}**",
        Duration::new().parse(
            (Utc.with_ymd_and_hms(utc.year(), utc.month(), utc.day(), 0, 0, 0).unwrap() + ChronoDuration::try_days(1).unwrap()).timestamp()
                - now() as i64,
        )?,
    ));

    for (server, plan) in [("Tommy", "2"), ("Ricky", "1"), ("Johnny", "9")] {
        embed = embed.add_field(
            server,
            match REQWEST.get("https://heliohost.org/assets/monitor.php").query(&[("plan", plan)]).send().await?.text().await? == "1" {
                true => "Open",
                false => "Closed",
            },
            true,
        );
    }

    ctx.respond(embed, true).await
}
