use crate::{
    statics::{MONGODB, colors::ERROR_EMBED_COLOR},
    traits::UserExt,
};
use anyhow::Result;
use slashook::{chrono::Utc, structs::embeds::Embed};
use twilight_model::gateway::payload::incoming::BanAdd;

pub async fn handle(event: &BanAdd) -> Result<()> {
    let embed = Embed::new()
        .set_color(ERROR_EMBED_COLOR)
        .unwrap_or_default()
        .set_title("Member Banned")
        .set_description(format!("<@{}>", event.user.id))
        .add_field("Username", event.user.label(), false)
        .set_timestamp(Utc::now());

    let mongodb = MONGODB.get().unwrap();
    mongodb.guilds.send_log(event.guild_id, embed, false).await
}
