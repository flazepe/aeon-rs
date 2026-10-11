use crate::structs::{
    api::dictionary::Dictionary,
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
    AeonCommand::new("dictionary", &["d", "define"]).set_main(async |ctx: Arc<AeonCommandContext>| {
        let ephemeral = match ctx.command_input {
            AeonCommandInput::ApplicationCommand(..) => !ctx.get_bool_arg("show").unwrap_or(false),
            AeonCommandInput::MessageCommand(..) => false,
        };

        let dictionary = Dictionary::search(ctx.get_string_arg("word", 0, true)?).await?;
        ctx.respond_pagination(dictionary.format_pagination("dictionary"), ephemeral).await
    })
});

pub fn get_slashook_command() -> SlashookCommand {
    #[command(
        name = COMMAND.name.clone(),
        description = "Defines a word from the dictionary.",
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
