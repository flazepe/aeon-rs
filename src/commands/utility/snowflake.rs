use crate::{
    statics::regex::EMOJI_REGEX,
    structs::{
        command::AeonCommand,
        command_context::{AeonCommandContext, AeonCommandInput},
        snowflake::Snowflake,
    },
};
use slashook::{
    command,
    commands::{Command as SlashookCommand, CommandInput, CommandResponder},
    structs::interactions::{IntegrationType, InteractionContextType, InteractionOptionType},
};
use std::{sync::Arc, sync::LazyLock};

pub static COMMAND: LazyLock<AeonCommand> = LazyLock::new(|| {
    AeonCommand::new("snowflake", &["id"]).set_main(|ctx: Arc<AeonCommandContext>| async move {
        let id = ctx.get_string_arg("snowflake", 0, true)?;
        let emoji_id = EMOJI_REGEX.captures(&id).and_then(|captures| captures.get(1).map(|capture| capture.as_str().to_string()));
        let snowflake = Snowflake::new(emoji_id.unwrap_or_else(|| id.chars().filter(|char| char.is_numeric()).collect::<String>()))?;

        ctx.respond(snowflake.format(), false).await
    })
});

pub fn get_slashook_command() -> SlashookCommand {
    #[command(
        name = COMMAND.name.clone(),
        description = "Shows Discord snowflake (ID) information.",
        integration_types = [IntegrationType::GUILD_INSTALL, IntegrationType::USER_INSTALL],
        contexts = [InteractionContextType::GUILD, InteractionContextType::BOT_DM, InteractionContextType::PRIVATE_CHANNEL],
        options = [
            {
                name = "snowflake",
                description = "The snowflake",
                option_type = InteractionOptionType::STRING,
                required = true,
            },
        ],
    )]
    async fn func(input: CommandInput, res: CommandResponder) {
        COMMAND.run(AeonCommandInput::ApplicationCommand(input, res)).await?;
    }

    func
}
