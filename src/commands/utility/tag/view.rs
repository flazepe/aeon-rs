use crate::{
    statics::REST,
    structs::{
        command_context::{AeonCommandContext, AeonCommandInput, CommandInputExt},
        database::tags::Tags,
    },
};
use anyhow::{Result, bail};
use slashook::{
    commands::MessageResponse,
    structs::{channels::Channel, messages::AllowedMentions},
};
use std::sync::Arc;

pub async fn run(ctx: Arc<AeonCommandContext>) -> Result<()> {
    let (name, guild_id, channel_id) = match &ctx.command_input {
        AeonCommandInput::ApplicationCommand(input, _) => {
            (input.get_string_arg("tag")?, input.guild_id.clone(), input.channel_id.as_ref().unwrap().clone())
        },
        AeonCommandInput::MessageCommand(message, args, _) => {
            (args.into(), message.guild_id.map(|guild_id| guild_id.to_string()), message.channel_id.to_string())
        },
    };

    let Some(guild_id) = guild_id else { return Ok(()) };

    if name.is_empty() {
        bail!("Please provide a name.");
    }

    let tag = Tags::get(name, guild_id).await?;
    let nsfw_channel = Channel::fetch(&REST, channel_id).await.is_ok_and(|channel| channel.nsfw.unwrap_or(false));

    if tag.nsfw && !nsfw_channel {
        bail!("NSFW channels only.");
    }

    ctx.respond(MessageResponse::from(tag.content).set_allowed_mentions(AllowedMentions::new()), false).await
}
