mod render_replay;
mod user;

use crate::structs::command::Command;
use once_cell::sync::Lazy;
use slashook::{
    command,
    commands::{Command as SlashookCommand, CommandInput, CommandResponder},
    structs::interactions::{ApplicationCommandOptionChoice, InteractionOptionType},
};

static COMMAND: Lazy<Command> = Lazy::new(|| Command::new().subcommand("render-replay", render_replay::run).subcommand("user", user::run));

pub fn get_command() -> SlashookCommand {
    #[command(
        name = "osu",
        description = "Fetches various resources from osu!.",
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
    async fn osu(input: CommandInput, res: CommandResponder) {
        COMMAND.run(input, res).await?;
    }

    osu
}
