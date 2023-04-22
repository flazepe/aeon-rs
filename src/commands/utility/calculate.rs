use crate::{
    macros::if_else,
    statics::emojis::{ERROR_EMOJI, SUCCESS_EMOJI},
    traits::ArgGetters,
};
use reqwest::get;
use slashook::{
    command,
    commands::{Command, CommandInput, CommandResponder, MessageResponse},
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

        let body = get(if_else!(
            expression.chars().all(|char| char.is_numeric()),
            format!("http://numbersapi.com/{expression}"),
            format!("https://api.mathjs.org/v4/?expr={expression}")
        ))
        .await?
        .text()
        .await?;

        res.send_message(if_else!(
            body.is_empty() || body.contains("Error"),
            MessageResponse::from(format!("{ERROR_EMOJI} Invalid expression.")).set_ephemeral(true),
            MessageResponse::from(format!("{SUCCESS_EMOJI} `{body}`"))
        ))
        .await?;
    }

    calculate
}
