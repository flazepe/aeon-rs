use crate::structs::{api::latex::LaTeX, command::Command, command_context::CommandContext};
use slashook::{
    command,
    commands::{Command as SlashookCommand, CommandInput, CommandResponder},
    structs::interactions::{IntegrationType, InteractionContextType, InteractionOptionType},
};
use std::sync::LazyLock;

static COMMAND: LazyLock<Command> = LazyLock::new(|| {
    Command::new().main(|ctx: CommandContext| async move {
        match LaTeX::render(
            ctx.get_string_arg("expression")?,
            ctx.get_string_arg("preamble").ok(),
            ctx.get_bool_arg("transparent").unwrap_or(false),
        )
        .await
        {
            Ok(image) => ctx.respond(image, false).await,
            Err(error) => ctx.respond_error(error, true).await,
        }
    })
});

pub fn get_command() -> SlashookCommand {
    #[command(
        name = "latex",
        description = "Renders a LaTeX expression.",
        integration_types = [IntegrationType::GUILD_INSTALL, IntegrationType::USER_INSTALL],
        contexts = [InteractionContextType::GUILD, InteractionContextType::BOT_DM, InteractionContextType::PRIVATE_CHANNEL],
        options = [
            {
                name = "expression",
                description = "The expression",
                option_type = InteractionOptionType::STRING,
                required = true,
            },
            {
                name = "preamble",
                description = "The preamble",
                option_type = InteractionOptionType::STRING,
            },
            {
                name = "transparent",
                description = "Whether to produce a transparent image",
                option_type = InteractionOptionType::BOOLEAN,
            },
        ],
    )]
    async fn latex(input: CommandInput, res: CommandResponder) {
        COMMAND.run(input, res).await?;
    }

    latex
}
