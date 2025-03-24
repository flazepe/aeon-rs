use crate::{
    statics::REQWEST,
    structs::{
        command::Command,
        command_context::{CommandContext, CommandInputExt, Input},
    },
};
use slashook::{
    command,
    commands::{Command as SlashookCommand, CommandInput, CommandResponder},
    structs::interactions::{IntegrationType, InteractionContextType, InteractionOptionType},
};
use std::{sync::LazyLock, time::Duration};

pub static COMMAND: LazyLock<Command> = LazyLock::new(|| {
    Command::new("calculate", &["calc"]).main(|ctx: CommandContext| async move {
        let expression = match &ctx.input {
            Input::ApplicationCommand(input,  _) => input.get_string_arg("expression")?,
            Input::MessageCommand(_, _, args)   => args.into(),
        };

        if expression.is_empty() {
            return ctx.respond_error("Please provide an expression", true).await;
        }

        if expression.chars().all(|char| char.is_numeric()) {
            let fact = REQWEST.get(format!("http://numbersapi.com/{expression}")).send().await?.text().await?;
            return ctx.respond(fact, false).await;
        }

        let body =
            match REQWEST.get("https://api.mathjs.org/v4/").query(&[("expr", expression)]).timeout(Duration::from_secs(2)).send().await {
                Ok(response) => response.text().await?,
                Err(_) => return ctx.respond_error("Calculation took too long.", true).await,
            };

        if body.is_empty() || body.contains("Error") {
            ctx.respond_error("Invalid expression.", true).await
        } else {
            ctx.respond_success(format!("`{}`", body.chars().take(1000).collect::<String>().replace('`', "｀")), false).await
        }
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
        COMMAND.run(Input::ApplicationCommand(input, res)).await?;
    }

    func
}
