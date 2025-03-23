use crate::{statics::colors::ERROR_COLOR, structs::database::guilds::Guilds};
use anyhow::Result;
use slashook::structs::embeds::Embed;
use twilight_model::gateway::payload::incoming::MemberRemove;

pub async fn log(event: &MemberRemove) -> Result<()> {
    let embed = Embed::new()
        .set_color(ERROR_COLOR)
        .unwrap_or_default()
        .set_title("Member Left")
        .set_description(format!("<@{}>", event.user.id))
        .add_field("Username", format!("{} ({})", event.user.name, event.user.id), false);

    Guilds::send_log(event.guild_id, embed).await
}
