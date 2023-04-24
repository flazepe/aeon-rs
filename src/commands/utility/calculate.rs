use crate::{macros::if_else, structs::interaction::Interaction, traits::ArgGetters};
use reqwest::{get, Client};
use slashook::{
    command,
    commands::{Command, CommandInput, CommandResponder},
    structs::interactions::InteractionOptionType,
};

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
        let expression = input.get_string_arg("expression")?;

        let body = if_else!(
            expression.chars().all(|char| char.is_numeric()),
            get(format!("http://numbersapi.com/{expression}")).await?,
            Client::new()
                .get("https://api.mathjs.org/v4/")
                .query(&[("expr", expression.as_str())])
                .send()
                .await?
        )
        .text()
        .await?
        .replace("`", "｀")
        .chars()
        .take(1000)
        .collect::<String>();

        let interaction = Interaction::new(&input, &res);

        if_else!(
            body.is_empty() || body.contains("Error"),
            interaction.respond_error("Invalid expression.", true).await?,
            interaction.respond_success(format!("`{body}`"), false).await?
        )
    }

    calculate
}
