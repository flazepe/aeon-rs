use crate::structs::{command_context::CommandContext, database::tags::Tags};
use anyhow::Result;
use slashook::{
    commands::MessageResponse,
    structs::{channels::Channel, messages::AllowedMentions},
};

pub async fn run(ctx: CommandContext) -> Result<()> {
    let name = ctx.get_string_arg("tag")?;
    let guild_id = ctx.input.guild_id.as_ref().unwrap();

    match Tags::get(name, guild_id).await {
        Ok(tag) => {
            let nsfw_channel = Channel::fetch(&ctx.input.rest, ctx.input.channel_id.as_ref().unwrap())
                .await
                .map_or(false, |channel| channel.nsfw.unwrap_or(false));

            if tag.nsfw && !nsfw_channel {
                return ctx.respond_error("NSFW channels only.", true).await;
            }

            ctx.respond(MessageResponse::from(tag.content).set_allowed_mentions(AllowedMentions::new()), false).await
        },
        Err(error) => ctx.respond_error(error, true).await,
    }
}
