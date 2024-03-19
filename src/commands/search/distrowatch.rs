use crate::structs::{command::Command, command_context::CommandContext, scraping::distrowatch::Distro};
use once_cell::sync::Lazy;
use slashook::{
    command,
    commands::{Command as SlashookCommand, CommandInput, CommandResponder},
    structs::interactions::{IntegrationType, InteractionContextType, InteractionOptionType},
};

static COMMAND: Lazy<Command> = Lazy::new(|| {
    Command::new().main(|ctx: CommandContext| async move {
        match Distro::get(ctx.get_string_arg("distro")?).await {
            Ok(distro) => ctx.respond(distro.format(), false).await,
            Err(error) => ctx.respond_error(error, true).await,
        }
    })
});

pub fn get_command() -> SlashookCommand {
    #[command(
        name = "distrowatch",
        description = "Fetches a distribution from distrowatch.",
        integration_types = [IntegrationType::GUILD_INSTALL, IntegrationType::USER_INSTALL],
        contexts = [InteractionContextType::GUILD, InteractionContextType::BOT_DM, InteractionContextType::PRIVATE_CHANNEL],
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
