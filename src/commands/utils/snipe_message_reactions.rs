use crate::{constants::*, structs::snipe::ReactionSnipes};
use anyhow::Context;
use slashook::{command, commands::*, structs::interactions::*};

pub fn get_command() -> Command {
    #[command(
        name = "Snipe Reactions",
        command_type = ApplicationCommandType::MESSAGE,
    )]
    fn snipe_message_reactions(input: CommandInput, res: CommandResponder) {
        match ReactionSnipes::new(
            input.guild_id,
            input.target_message.context("missing target message")?.id,
        )
        .to_response()
        {
            Ok(response) => {
                res.send_message(response).await?;
            }
            Err(error) => {
                res.send_message(format!("{ERROR_EMOJI} {error}")).await?;
            }
        }
    }

    snipe_message_reactions
}
