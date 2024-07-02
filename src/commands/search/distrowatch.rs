use crate::structs::{
    command::Command,
    command_context::CommandContext,
    scraping::distrowatch::{statics::DISTRIBUTIONS, Distribution},
};
use once_cell::sync::Lazy;
use slashook::{
    command,
    commands::{Command as SlashookCommand, CommandInput, CommandResponder},
    structs::interactions::{IntegrationType, InteractionContextType, InteractionOptionType},
};

static COMMAND: Lazy<Command> = Lazy::new(|| {
    Command::new().main(|ctx: CommandContext| async move {
        if ctx.input.is_autocomplete() {
            return ctx.autocomplete(DISTRIBUTIONS.iter()).await;
        }

        match Distribution::get(ctx.get_string_arg("distribution")?).await {
            Ok(distribution) => ctx.respond(distribution.format(), false).await,
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
                name = "distribution",
                description = "The distribution",
                option_type = InteractionOptionType::STRING,
                autocomplete = true,
                required = true,
            },
        ],
    )]
    async fn distrowatch(input: CommandInput, res: CommandResponder) {
        COMMAND.run(input, res).await?;
    }

    distrowatch
}
