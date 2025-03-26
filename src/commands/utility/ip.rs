use crate::structs::{
    api::ip_info::IpInfo,
    command::AeonCommand,
    command_context::{AeonCommandContext, AeonCommandInput, CommandInputExt},
};
use anyhow::bail;
use slashook::{
    command,
    commands::{Command as SlashookCommand, CommandInput, CommandResponder},
    structs::interactions::{IntegrationType, InteractionContextType, InteractionOptionType},
};
use std::sync::{Arc, LazyLock};

pub static COMMAND: LazyLock<AeonCommand> = LazyLock::new(|| {
    AeonCommand::new("ip", &[]).main(|ctx: Arc<AeonCommandContext>| async move {
        let ip = match &ctx.command_input {
            AeonCommandInput::ApplicationCommand(input, _) => input.get_string_arg("ip")?,
            AeonCommandInput::MessageCommand(_, args, _) => args.into(),
        };

        if ip.is_empty() {
            bail!("Please provide an IP address.");
        }

        let ip_info = IpInfo::get(ip).await?;
        ctx.respond(ip_info.format(), false).await
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
        COMMAND.run(AeonCommandInput::ApplicationCommand(input, res)).await?;
    }

    func
}
