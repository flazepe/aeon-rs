use crate::structs::{
    command::Command,
    command_context::{CommandContext, Input},
    snipes::ReactionSnipes,
};
use slashook::{
    command,
    commands::{Command as SlashookCommand, CommandInput, CommandResponder},
    structs::interactions::{ApplicationCommandType, IntegrationType, InteractionContextType},
};
use std::sync::LazyLock;

pub static COMMAND: LazyLock<Command> = LazyLock::new(|| {
    Command::new("Snipe Reactions", &[]).main(|ctx: CommandContext| async move {
        let Input::ApplicationCommand(input, _) = &ctx.input else { return Ok(()) };

        match ReactionSnipes::new(
            input.guild_id.as_ref().unwrap(),
            input.channel_id.as_ref().unwrap(),
            &input.target_message.as_ref().unwrap().id,
            input.app_permissions,
        )
        .to_response()
        {
            Ok(response) => ctx.respond(response, false).await,
            Err(error) => ctx.respond_error(error, true).await,
        }
    })
});

pub fn get_slashook_command() -> SlashookCommand {
    #[command(
        name = COMMAND.name.clone(),
        command_type = ApplicationCommandType::MESSAGE,
        integration_types = [IntegrationType::GUILD_INSTALL],
        contexts = [InteractionContextType::GUILD],
    )]
    async fn func(input: CommandInput, res: CommandResponder) {
        COMMAND.run(Input::ApplicationCommand(input, res)).await?;
    }

    func
}
