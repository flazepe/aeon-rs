use crate::{
    statics::CONFIG,
    structs::{
        command::AeonCommand,
        command_context::{AeonCommandContext, AeonCommandInput},
    },
};
use slashook::{
    command,
    commands::{Command as SlashookCommand, CommandInput, CommandResponder},
    structs::interactions::{ApplicationCommandType, IntegrationType, InteractionContextType},
};
use std::sync::{Arc, LazyLock};

pub static COMMAND: LazyLock<AeonCommand> = LazyLock::new(|| {
    AeonCommand::new("Delete Response", &[]).set_main(|ctx: Arc<AeonCommandContext>| async move {
        let AeonCommandInput::ApplicationCommand(input, _) = &ctx.command_input else { return Ok(()) };
        let Some(message) = &input.target_message else { return Ok(()) };

        if message.author.as_ref().is_none_or(|author| author.id != CONFIG.bot.client_id) {
            return ctx.respond_error("This isn't my message.", true).await;
        }

        let deletable = if message.content.starts_with(&input.user.mention()) {
            true
        } else if let Some(interaction_metadata) = &message.interaction_metadata
            && interaction_metadata.user.id == input.user.id
        {
            true
        } else if let Some(referenced_message) = &message.referenced_message
            && referenced_message.author.as_ref().is_some_and(|author| author.id == input.user.id)
        {
            true
        } else if let Some(message_reference) = &message.message_reference
            && let Some(message_id) = &message_reference.message_id
            && let Some(channel) = &input.channel
            && let Ok(referenced_message) = channel.fetch_message(&input.rest, message_id).await
            && referenced_message.author.is_some_and(|author| author.id == input.user.id)
        {
            true
        } else {
            false
        };

        if deletable {
            message.delete(&input.rest).await?;
            ctx.respond_success("Gone.", true).await
        } else {
            ctx.respond_error("This isn't your command.", true).await
        }
    })
});

pub fn get_slashook_command() -> SlashookCommand {
    #[command(
        name = COMMAND.name.clone(),
        command_type = ApplicationCommandType::MESSAGE,
        integration_types = [IntegrationType::GUILD_INSTALL, IntegrationType::USER_INSTALL],
        contexts = [InteractionContextType::GUILD, InteractionContextType::BOT_DM, InteractionContextType::PRIVATE_CHANNEL],
    )]
    async fn func(input: CommandInput, res: CommandResponder) {
        COMMAND.run(AeonCommandInput::ApplicationCommand(Box::new(input), res)).await?;
    }

    func
}
