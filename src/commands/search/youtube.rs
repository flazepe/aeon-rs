use crate::{
    statics::REQWEST,
    structs::{
        command::Command,
        command_context::{CommandContext, CommandInputExt, Input},
    },
};
use slashook::{
    command,
    commands::{Command as SlashookCommand, CommandInput, CommandResponder},
    structs::interactions::{IntegrationType, InteractionContextType, InteractionOptionType},
};
use std::sync::LazyLock;

pub static COMMAND: LazyLock<Command> = LazyLock::new(|| {
    Command::new("youtube", &["yt"]).main(|ctx: CommandContext| async move {
        let Input::ApplicationCommand { input, res: _ } = &ctx.input else { return Ok(()) };
        let text = REQWEST
            .get("https://www.youtube.com/results")
            .query(&[("search_query", input.get_string_arg("video")?)])
            .send()
            .await?
            .text()
            .await?;
        let id = text.split(r#"videoId":""#).nth(1).unwrap_or("").split('"').next().unwrap();

        if id.is_empty() {
            return ctx.respond_error("Video not found.", true).await;
        }

        if input.channel.as_ref().and_then(|channel| channel.nsfw).unwrap_or(false) {
            ctx.respond(format!("https://www.youtube.com/watch?v={id}"), false).await
        } else {
            ctx.respond_error("NSFW channels only.", true).await
        }
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
        COMMAND.run(Input::ApplicationCommand { input, res }).await?;
    }

    func
}
