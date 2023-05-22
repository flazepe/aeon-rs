use crate::structs::{api::saucenao::SauceNAOSearch, command::Command, command_context::CommandContext};
use once_cell::sync::Lazy;
use slashook::{
    command,
    commands::{Command as SlashookCommand, CommandInput, CommandResponder},
    structs::interactions::InteractionOptionType,
};

static COMMAND: Lazy<Command> = Lazy::new(|| {
    Command::new().main(|ctx: CommandContext| async move {
        match SauceNAOSearch::query(
            match ctx.get_string_arg("image-url").or(ctx.get_attachment_arg("image-file").map(|attachment| attachment.url.clone())) {
                Ok(url) => url,
                Err(_) => return ctx.respond_error("Please provide an image URL or file.", true).await,
            },
        )
        .await
        {
            Ok(saucenao_search) => ctx.respond(saucenao_search.format(), false).await,
            Err(error) => ctx.respond_error(error, true).await,
        }
    })
});

pub fn get_command() -> SlashookCommand {
    #[command(
		name = "sauce",
		description = "Fetches sauce from an image.",
		options = [
			{
				name = "image-url",
				description = "The image URL",
				option_type = InteractionOptionType::STRING,
			},
			{
				name = "image-file",
				description = "The image file",
				option_type = InteractionOptionType::ATTACHMENT,
			},
		],
	)]
    async fn sauce(input: CommandInput, res: CommandResponder) {
        COMMAND.run(input, res).await?;
    }

    sauce
}
