use crate::{
    statics::REST,
    structs::{command_context::AeonCommandContext, database::tags::Tags},
};
use anyhow::{Result, bail};
use slashook::{
    commands::MessageResponse,
    structs::{channels::Channel, messages::AllowedMentions},
};
use std::sync::Arc;

pub async fn run(ctx: Arc<AeonCommandContext>) -> Result<()> {
    let Some(guild_id) = ctx.get_guild_id() else { return Ok(()) };
    let channel_id = ctx.get_channel_id();
    let name = ctx.get_string_arg("tag")?;
    let tag = Tags::get(name, guild_id).await?;
    let nsfw_channel = Channel::fetch(&REST, channel_id).await.is_ok_and(|channel| channel.nsfw.unwrap_or(false));

    if tag.nsfw && !nsfw_channel {
        bail!("NSFW channels only.");
    }

    ctx.respond(MessageResponse::from(tag.content).set_allowed_mentions(AllowedMentions::new()), false).await
}
