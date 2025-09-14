use crate::structs::{
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
    AeonCommand::new("edited", &[]).set_main(|ctx: Arc<AeonCommandContext>| async move {
        let AeonCommandInput::ApplicationCommand(_, res) = &ctx.command_input else { return Ok(()) };
        let left_text = ctx.get_string_arg("left-text", 0, false)?;
        let right_text = ctx.get_string_arg("right-text", 0, false);
        let to_edit = if let Ok(right_text) = right_text {
            format!("{left_text} \u{202b}{right_text} \u{202b}")
        } else {
            format!("\u{202b}{left_text} \u{202b}")
        };

        if ctx.get_bool_arg("codeblock").unwrap_or(false) {
            ctx.respond(format!("```\n{}```", to_edit.replace('`', "`\u{200b}").chars().take(1994).collect::<String>()), false).await?;
        } else {
            ctx.respond("lorem ipsum", false).await?;
            res.edit_original_message(to_edit.chars().take(2000).collect::<String>()).await?;
        }

        Ok(())
    })
});

pub fn get_slashook_command() -> SlashookCommand {
    #[command(
        name = COMMAND.name.clone(),
        description = "Sends a silly edited text.",
        integration_types = [IntegrationType::GUILD_INSTALL, IntegrationType::USER_INSTALL],
        contexts = [InteractionContextType::GUILD, InteractionContextType::BOT_DM, InteractionContextType::PRIVATE_CHANNEL],
        options = [
            {
                name = "left-text",
                description = "The left text",
                option_type = InteractionOptionType::STRING,
                required = true,
            },
			{
                name = "right-text",
                description = "The right text",
                option_type = InteractionOptionType::STRING,
            },
			{
                name = "codeblock",
                description = "Whether to send the edited text in a codeblock",
                option_type = InteractionOptionType::BOOLEAN,
            },
        ],
    )]
    async fn func(input: CommandInput, res: CommandResponder) {
        COMMAND.run(AeonCommandInput::ApplicationCommand(Box::new(input), res)).await?;
    }

    func
}
