use crate::structs::{api::ip_info::IPInfo, command::AeonCommand, command_context::CommandContext};
use anyhow::Result;
use once_cell::sync::Lazy;
use slashook::{
    command,
    commands::{Command, CommandInput, CommandResponder},
    structs::interactions::InteractionOptionType,
};

async fn run(ctx: CommandContext) -> Result<()> {
    match IPInfo::get(ctx.get_string_arg("ip")?).await {
        Ok(ip_info) => ctx.respond(ip_info.format(), false).await,
        Err(error) => ctx.respond_error(error, true).await,
    }
}

static COMMAND: Lazy<AeonCommand> = Lazy::new(|| AeonCommand::new().main(run));

pub fn get_command() -> Command {
    #[command(
        name = "ip",
        description = "Fetches information based on the given IP address.",
        options = [
            {
                name = "ip",
                description = "The IP address",
                option_type = InteractionOptionType::STRING,
                required = true,
            },
        ],
    )]
    async fn ip(input: CommandInput, res: CommandResponder) {
        COMMAND.run(input, res).await?;
    }

    ip
}
