use crate::structs::{command_context::CommandContext, database::reminders::Reminders, duration::Duration};
use anyhow::Result;

pub async fn run(ctx: CommandContext) -> Result<()> {
    ctx.res.defer(ctx.input.is_string_select()).await?;

    // Delete snoozed reminder
    if let Some(message) = ctx.input.message.as_ref() {
        if message.interaction_metadata.is_none() {
            ctx.input.rest.delete::<()>(format!("channels/{}/messages/{}", message.channel_id, message.id)).await.ok();
        }
    }

    let user_id = &ctx.input.user.id;
    let url = {
        let guild_id = ctx.input.guild_id.as_deref().unwrap_or("@me");
        let channel_id = ctx.input.channel_id.as_ref().unwrap();
        let message_id = ctx.res.get_original_message().await?.id;
        ctx.input.custom_id.as_ref().map_or_else(|| format!("{guild_id}/{channel_id}/{message_id}"), |custom_id| custom_id.to_string())
    };
    let time = Duration::new()
        .parse(ctx.input.values.as_ref().map_or(ctx.get_string_arg("time").as_deref().unwrap_or(""), |values| values[0].as_str()))
        .unwrap_or_default();
    let interval = Duration::new().parse(ctx.get_string_arg("interval").unwrap_or_else(|_| "".into())).unwrap_or_default();
    let reminder = {
        let mut reminder = ctx.get_string_arg("reminder").unwrap_or_else(|_| "Do something".into());
        if ctx.input.is_string_select() {
            if let Some(parsed_reminder) = || -> Option<&String> { ctx.input.message.as_ref()?.embeds.first()?.description.as_ref() }() {
                reminder = parsed_reminder.to_string();
            };
        }
        reminder
    };
    let dm = ctx.input.message.as_ref().map_or(
        false,
        // DM if select menu's message was from an interaction
        |message| message.interaction_metadata.is_some(),
    ) || ctx.input.guild_id.is_none()
        || ctx.get_bool_arg("dm").unwrap_or(false);

    match Reminders::set(user_id, url, time, interval, reminder, dm).await {
        Ok(response) => ctx.respond_success(response, false).await,
        Err(error) => ctx.respond_error(error, true).await,
    }
}
