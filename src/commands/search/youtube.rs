use crate::{
    statics::REQWEST,
    structs::{command::Command, command_context::CommandContext},
};
use once_cell::sync::Lazy;
use slashook::{
    command,
    commands::{Command as SlashookCommand, CommandInput, CommandResponder},
    structs::{channels::Channel, interactions::InteractionOptionType},
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

        let id = text.split(r#"videoId":""#).nth(1).unwrap_or("").split('"').next().unwrap();

        if id.is_empty() {
            return ctx.respond_error("Video not found.", true).await;
        }

        match Channel::fetch(&ctx.input.rest, ctx.input.channel_id.as_ref().unwrap())
            .await
            .map_or(false, |channel| channel.nsfw.unwrap_or(false))
        {
            true => ctx.respond(format!("https://www.youtube.com/watch?v={id}"), false).await,
            false => ctx.respond_error("NSFW channels only.", true).await,
        }

        /*
        let url = format!("https://www.youtube.com/watch?v={id}");

        match REQWEST.get(&url).send().await?.text().await?.contains("LOGIN_REQUIRED")
            && !Channel::fetch(&ctx.input.rest, ctx.input.channel_id.as_ref().unwrap())
                .await
                .map_or(false, |channel| channel.nsfw.unwrap_or(false))
        {
            true => ctx.respond_error("NSFW channels only.", true).await,
            false => ctx.respond(url, false).await,
        }
        */
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
