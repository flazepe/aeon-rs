use crate::structs::{command_context::AeonCommandContext, database::tags::Tags};
use anyhow::Result;
use slashook::{commands::MessageResponse, structs::messages::AllowedMentions};
use std::sync::Arc;

pub async fn run(ctx: Arc<AeonCommandContext>) -> Result<()> {
    let Some(guild_id) = ctx.get_guild_id() else { return Ok(()) };
    let name = ctx.get_string_arg("tag")?;
    let tag = Tags::get(name, guild_id).await?;

    if tag.nsfw {
        ctx.ensure_nsfw_channel().await?;
    }

    ctx.respond(MessageResponse::from(tag.content).set_allowed_mentions(AllowedMentions::new()), false).await
}
