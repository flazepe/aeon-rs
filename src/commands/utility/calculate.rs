use crate::{
    statics::REQWEST,
    structs::{command::Command, command_context::CommandContext},
};
use std::sync::LazyLock;
use slashook::{
    command,
    commands::{Command as SlashookCommand, CommandInput, CommandResponder},
    structs::interactions::{IntegrationType, InteractionContextType, InteractionOptionType},
};
use std::time::Duration;

static COMMAND: LazyLock<Command> = LazyLock::new(|| {
    Command::new().main(|ctx: CommandContext| async move {
        let expression = ctx.get_string_arg("expression")?;

        if expression.chars().all(|char| char.is_numeric()) {
            return ctx.respond(REQWEST.get(format!("http://numbersapi.com/{expression}")).send().await?.text().await?, false).await;
        }

        let body =
            match REQWEST.get("https://api.mathjs.org/v4/").query(&[("expr", expression)]).timeout(Duration::from_secs(2)).send().await {
                Ok(response) => response.text().await?,
                Err(_) => return ctx.respond_error("Calculation took too long.", true).await,
            };

        match body.is_empty() || body.contains("Error") {
            true => ctx.respond_error("Invalid expression.", true).await,
            false => ctx.respond_success(format!("`{}`", body.chars().take(1000).collect::<String>().replace('`', "｀")), false).await,
        }
    })
});

pub fn get_command() -> SlashookCommand {
    #[command(
        name = "calculate",
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
    async fn calculate(input: CommandInput, res: CommandResponder) {
        COMMAND.run(input, res).await?;
    }

    calculate
}
