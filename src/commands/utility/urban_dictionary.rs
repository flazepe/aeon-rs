use crate::structs::{
    api::urban_dictionary::UrbanDictionary,
    command::AeonCommand,
    command_context::{AeonCommandContext, AeonCommandInput},
};
use slashook::{
    command,
    commands::{Command as SlashookCommand, CommandInput, CommandResponder},
    structs::interactions::{IntegrationType, InteractionContextType, InteractionOptionType},
};
use std::{sync::Arc, sync::LazyLock};

pub static COMMAND: LazyLock<AeonCommand> = LazyLock::new(|| {
    AeonCommand::new("urban-dictionary", &["ud", "urban"]).set_main(async |ctx: Arc<AeonCommandContext>| {
        let ephemeral = match ctx.command_input {
            AeonCommandInput::ApplicationCommand(..) => !ctx.get_bool_arg("show").unwrap_or(false),
            AeonCommandInput::MessageCommand(..) => false,
        };

        let urban_dictionary = UrbanDictionary::search(ctx.get_string_arg("word", 0, true)?).await?;
        ctx.respond_pagination(urban_dictionary.format_pagination("urban-dictionary"), ephemeral).await
    })
});

pub fn get_slashook_command() -> SlashookCommand {
    #[command(
        name = COMMAND.name.clone(),
        description = "Defines a word from the Urban Dictionary.",
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
    async fn func(input: CommandInput, res: CommandResponder) {
        COMMAND.run(AeonCommandInput::ApplicationCommand(Box::new(input), res)).await?;
    }

    func
}
