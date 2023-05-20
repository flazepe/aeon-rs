use crate::structs::{command::AeonCommand, command_context::CommandContext, scraping::distrowatch::Distro};
use anyhow::Result;
use once_cell::sync::Lazy;
use slashook::{
    command,
    commands::{Command, CommandInput, CommandResponder},
    structs::interactions::InteractionOptionType,
};

async fn run(ctx: CommandContext) -> Result<()> {
    match Distro::get(ctx.get_string_arg("distro")?).await {
        Ok(distro) => ctx.respond(distro.format(), false).await,
        Err(error) => ctx.respond_error(error, true).await,
    }
}

static COMMAND: Lazy<AeonCommand> = Lazy::new(|| AeonCommand::new().main(run));

pub fn get_command() -> Command {
    #[command(
        name = "distrowatch",
        description = "Fetches a distribution from distrowatch.",
        options = [
            {
                name = "distro",
                description = "The distribution",
                option_type = InteractionOptionType::STRING,
                required = true,
            },
        ],
    )]
    async fn distrowatch(input: CommandInput, res: CommandResponder) {
        COMMAND.run(input, res).await?;
    }

    distrowatch
}
