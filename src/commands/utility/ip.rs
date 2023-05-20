use crate::structs::{api::ip_info::IPInfo, command::AeonCommand, command_context::CommandContext};
use once_cell::sync::Lazy;
use slashook::{
    command,
    commands::{Command, CommandInput, CommandResponder},
    structs::interactions::InteractionOptionType,
};

static COMMAND: Lazy<AeonCommand> = Lazy::new(|| {
    AeonCommand::new().main(|ctx: CommandContext| async move {
        match IPInfo::get(ctx.get_string_arg("ip")?).await {
            Ok(ip_info) => ctx.respond(ip_info.format(), false).await,
            Err(error) => ctx.respond_error(error, true).await,
        }
    })
});

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
