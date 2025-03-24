use crate::structs::{
    command_context::{AeonCommandContext, CommandInputExt, AeonCommandInput},
    database::tags::Tags,
};
use anyhow::Result;
use slashook::{
    commands::MessageResponse,
    structs::{channels::Channel, messages::AllowedMentions},
};

pub async fn run(ctx: AeonCommandContext) -> Result<()> {
    let AeonCommandInput::ApplicationCommand(input,  _) = &ctx.command_input else { return Ok(()) };
    let name = input.get_string_arg("tag")?;
    let guild_id = input.guild_id.as_ref().unwrap();

    match Tags::get(name, guild_id).await {
        Ok(tag) => {
            let nsfw_channel =
                Channel::fetch(&input.rest, input.channel_id.as_ref().unwrap()).await.is_ok_and(|channel| channel.nsfw.unwrap_or(false));

            if tag.nsfw && !nsfw_channel {
                return ctx.respond_error("NSFW channels only.", true).await;
            }

            ctx.respond(MessageResponse::from(tag.content).set_allowed_mentions(AllowedMentions::new()), false).await
        },
        Err(error) => ctx.respond_error(error, true).await,
    }
}
