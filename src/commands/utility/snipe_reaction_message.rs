use crate::structs::{interaction::Interaction, snipes::ReactionSnipes};
use slashook::{
    command,
    commands::{Command, CommandInput, CommandResponder},
    structs::interactions::ApplicationCommandType,
};

pub fn get_command() -> Command {
    #[command(
        name = "Snipe Reactions",
        command_type = ApplicationCommandType::MESSAGE,
        dm_permission = false,
    )]
    async fn snipe_reaction_message(input: CommandInput, res: CommandResponder) {
        let Ok(interaction) = Interaction::new(&input, &res).verify().await else { return Ok(()); };

        match ReactionSnipes::new(input.guild_id.as_ref().unwrap(), &input.target_message.as_ref().unwrap().id).to_response() {
            Ok(response) => interaction.respond(response, false).await?,
            Err(error) => interaction.respond_error(error, true).await?,
        };
    }

    snipe_reaction_message
}