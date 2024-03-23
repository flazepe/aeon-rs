use crate::structs::{
    api::{dictionary::Dictionary, urban_dictionary::UrbanDictionary},
    command::Command,
    command_context::CommandContext,
};
use once_cell::sync::Lazy;
use slashook::{
    command,
    commands::{Command as SlashookCommand, CommandInput, CommandResponder},
    structs::interactions::{IntegrationType, InteractionContextType, InteractionOptionType},
};

static COMMAND: Lazy<Command> = Lazy::new(|| {
    Command::new().main(|ctx: CommandContext| async move {
        let ephemeral = !ctx.get_bool_arg("show").unwrap_or(false);

        match ctx.get_bool_arg("urban-dictionary").unwrap_or(false) {
            true => match UrbanDictionary::search(ctx.get_string_arg("word")?).await {
                Ok(urban_dictionary) => ctx.respond(urban_dictionary.format(), ephemeral).await,
                Err(error) => ctx.respond_error(error, true).await,
            },
            false => match Dictionary::search(ctx.get_string_arg("word")?).await {
                Ok(dictionary) => ctx.respond(dictionary.format(), ephemeral).await,
                Err(error) => ctx.respond_error(error, true).await,
            },
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
                name = "urban-dictionary",
                description = "Whether to search the Urban Dictionary",
                option_type = InteractionOptionType::BOOLEAN,
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
