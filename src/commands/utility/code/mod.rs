mod delete;
mod list;
mod run;

use crate::structs::{api::piston::Piston, command::AeonCommand, command_context::AeonCommandInput, database::Database};
use slashook::{
    command,
    commands::{Command as SlashookCommand, CommandInput, CommandResponder},
    structs::interactions::{ApplicationCommandOptionChoice, IntegrationType, InteractionContextType, InteractionOptionType},
};
use std::sync::LazyLock;

pub static COMMAND: LazyLock<AeonCommand> = LazyLock::new(|| {
    AeonCommand::new("code", &[])
        .add_subcommand("run", &["execute"], run::run)
        .add_subcommand("delete", &["del"], delete::run)
        .add_subcommand("list", &["ls"], list::run)
});

pub fn get_slashook_command() -> SlashookCommand {
    #[command(
        name = COMMAND.name.clone(),
        description = "Run or update code snippets.",
        integration_types = [IntegrationType::GUILD_INSTALL, IntegrationType::USER_INSTALL],
        contexts = [InteractionContextType::GUILD, InteractionContextType::BOT_DM, InteractionContextType::PRIVATE_CHANNEL],
        subcommands = [
            {
                name = "run",
                description = "Runs code.",
                options = [
                    {
                        name = "programming-language",
                        description = "The programming language",
                        option_type = InteractionOptionType::STRING,
                        autocomplete = true,
                    },
                    {
                        name = "name",
                        description = "The code snippet name to create or run",
                        option_type = InteractionOptionType::STRING,
                        autocomplete = true,
                    },
                ],
            },
            {
                name = "delete",
                description = "Deletes a code snippet.",
                options = [
                    {
                        name = "name",
                        description = "The code snippet name to delete",
                        option_type = InteractionOptionType::STRING,
                        autocomplete = true,
                        required = true,
                    },
                ],
            },
            {
                name = "list",
                description = "Lists all your code snippets.",
            }
        ],
    )]
    async fn func(input: CommandInput, res: CommandResponder) {
        if input.is_autocomplete() {
            let focused = input.focused.as_deref().unwrap_or_default();
            let value = input.args.get(focused).and_then(|option_value| option_value.as_string()).unwrap_or_default().to_lowercase();

            match focused {
                "programming-language" => res.autocomplete(Piston::autocomplete_runtime(&value)).await?,
                "name" => {
                    let mongodb = Database::get_mongodb()?;
                    let code_snippets = mongodb.code_snippets.get_all(input.user.id).await.unwrap_or_default();
                    let option_choices = code_snippets
                        .iter()
                        .filter(|code_snippet| code_snippet.name.contains(&value))
                        .take(25)
                        .map(|code_snippet| ApplicationCommandOptionChoice::new(&code_snippet.name, code_snippet.name.clone()))
                        .collect();

                    res.autocomplete(option_choices).await?;
                },
                _ => {},
            }

            return;
        }

        COMMAND.run(AeonCommandInput::ApplicationCommand(Box::new(input), res)).await?;
    }

    func
}
