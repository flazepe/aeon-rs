mod create;
mod delete;
mod edit;
mod list;
mod meta;
mod toggle_alias;
mod toggle_nsfw;
mod view;

use crate::structs::database::tags::Tags;
use anyhow::Context;
use slashook::{
    command,
    commands::{Command, CommandInput, CommandResponder},
    structs::interactions::{ApplicationCommandOptionChoice, InteractionOptionType},
};

pub fn get_command() -> Command {
    #[command(
        name = "tag",
        description = "Sends or manages server tags.",
        dm_permission = false,
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
                        max_length = 30,
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
                        max_length = 30,
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
                        max_length = 30,
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
                        max_length = 30,
						required = true,
                    },
					{
                        name = "alias",
                        description = "The tag alias to add or remove",
                        option_type = InteractionOptionType::STRING,
                        max_length = 30,
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
                        max_length = 30,
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
                        max_length = 30,
						autocomplete = true,
						required = true,
                    },
                ],
            },
        ],
    )]
    async fn tag(input: CommandInput, res: CommandResponder) {
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
                    Tags::new()
                        .search(input.guild_id.unwrap(), None::<String>)
                        .await
                        .unwrap_or(vec![])
                        .iter()
                        .filter(|tag| format!("{}{}", tag.name, tag.content).to_lowercase().contains(&value))
                        .take(25)
                        .map(|tag| ApplicationCommandOptionChoice::new(&tag.name, tag.name.clone()))
                        .collect::<Vec<ApplicationCommandOptionChoice>>(),
                )
                .await?;
        }

        match input.custom_id.as_deref().unwrap_or_else(|| input.subcommand.as_deref().unwrap_or("")) {
            "create" => create::run(input, res).await?,
            "delete" => delete::run(input, res).await?,
            "edit" => edit::run(input, res).await?,
            "list" => list::run(input, res).await?,
            "meta" => meta::run(input, res).await?,
            "toggle-alias" => toggle_alias::run(input, res).await?,
            "toggle-nsfw" => toggle_nsfw::run(input, res).await?,
            "view" => view::run(input, res).await?,
            _ => {},
        };
    }

    tag
}
