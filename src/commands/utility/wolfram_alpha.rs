use crate::structs::{
    api::wolfram_alpha::WolframAlpha,
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
    AeonCommand::new("wolfram-alpha", &[]).set_main(|ctx: Arc<AeonCommandContext>| async move {
        ctx.defer(false).await?;

        let query = ctx.get_string_arg("query", 0, true)?;
        let result = WolframAlpha::query(query).await?;
        let show_all = ctx.get_bool_arg("show-all").unwrap_or_default();

        ctx.respond(result.format_response(show_all), false).await
    })
});

pub fn get_slashook_command() -> SlashookCommand {
    #[command(
        name = COMMAND.name.clone(),
        description = "Queries Wolfram Alpha.",
        integration_types = [IntegrationType::GUILD_INSTALL, IntegrationType::USER_INSTALL],
        contexts = [InteractionContextType::GUILD, InteractionContextType::BOT_DM, InteractionContextType::PRIVATE_CHANNEL],
        options = [
            {
                name = "query",
                description = "The query",
                option_type = InteractionOptionType::STRING,
                required = true,
            },
            {
                name = "show-all",
                description = "Whether to show all results",
                option_type = InteractionOptionType::BOOLEAN,
            },
        ],
    )]
    async fn func(input: CommandInput, res: CommandResponder) {
        COMMAND.run(AeonCommandInput::ApplicationCommand(Box::new(input), res)).await?;
    }

    func
}
