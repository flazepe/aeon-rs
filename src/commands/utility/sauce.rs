use crate::structs::{api::saucenao::SauceNAOSearch, command::AeonCommand, command_context::CommandContext};
use anyhow::Result;
use slashook::{
    command,
    commands::{Command, CommandInput, CommandResponder},
    structs::interactions::InteractionOptionType,
};

pub fn get_command() -> Command {
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
        AeonCommand::new(input, res).main(run).run().await?;
    }

    sauce
}

async fn run(ctx: CommandContext) -> Result<()> {
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
}
