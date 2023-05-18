use crate::{
    structs::{command_context::CommandContext, database::tags::Tags},
    traits::ArgGetters,
};
use anyhow::Result;
use slashook::{
    commands::MessageResponse,
    structs::channels::{AllowedMentions, Channel},
};

pub async fn run(ctx: CommandContext) -> Result<()> {
    match Tags::new().get(ctx.input.get_string_arg("tag")?, ctx.input.guild_id.as_ref().unwrap()).await {
        Ok(tag) => {
            if tag.nsfw {
                if !Channel::fetch(&ctx.input.rest, ctx.input.channel_id.as_ref().unwrap())
                    .await
                    .map_or(false, |channel| channel.nsfw.unwrap_or(false))
                {
                    return ctx.respond_error("NSFW channels only.", true).await;
                }
            }

            ctx.respond(MessageResponse::from(tag.content).set_allowed_mentions(AllowedMentions::new()), false).await
        },
        Err(error) => ctx.respond_error(error, true).await,
    }
}
