use crate::structs::{
    api::piston::Piston,
    command::AeonCommand,
    command_context::{AeonCommandContext, AeonCommandInput},
};
use slashook::{
    command,
    commands::{Command as SlashookCommand, CommandInput, CommandResponder, MessageResponse},
    structs::{
        interactions::{IntegrationType, InteractionContextType, InteractionOptionType},
        utils::File,
    },
};
use std::{sync::Arc, sync::LazyLock};

pub static COMMAND: LazyLock<AeonCommand> = LazyLock::new(|| {
    AeonCommand::new("eval", &["e", "ev", "evak"]).set_main(|ctx: Arc<AeonCommandContext>| async move {
        let code = ctx.get_string_arg("code", 0, true)?;
        let piston = Piston::new("javascript", code).run().await?;
        let output = piston.output.as_deref().unwrap_or("No output.");
        let response = if output.len() > 1993 {
            MessageResponse::from(File::new("output.txt", output))
        } else {
            MessageResponse::from(format!("```\n{output}```"))
        };

        ctx.respond(response, false).await
    })
});

pub fn get_slashook_command() -> SlashookCommand {
    #[command(
        name = COMMAND.name.clone(),
        description = "Evaluates a code (owner-only).",
        integration_types = [IntegrationType::GUILD_INSTALL, IntegrationType::USER_INSTALL],
        contexts = [InteractionContextType::GUILD, InteractionContextType::BOT_DM, InteractionContextType::PRIVATE_CHANNEL],
        options = [
            {
                name = "code",
                description = "The code",
                option_type = InteractionOptionType::STRING,
                required = true,
            },
        ],
    )]
    async fn func(input: CommandInput, res: CommandResponder) {
        COMMAND.run(AeonCommandInput::ApplicationCommand(Box::new(input), res)).await?;
    }

    func
}
