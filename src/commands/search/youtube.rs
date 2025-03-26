use crate::{
    statics::REQWEST,
    structs::{
        command::AeonCommand,
        command_context::{AeonCommandContext, AeonCommandInput},
    },
};
use anyhow::bail;
use slashook::{
    command,
    commands::{Command as SlashookCommand, CommandInput, CommandResponder},
    structs::interactions::{IntegrationType, InteractionContextType, InteractionOptionType},
};
use std::sync::{Arc, LazyLock};

pub static COMMAND: LazyLock<AeonCommand> = LazyLock::new(|| {
    AeonCommand::new("youtube", &["yt"]).main(|ctx: Arc<AeonCommandContext>| async move {
        let AeonCommandInput::ApplicationCommand(input, _) = &ctx.command_input else { return Ok(()) };
        let text = REQWEST
            .get("https://www.youtube.com/results")
            .query(&[("search_query", ctx.get_string_arg("video")?)])
            .send()
            .await?
            .text()
            .await?;
        let id = text.split(r#"videoId":""#).nth(1).unwrap_or("").split('"').next().unwrap();

        if id.is_empty() {
            bail!("Video not found.");
        }

        if !input.channel.as_ref().and_then(|channel| channel.nsfw).unwrap_or(false) {
            bail!("NSFW channels only.")
        }

        ctx.respond(format!("https://www.youtube.com/watch?v={id}"), false).await
    })
});

pub fn get_slashook_command() -> SlashookCommand {
    #[command(
        name = COMMAND.name.clone(),
		description = "Searches for a video on YouTube.",
        integration_types = [IntegrationType::GUILD_INSTALL, IntegrationType::USER_INSTALL],
        contexts = [InteractionContextType::GUILD, InteractionContextType::BOT_DM, InteractionContextType::PRIVATE_CHANNEL],
		options = [
			{
				name = "video",
				description = "The video",
				option_type = InteractionOptionType::STRING,
				required = true,
			},
		],
	)]
    async fn func(input: CommandInput, res: CommandResponder) {
        COMMAND.run(AeonCommandInput::ApplicationCommand(input, res)).await?;
    }

    func
}
