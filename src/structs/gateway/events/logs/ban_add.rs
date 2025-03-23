use crate::{statics::colors::ERROR_COLOR, structs::database::guilds::Guilds};
use anyhow::Result;
use slashook::structs::embeds::Embed;
use twilight_model::gateway::payload::incoming::BanAdd;

pub async fn handle(event: &BanAdd) -> Result<()> {
    let embed = Embed::new()
        .set_color(ERROR_COLOR)
        .unwrap_or_default()
        .set_title("Member Banned")
        .set_description(format!("<@{}>", event.user.id))
        .add_field("Username", format!("{} ({})", event.user.name, event.user.id), false);

    Guilds::send_log(event.guild_id, embed).await
}
