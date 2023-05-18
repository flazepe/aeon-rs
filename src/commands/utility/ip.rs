use crate::{
    structs::{api::ip_info::IPInfo, command::AeonCommand, command_context::CommandContext},
    traits::ArgGetters,
};
use anyhow::Result;
use slashook::{
    command,
    commands::{Command, CommandInput, CommandResponder},
    structs::interactions::InteractionOptionType,
};

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
        AeonCommand::new(input, res).main(run).run().await?;
    }

    ip
}

async fn run(ctx: CommandContext) -> Result<()> {
    match IPInfo::get(ctx.input.get_string_arg("ip")?).await {
        Ok(ip_info) => ctx.respond(ip_info.format(), false).await,
        Err(error) => ctx.respond_error(error, true).await,
    }
}
