use crate::structs::{
    command_context::{AeonCommandContext, AeonCommandInput, CommandInputExt},
    database::reminders::Reminders,
    duration::Duration,
};
use anyhow::Result;
use slashook::structs::Permissions;
use std::sync::Arc;

pub async fn run(ctx: Arc<AeonCommandContext>) -> Result<()> {
    let AeonCommandInput::ApplicationCommand(input, res) = &ctx.command_input else { return Ok(()) };

    // Must defer to not update original message
    res.defer(input.is_string_select()).await?;

    // Delete snoozed reminder
    if let Some(message) = input.message.as_ref() {
        if message.interaction_metadata.is_none() {
            let _ = input.rest.delete::<()>(format!("channels/{}/messages/{}", message.channel_id, message.id)).await;
        }
    }

    let user_id = &input.user.id;
    let url = {
        let guild_id = input.guild_id.as_deref().unwrap_or("@me");
        let channel_id = input.channel_id.as_ref().unwrap();
        let message_id = res.get_original_message().await?.id;
        input.custom_id.as_ref().map_or_else(|| format!("{guild_id}/{channel_id}/{message_id}"), |custom_id| custom_id.to_string())
    };
    let time = Duration::new()
        .parse(input.values.as_ref().map_or(input.get_string_arg("time").as_deref().unwrap_or(""), |values| values[0].as_str()))
        .unwrap_or_default();
    let interval = Duration::new().parse(input.get_string_arg("interval").unwrap_or_else(|_| "".into())).unwrap_or_default();
    let reminder = {
        let mut reminder = input.get_string_arg("reminder").unwrap_or_else(|_| "Do something".into());
        if input.is_string_select() {
            if let Some(parsed_reminder) = || -> Option<&String> { input.message.as_ref()?.embeds.first()?.description.as_ref() }() {
                reminder = parsed_reminder.to_string();
            };
        }
        reminder
    };
    let dm = input.get_bool_arg("dm").unwrap_or(false)
        || input.guild_id.is_none()
        || input.message.as_ref().is_some_and(|message| message.interaction_metadata.is_some()) // DM if select menu's message was from an interaction
        || !input.app_permissions.contains(Permissions::VIEW_CHANNEL | Permissions::SEND_MESSAGES);
    let response = Reminders::set(user_id, url, time, interval, reminder, dm).await?;

    ctx.respond_success(response, false).await
}
