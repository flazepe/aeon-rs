use crate::{
    commands::COMMANDS,
    structs::{
        command::AeonCommand,
        command_context::{AeonCommandContext, AeonCommandInput},
    },
};
use slashook::{
    command,
    commands::{Command as SlashookCommand, CommandInput, CommandResponder},
    structs::interactions::{IntegrationType, InteractionContextType},
};
use std::sync::{Arc, LazyLock};

pub static COMMAND: LazyLock<AeonCommand> = LazyLock::new(|| {
    AeonCommand::new("help", &["h"]).set_main(|ctx: Arc<AeonCommandContext>| async move {
        let format_names = |name: &String, aliases: &Vec<String>| {
            let mut names = vec![name.clone()];
            names.append(&mut aliases.clone());
            names.join("|")
        };

        let commands = COMMANDS
            .iter()
            .map(|command| {
                let command_name = format_names(&command.name, &command.aliases);

                if command.subcommands.is_empty() {
                    format!("`{command_name}`")
                } else {
                    command
                        .subcommands
                        .iter()
                        .map(|subcommand| {
                            let subcommand_name = format_names(&subcommand.name, &subcommand.aliases);
                            format!("`{command_name} {subcommand_name}`")
                        })
                        .collect::<Vec<String>>()
                        .join("\n")
                }
            })
            .collect::<Vec<String>>()
            .join("\n");

        ctx.respond(format!("# Commands\n{commands}"), true).await
    })
});

pub fn get_slashook_command() -> SlashookCommand {
    #[command(
        name = COMMAND.name.clone(),
        description = "Help command.",
        integration_types = [IntegrationType::GUILD_INSTALL, IntegrationType::USER_INSTALL],
        contexts = [InteractionContextType::GUILD, InteractionContextType::BOT_DM, InteractionContextType::PRIVATE_CHANNEL],
    )]
    async fn func(input: CommandInput, res: CommandResponder) {
        COMMAND.run(AeonCommandInput::ApplicationCommand(Box::new(input), res)).await?;
    }

    func
}
