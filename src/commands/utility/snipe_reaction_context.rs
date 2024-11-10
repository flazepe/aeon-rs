use crate::structs::{command::Command, command_context::CommandContext, snipes::ReactionSnipes};
use std::sync::LazyLock;
use slashook::{
    command,
    commands::{Command as SlashookCommand, CommandInput, CommandResponder},
    structs::interactions::{ApplicationCommandType, IntegrationType, InteractionContextType},
};

static COMMAND: LazyLock<Command> = LazyLock::new(|| {
    Command::new().main(|ctx: CommandContext| async move {
        match ReactionSnipes::new(
            ctx.input.guild_id.as_ref().unwrap(),
            ctx.input.channel_id.as_ref().unwrap(),
            &ctx.input.target_message.as_ref().unwrap().id,
            ctx.input.app_permissions,
        )
        .to_response()
        {
            Ok(response) => ctx.respond(response, false).await,
            Err(error) => ctx.respond_error(error, true).await,
        }
    })
});

pub fn get_command() -> SlashookCommand {
    #[command(
        name = "Snipe Reactions",
        command_type = ApplicationCommandType::MESSAGE,
        integration_types = [IntegrationType::GUILD_INSTALL],
        contexts = [InteractionContextType::GUILD],
    )]
    async fn snipe_reaction_context(input: CommandInput, res: CommandResponder) {
        COMMAND.run(input, res).await?;
    }

    snipe_reaction_context
}
