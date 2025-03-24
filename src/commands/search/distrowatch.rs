use crate::structs::{
    command::Command,
    command_context::{CommandContext, CommandInputExt, Input},
    scraping::distrowatch::{Distribution, statics::DISTRIBUTIONS},
};
use slashook::{
    command,
    commands::{Command as SlashookCommand, CommandInput, CommandResponder},
    structs::interactions::{IntegrationType, InteractionContextType, InteractionOptionType},
};
use std::sync::LazyLock;

pub static COMMAND: LazyLock<Command> = LazyLock::new(|| {
    Command::new("distrowatch", &["distro"]).main(|ctx: CommandContext| async move {
        if let Input::ApplicationCommand { input, res: _ } = &ctx.input {
            if input.is_autocomplete() {
                return ctx.autocomplete(DISTRIBUTIONS.iter()).await;
            }
        }

        let distribution = match &ctx.input {
            Input::ApplicationCommand { input, res: _ } => input.get_string_arg("distribution")?,
            Input::MessageCommand { message: _, sender: _, args } => args.into(),
        };

        if distribution.is_empty() {
            return ctx.respond_error("Please provide a distribution.", true).await;
        }

        match Distribution::get(distribution).await {
            Ok(distribution) => ctx.respond(distribution.format(), false).await,
            Err(error) => ctx.respond_error(error, true).await,
        }
    })
});

pub fn get_slashook_command() -> SlashookCommand {
    #[command(
        name = COMMAND.name.clone(),
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
    async fn func(input: CommandInput, res: CommandResponder) {
        COMMAND.run(Input::ApplicationCommand { input, res }).await?;
    }

    func
}
