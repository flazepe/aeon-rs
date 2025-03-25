mod create;
mod delete;
mod edit;
mod list;
mod meta;
mod toggle_alias;
mod toggle_nsfw;
mod view;

use crate::structs::{command::AeonCommand, command_context::AeonCommandInput, database::tags::Tags};
use anyhow::Context;
use slashook::{
    command,
    commands::{Command as SlashookCommand, CommandInput, CommandResponder},
    structs::interactions::{ApplicationCommandOptionChoice, IntegrationType, InteractionContextType, InteractionOptionType},
};
use std::sync::LazyLock;

pub static COMMAND: LazyLock<AeonCommand> = LazyLock::new(|| {
    AeonCommand::new("tag", &["t"])
        .subcommand("create", &[], create::run)
        .subcommand("delete", &[], delete::run)
        .subcommand("edit", &[], edit::run)
        .subcommand("list", &["ls"], list::run)
        .subcommand("meta", &[], meta::run)
        .subcommand("toggle-alias", &[], toggle_alias::run)
        .subcommand("toggle-nsfw", &[], toggle_nsfw::run)
        .subcommand("view", &[], view::run)
});

pub fn get_slashook_command() -> SlashookCommand {
    #[command(
        name = COMMAND.name.clone(),
        description = "Sends or manages server tags.",
        integration_types = [IntegrationType::GUILD_INSTALL],
        contexts = [InteractionContextType::GUILD],
        subcommands = [
			{
                name = "create",
                description = "Creates a new tag.",
            },
			{
                name = "delete",
                description = "Deletes a tag.",
                options = [
                    {
                        name = "tag",
                        description = "The tag",
                        option_type = InteractionOptionType::STRING,
                        max_length = 32,
						autocomplete = true,
						required = true,
                    },
                ],
            },
			{
                name = "edit",
                description = "Edits a tag.",	
                options = [
                    {
                        name = "tag",
                        description = "The tag",
                        option_type = InteractionOptionType::STRING,
                        max_length = 32,
						autocomplete = true,
						required = true,
                    },
                ],
            },
            {
                name = "list",
                description = "Lists available tags.",
                options = [
                    {
                        name = "query",
                        description = "The query",
                        option_type = InteractionOptionType::STRING,
                    },
                    {
                        name = "author",
                        description = "The tag author",
                        option_type = InteractionOptionType::USER,
                    },
                ],
            },
			{
                name = "meta",
                description = "Sends tag information.",
                options = [
                    {
                        name = "tag",
                        description = "The tag",
                        option_type = InteractionOptionType::STRING,
                        max_length = 32,
						autocomplete = true,
						required = true,
                    },
                ],
            },
            {
                name = "toggle-alias",
                description = "Adds or removes an alias from a tag.",
                options = [
                    {
                        name = "tag",
                        description = "The tag",
                        option_type = InteractionOptionType::STRING,
                        max_length = 32,
						required = true,
                    },
					{
                        name = "alias",
                        description = "The tag alias to add or remove",
                        option_type = InteractionOptionType::STRING,
                        max_length = 32,
						required = true,
                    },
                ],
            },
            {
                name = "toggle-nsfw",
                description = "Toggles a tag's NSFW state.",
                options = [
                    {
                        name = "tag",
                        description = "The tag",
                        option_type = InteractionOptionType::STRING,
                        max_length = 32,
						autocomplete = true,
						required = true,
                    },
                ],
            },
            {
                name = "view",
                description = "Sends a tag.",
                options = [
                    {
                        name = "tag",
                        description = "The tag",
                        option_type = InteractionOptionType::STRING,
                        max_length = 32,
						autocomplete = true,
						required = true,
                    },
                ],
            },
        ],
    )]
    async fn func(input: CommandInput, res: CommandResponder) {
        if input.is_autocomplete() {
            let value = input
                .args
                .get(&input.focused.context("Missing focused arg.")?)
                .context("Could not get focused arg.")?
                .as_string()
                .context("Could not convert focused arg to String.")?
                .to_lowercase();

            return res
                .autocomplete(
                    Tags::search(input.guild_id.unwrap(), None::<String>)
                        .await
                        .unwrap_or_else(|_| vec![])
                        .iter()
                        .filter(|tag| format!("{}{}", tag.name, tag.content).to_lowercase().contains(&value))
                        .take(25)
                        .map(|tag| ApplicationCommandOptionChoice::new(&tag.name, tag.name.clone()))
                        .collect::<Vec<ApplicationCommandOptionChoice>>(),
                )
                .await?;
        }

        COMMAND.run(AeonCommandInput::ApplicationCommand(input, res)).await?;
    }

    func
}
