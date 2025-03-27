use crate::{statics::colors::SUCCESS_COLOR, structs::database::guilds::Guilds};
use anyhow::Result;
use slashook::{chrono::Utc, structs::embeds::Embed};
use twilight_model::gateway::payload::incoming::MemberAdd;

pub async fn handle(event: &MemberAdd) -> Result<()> {
    let embed = Embed::new()
        .set_color(SUCCESS_COLOR)
        .unwrap_or_default()
        .set_title("Member Joined")
        .set_description(format!("<@{}>", event.user.id))
        .add_field("Username", format!("{} ({})", event.user.name, event.user.id), false)
        .set_timestamp(Utc::now());

    Guilds::send_log(event.guild_id, embed).await
}
