use crate::{
    statics::REQWEST,
    structs::{command::Command, command_context::CommandContext},
};
use once_cell::sync::Lazy;
use slashook::{
    command,
    commands::{Command as SlashookCommand, CommandInput, CommandResponder},
    structs::interactions::InteractionOptionType,
};

static COMMAND: Lazy<Command> = Lazy::new(|| {
    Command::new().main(|ctx: CommandContext| async move {
        let text = REQWEST
            .get("https://www.youtube.com/results")
            .query(&[("search_query", ctx.get_string_arg("query")?)])
            .send()
            .await?
            .text()
            .await?;

        let id = text.split("videoId\":\"").skip(1).next().unwrap_or("").split('"').next().unwrap();

        match id.is_empty() {
            true => ctx.respond_error("Video not found.", true).await,
            false => ctx.respond(format!("https://www.youtube.com/watch?v={}", id), false).await,
        }
    })
});

pub fn get_command() -> SlashookCommand {
    #[command(
		name = "youtube",
		description = "Searches for a video on YouTube.",
		options = [
			{
				name = "query",
				description = "The query",
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
