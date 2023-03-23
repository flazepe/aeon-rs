use crate::{statics::emojis::*, structs::snipe::ReactionSnipes};
use slashook::{command, commands::*, structs::interactions::*};

pub fn get_command() -> Command {
    #[command(
        name = "Snipe Reactions",
        command_type = ApplicationCommandType::MESSAGE,
    )]
    fn snipe_message_reactions(input: CommandInput, res: CommandResponder) {
        match ReactionSnipes::new(input.guild_id, input.target_message.unwrap().id).to_response() {
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
