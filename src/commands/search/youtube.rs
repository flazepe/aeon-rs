use crate::{
    statics::REQWEST,
    structs::{command::Command, command_context::CommandContext},
};
use once_cell::sync::Lazy;
use slashook::{
    command,
    commands::{Command as SlashookCommand, CommandInput, CommandResponder},
    structs::interactions::{IntegrationType, InteractionContextType, InteractionOptionType},
};

static COMMAND: Lazy<Command> = Lazy::new(|| {
    Command::new().main(|ctx: CommandContext| async move {
        let text = REQWEST
            .get("https://www.youtube.com/results")
            .query(&[("search_query", ctx.get_string_arg("video")?)])
            .send()
            .await?
            .text()
            .await?;

        let id = text.split(r#"videoId":""#).nth(1).unwrap_or("").split('"').next().unwrap();

        if id.is_empty() {
            return ctx.respond_error("Video not found.", true).await;
        }

        match ctx.input.channel.as_ref().and_then(|channel| channel.nsfw).unwrap_or(false) {
            true => ctx.respond(format!("https://www.youtube.com/watch?v={id}"), false).await,
            false => ctx.respond_error("NSFW channels only.", true).await,
        }
    })
});

pub fn get_command() -> SlashookCommand {
    #[command(
		name = "youtube",
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
    async fn youtube(input: CommandInput, res: CommandResponder) {
        COMMAND.run(input, res).await?;
    }

    youtube
}
