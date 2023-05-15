mod render_replay;

use slashook::{
    command,
    commands::{Command, CommandInput, CommandResponder},
    structs::interactions::InteractionOptionType,
};

pub fn get_command() -> Command {
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
        ],
    )]
    async fn osu(input: CommandInput, res: CommandResponder) {
        match input.subcommand.as_deref().unwrap_or("") {
            "render-replay" => render_replay::run(input, res).await?,
            _ => {},
        };
    }

    osu
}
