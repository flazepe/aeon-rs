use crate::structs::{api::ip_info::IpInfo, command::Command, command_context::CommandContext};
use once_cell::sync::Lazy;
use slashook::{
    command,
    commands::{Command as SlashookCommand, CommandInput, CommandResponder},
    structs::interactions::InteractionOptionType,
};

static COMMAND: Lazy<Command> = Lazy::new(|| {
    Command::new().main(|ctx: CommandContext| async move {
        match IpInfo::get(ctx.get_string_arg("ip")?).await {
            Ok(ip_info) => ctx.respond(ip_info.format(), false).await,
            Err(error) => ctx.respond_error(error, true).await,
        }
    })
});

pub fn get_command() -> SlashookCommand {
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
