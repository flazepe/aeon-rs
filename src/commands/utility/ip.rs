use crate::structs::{
    api::ip_info::IpInfo,
    command::Command,
    command_context::{CommandContext, CommandInputExt, Input},
};
use slashook::{
    command,
    commands::{Command as SlashookCommand, CommandInput, CommandResponder},
    structs::interactions::{IntegrationType, InteractionContextType, InteractionOptionType},
};
use std::sync::LazyLock;

pub static COMMAND: LazyLock<Command> = LazyLock::new(|| {
    Command::new("ip", &[]).main(|ctx: CommandContext| async move {
        let ip = match &ctx.input {
            Input::ApplicationCommand { input, res: _ } => input.get_string_arg("ip")?,
            Input::MessageCommand { message: _, sender: _, args } => args.into(),
        };

        if ip.is_empty() {
            return ctx.respond_error("Please provide an IP address.", true).await;
        }

        match IpInfo::get(ip).await {
            Ok(ip_info) => ctx.respond(ip_info.format(), false).await,
            Err(error) => ctx.respond_error(error, true).await,
        }
    })
});

pub fn get_slashook_command() -> SlashookCommand {
    #[command(
        name = COMMAND.name.clone(),
        description = "Fetches information based on the given IP address.",
        integration_types = [IntegrationType::GUILD_INSTALL, IntegrationType::USER_INSTALL],
        contexts = [InteractionContextType::GUILD, InteractionContextType::BOT_DM, InteractionContextType::PRIVATE_CHANNEL],
        options = [
            {
                name = "ip",
                description = "The IP address",
                option_type = InteractionOptionType::STRING,
                required = true,
            },
        ],
    )]
    async fn func(input: CommandInput, res: CommandResponder) {
        COMMAND.run(Input::ApplicationCommand { input, res }).await?;
    }

    func
}
