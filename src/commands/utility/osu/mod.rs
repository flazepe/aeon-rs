mod render_replay;
mod user;

use crate::structs::{command::AeonCommand, command_context::AeonCommandInput};
use slashook::{
    command,
    commands::{Command as SlashookCommand, CommandInput, CommandResponder},
    structs::interactions::{ApplicationCommandOptionChoice, IntegrationType, InteractionContextType, InteractionOptionType},
};
use std::sync::LazyLock;

pub static COMMAND: LazyLock<AeonCommand> = LazyLock::new(|| {
    AeonCommand::new("osu", &[]).subcommand("render-replay", &["render", "replay"], render_replay::run).subcommand("user", &[], user::run)
});

pub fn get_slashook_command() -> SlashookCommand {
    #[command(
        name = COMMAND.name.clone(),
        description = "Fetches various resources from osu!.",
        integration_types = [IntegrationType::GUILD_INSTALL, IntegrationType::USER_INSTALL],
        contexts = [InteractionContextType::GUILD, InteractionContextType::BOT_DM, InteractionContextType::PRIVATE_CHANNEL],
        subcommands = [
			{
                name = "render-replay",
                description = "Renders an osu! replay.",
                options = [
                    {
                        name = "replay-url",
                        description = "The URL to the replay file",
                        option_type = InteractionOptionType::STRING,
                    },
					{
                        name = "replay-file",
                        description = "The replay file",
                        option_type = InteractionOptionType::ATTACHMENT,
                    },
					{
                        name = "skin",
                        description = "The skin to use",
                        option_type = InteractionOptionType::STRING,
                        autocomplete = true,
                    },
                ],
            },
            {
                name = "user",
                description = "Fetches a user from osu!.",
                options = [
                    {
                        name = "user",
                        description = "The user",
                        option_type = InteractionOptionType::STRING,
                        required = true,
                    },
                    {
                        name = "mode",
                        description = "The mode",
                        option_type = InteractionOptionType::STRING,
                        choices = [
                            ApplicationCommandOptionChoice::new("osu!", "osu"),
                            ApplicationCommandOptionChoice::new("osu!taiko", "taiko"),
                            ApplicationCommandOptionChoice::new("osu!catch", "fruits"),
                            ApplicationCommandOptionChoice::new("osu!mania", "mania"),
                        ],
                    }
                ],
            },
        ],
    )]
    async fn func(input: CommandInput, res: CommandResponder) {
        COMMAND.run(AeonCommandInput::ApplicationCommand(input, res)).await?;
    }

    func
}
