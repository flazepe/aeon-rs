mod alias;
mod create;
mod delete;
mod edit;
mod list;
mod meta;
mod view;

use crate::{functions::hashmap_autocomplete, structs::tags::Tags};
use slashook::{
    command,
    commands::{Command, CommandInput, CommandResponder},
    structs::interactions::InteractionOptionType,
};
use std::collections::HashMap;

pub fn get_command() -> Command {
    #[command(
        name = "tag",
        description = "Sends or manages server tags.",
        dm_permission = false,
        subcommands = [
			{
                name = "alias",
                description = "Adds or removes an alias from a server tag.",
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
                name = "create",
                description = "Creates a new server tag.",	
                options = [
                    {
                        name = "tag",
                        description = "The tag",
                        option_type = InteractionOptionType::STRING,
                        max_length = 30,
						required = true,
                    },
					{
                        name = "content",
                        description = "The tag content",
                        option_type = InteractionOptionType::STRING,
                        max_length = 1000,
						required = true,
                    },
                ],
            },
			{
                name = "delete",
                description = "Deletes a server tag.",
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
                description = "Edits a server tag.",	
                options = [
                    {
                        name = "tag",
                        description = "The tag",
                        option_type = InteractionOptionType::STRING,
                        max_length = 30,
						autocomplete = true,
						required = true,
                    },
                    {
                        name = "name",
                        description = "The new tag name",
                        option_type = InteractionOptionType::STRING,
                        max_length = 30,
                    },
					{
                        name = "content",
                        description = "The new tag content",
                        option_type = InteractionOptionType::STRING,
                        max_length = 1000,
                    },
                    {
                        name = "nsfw",
                        description = "Whether the tag is NSFW",
                        option_type = InteractionOptionType::BOOLEAN,
                    },
                ],
            },
            {
                name = "list",
                description = "Creates a new server tag.",	
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
                description = "Sends a server tag information.",
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
                description = "Sends a server tag.",
                options = [
                    {
                        name = "tag",
                        description = "The tag",
                        option_type = InteractionOptionType::STRING,
                        max_length = 30,
						autocomplete = true,
						required = true,
                    },
                    {
                        name = "raw",
                        description = "Whether to send the raw tag content",
                        option_type = InteractionOptionType::BOOLEAN,
                    },
                ],
            },
        ],
    )]
    async fn tag(input: CommandInput, res: CommandResponder) {
        if input.is_autocomplete() {
            let guild_id = input.guild_id.as_ref().unwrap().clone();

            return hashmap_autocomplete(
                input,
                res,
                (HashMap::from_iter(
                    Tags::new()
                        .search(guild_id, None::<String>)
                        .await
                        .unwrap_or(vec![])
                        .iter()
                        .map(|tag| (tag.name.clone(), tag.name.clone())),
                ) as HashMap<String, String>)
                    .iter(),
            )
            .await?;
        }

        match input.subcommand.as_deref().unwrap_or("") {
            "alias" => alias::run(input, res).await?,
            "create" => create::run(input, res).await?,
            "delete" => delete::run(input, res).await?,
            "edit" => edit::run(input, res).await?,
            "list" => list::run(input, res).await?,
            "meta" => meta::run(input, res).await?,
            "view" => view::run(input, res).await?,
            _ => {},
        };
    }

    tag
}
