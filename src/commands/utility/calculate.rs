use crate::{
    statics::REQWEST,
    structs::{
        command::AeonCommand,
        command_context::{AeonCommandContext, AeonCommandInput, CommandInputExt},
    },
};
use anyhow::{Context, bail};
use slashook::{
    command,
    commands::{Command as SlashookCommand, CommandInput, CommandResponder},
    structs::interactions::{IntegrationType, InteractionContextType, InteractionOptionType},
};
use std::{sync::Arc, sync::LazyLock, time::Duration};

pub static COMMAND: LazyLock<AeonCommand> = LazyLock::new(|| {
    AeonCommand::new("calculate", &["calc"]).main(|ctx: Arc<AeonCommandContext>| async move {
        let expression = match &ctx.command_input {
            AeonCommandInput::ApplicationCommand(input, _) => input.get_string_arg("expression")?,
            AeonCommandInput::MessageCommand(_, args, _) => args.into(),
        };

        if expression.is_empty() {
            bail!("Please provide an expression");
        }

        if expression.chars().all(|char| char.is_numeric()) {
            let fact = REQWEST.get(format!("http://numbersapi.com/{expression}")).send().await?.text().await?;
            return ctx.respond(fact, false).await;
        }

        let body = REQWEST
            .get("https://api.mathjs.org/v4/")
            .query(&[("expr", expression)])
            .timeout(Duration::from_secs(2))
            .send()
            .await
            .context("Calculation took too long.")?
            .text()
            .await?;

        if body.is_empty() || body.contains("Error") {
            bail!("Invalid expression.");
        }

        ctx.respond_success(format!("`{}`", body.chars().take(1000).collect::<String>().replace('`', "｀")), false).await
    })
});

pub fn get_slashook_command() -> SlashookCommand {
    #[command(
        name = COMMAND.name.clone(),
        description = "Calculates a mathematics expression.",
        integration_types = [IntegrationType::GUILD_INSTALL, IntegrationType::USER_INSTALL],
        contexts = [InteractionContextType::GUILD, InteractionContextType::BOT_DM, InteractionContextType::PRIVATE_CHANNEL],
        options = [
            {
                name = "expression",
                description = "The expression",
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
