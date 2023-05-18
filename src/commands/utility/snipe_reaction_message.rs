use crate::structs::{command::AeonCommand, command_context::CommandContext, snipes::ReactionSnipes};
use anyhow::Result;
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
        AeonCommand::new(input, res).main(run).run().await?;
    }

    snipe_reaction_message
}

async fn run(ctx: CommandContext) -> Result<()> {
    match ReactionSnipes::new(ctx.input.guild_id.as_ref().unwrap(), &ctx.input.target_message.as_ref().unwrap().id).to_response() {
        Ok(response) => ctx.respond(response, false).await,
        Err(error) => ctx.respond_error(error, true).await,
    }
}
