use crate::structs::{command::Command, command_context::CommandContext, snipes::ReactionSnipes};
use once_cell::sync::Lazy;
use slashook::{
    command,
    commands::{Command as SlashookCommand, CommandInput, CommandResponder},
    structs::interactions::ApplicationCommandType,
};

static COMMAND: Lazy<Command> = Lazy::new(|| {
    Command::new().main(|ctx: CommandContext| async move {
        match ReactionSnipes::new(ctx.input.guild_id.as_ref().unwrap(), &ctx.input.target_message.as_ref().unwrap().id).to_response() {
            Ok(response) => ctx.respond(response, false).await,
            Err(error) => ctx.respond_error(error, true).await,
        }
    })
});

pub fn get_command() -> SlashookCommand {
    #[command(
        name = "Snipe Reactions",
        command_type = ApplicationCommandType::MESSAGE,
        dm_permission = false,
    )]
    async fn snipe_reaction_context(input: CommandInput, res: CommandResponder) {
        COMMAND.run(input, res).await?;
    }

    snipe_reaction_context
}