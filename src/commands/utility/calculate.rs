use crate::{
    statics::REQWEST,
    structs::{command::AeonCommand, command_context::CommandContext},
};
use once_cell::sync::Lazy;
use slashook::{
    command,
    commands::{Command, CommandInput, CommandResponder},
    structs::interactions::InteractionOptionType,
};

static COMMAND: Lazy<AeonCommand> = Lazy::new(|| {
    AeonCommand::new().main(|ctx: CommandContext| async move {
        let expression = ctx.get_string_arg("expression")?;

        let body = match expression.chars().all(|char| char.is_numeric()) {
            true => REQWEST.get(format!("http://numbersapi.com/{expression}")).send().await?,
            false => REQWEST.get("https://api.mathjs.org/v4/").query(&[("expr", expression.as_str())]).send().await?,
        }
        .text()
        .await?
        .replace("`", "｀")
        .chars()
        .take(1000)
        .collect::<String>();

        match body.is_empty() || body.contains("Error") {
            true => ctx.respond_error("Invalid expression.", true).await,
            false => ctx.respond_success(format!("`{body}`"), false).await,
        }
    })
});

pub fn get_command() -> Command {
    #[command(
        name = "calculate",
        description = "Calculates a mathematics expression.",
        options = [
            {
                name = "expression",
                description = "The expression",
                option_type = InteractionOptionType::STRING,
            },
        ],
    )]
    async fn calculate(input: CommandInput, res: CommandResponder) {
        COMMAND.run(input, res).await?;
    }

    calculate
}
