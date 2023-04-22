use crate::{statics::emojis::ERROR_EMOJI, structs::snipes::ReactionSnipes};
use slashook::{
    command,
    commands::{Command, CommandInput, CommandResponder, MessageResponse},
    structs::interactions::ApplicationCommandType,
};

pub fn get_command() -> Command {
    #[command(
        name = "Snipe Reactions",
        command_type = ApplicationCommandType::MESSAGE,
        dm_permission = false,
    )]
    async fn snipe_message_reactions(input: CommandInput, res: CommandResponder) {
        match ReactionSnipes::new(input.guild_id.unwrap(), input.target_message.unwrap().id).to_response() {
            Ok(response) => {
                res.send_message(response).await?;
            },
            Err(error) => {
                res.send_message(MessageResponse::from(format!("{ERROR_EMOJI} {error}")).set_ephemeral(true))
                    .await?;
            },
        };
    }

    snipe_message_reactions
}
