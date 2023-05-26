use crate::{functions::plural, statics::CONFIG, structs::command_context::CommandContext};
use anyhow::Result;
use slashook::{
    chrono::{Duration, Utc},
    structs::channels::{Channel, MessageFetchOptions},
};

pub async fn run(ctx: CommandContext) -> Result<()> {
    let channel = match ctx.get_channel_arg("channel") {
        Ok(channel) => channel.clone(),
        Err(_) => Channel::fetch(&ctx.input.rest, ctx.input.channel_id.as_ref().unwrap()).await?,
    };

    let mut messages = match channel.fetch_messages(&ctx.input.rest, MessageFetchOptions::new().set_limit(100)).await {
        Ok(messages) => messages,
        Err(_) => return ctx.respond_error("An error occurred while trying to fetch messages.", true).await,
    };

    messages.retain(|message| message.author.id == CONFIG.bot.client_id && message.timestamp > Utc::now() - Duration::weeks(2));
    messages.drain((ctx.get_i64_arg("amount").unwrap_or(1) as usize).min(messages.len())..);

    if messages.is_empty() {
        return ctx.respond_error("No messages found.", true).await;
    }

    match messages.len() {
        1 => messages[0].delete(&ctx.input.rest).await?,
        _ => {
            channel
                .bulk_delete_messages(&ctx.input.rest, messages.iter().map(|message| message.id.clone()).collect::<Vec<String>>())
                .await?
        },
    };

    ctx.respond_success(format!("Deleted {}", plural(messages.len(), "message")), true).await
}
