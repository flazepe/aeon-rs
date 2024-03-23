use crate::structs::{api::dictionary::Dictionary, command::Command, command_context::CommandContext};
use once_cell::sync::Lazy;
use slashook::{
    command,
    commands::{Command as SlashookCommand, CommandInput, CommandResponder},
    structs::interactions::{IntegrationType, InteractionContextType, InteractionOptionType},
};

static COMMAND: Lazy<Command> = Lazy::new(|| {
    Command::new().main(|ctx: CommandContext| async move {
        match Dictionary::search(ctx.get_string_arg("word")?).await {
            Ok(dictionary) => ctx.respond(dictionary[0].format(), !ctx.get_bool_arg("show").unwrap_or(false)).await,
            Err(error) => ctx.respond_error(error, true).await,
        }
    })
});

pub fn get_command() -> SlashookCommand {
    #[command(
        name = "define",
        description = "Defines a word.",
        integration_types = [IntegrationType::GUILD_INSTALL, IntegrationType::USER_INSTALL],
        contexts = [InteractionContextType::GUILD, InteractionContextType::BOT_DM, InteractionContextType::PRIVATE_CHANNEL],
        options = [
            {
                name = "word",
                description = "The word",
                option_type = InteractionOptionType::STRING,
                required = true,
            },
			{
                name = "show",
                description = "Whether to publicly show the response",
                option_type = InteractionOptionType::BOOLEAN,
            },
        ],
    )]
    async fn define(input: CommandInput, res: CommandResponder) {
        COMMAND.run(input, res).await?;
    }

    define
}
