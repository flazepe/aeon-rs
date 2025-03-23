use crate::{
    functions::label_num,
    statics::{CONFIG, FLAZEPE_ID},
    structs::{command::Command, command_context::CommandContext},
    traits::UserExt,
};
use slashook::{
    chrono::{Duration, Utc},
    command,
    commands::{Command as SlashookCommand, CommandInput, CommandResponder},
    structs::{
        Permissions,
        interactions::{IntegrationType, InteractionContextType, InteractionOptionType},
        messages::MessageFetchOptions,
    },
};
use std::sync::LazyLock;

static COMMAND: LazyLock<Command> = LazyLock::new(|| {
    Command::new().main(|ctx: CommandContext| async move {
        let has_permission = ctx.input.app_permissions.contains(Permissions::MANAGE_MESSAGES);
        let is_self_purge = ctx.get_user_arg("user").is_ok_and( |user| user.id == CONFIG.bot.client_id);

        if !has_permission && !is_self_purge {
            return ctx.respond_error("I do not have the Manage Messages permission to purge messages.", true).await;
        }

        let has_permission = ctx
            .input
            .member
            .as_ref()
            .is_some_and(|member| member.permissions.is_some_and(|permissions| permissions.contains(Permissions::MANAGE_MESSAGES)));
        let is_flazepe = ctx.input.user.id == FLAZEPE_ID;

        if !has_permission && !is_flazepe {
            return ctx.respond_error("You do not have the Manage Messages permission to purge messages.", true).await;
        }

        let channel = ctx.get_channel_arg("channel").unwrap_or(ctx.input.channel.as_ref().unwrap());

        let Ok(mut messages) = channel.fetch_messages(&ctx.input.rest, MessageFetchOptions::new().set_limit(100)).await else {
            return ctx.respond_error("An error occurred while trying to fetch messages. Please make sure I have the permission to view the channel and its messages.", true).await;
        };

        messages.retain(|message| {
            ctx.get_user_arg("user").map_or(true, |user| user.id == message.author.id)
                && message.timestamp > Utc::now() - Duration::weeks(2)
        });

        messages.drain((ctx.get_i64_arg("amount").unwrap_or(1) as usize).min(messages.len())..);

        if messages.is_empty() {
            return ctx.respond_error("No messages found.", true).await;
        }

        match messages.len() {
            1 => messages[0].delete(&ctx.input.rest).await?,
            _ => {
                if is_self_purge && !ctx.input.app_permissions.contains(Permissions::MANAGE_MESSAGES) {
                    ctx.defer(true).await?;

                    for message in &messages {
                        message.delete(&ctx.input.rest).await?;
                    }
                } else {
                    channel
                        .bulk_delete_messages(&ctx.input.rest, messages.iter().map(|message| message.id.clone()).collect::<Vec<String>>())
                        .await?
                }
            },
        };

        ctx.respond_success(
            format!(
                "Deleted {}{}{}.",
                label_num(messages.len(), "message", "messages"),
                ctx.get_user_arg("user").map(|user| format!(" from {}", user.label())).as_deref().unwrap_or(""),
                if channel.id != *ctx.input.channel_id.as_ref().unwrap() { format!(" in <#{}>", channel.id) } else { "".into() },
            ),
            true,
        )
        .await
    })
});

pub fn get_command() -> SlashookCommand {
    #[command(
        name = "purge",
        description = "Purges messages.",
        integration_types = [IntegrationType::GUILD_INSTALL],
        contexts = [InteractionContextType::GUILD],
        options = [
            {
                name = "amount",
                description = "The amount of messages to purge",
                option_type = InteractionOptionType::INTEGER,
                min_value = 1.0,
                max_value = 100.0,
            },
			{
                name = "user",
                description = "The user's messages to purge",
                option_type = InteractionOptionType::USER,
            },
			{
                name = "channel",
                description = "The channel to purge",
                option_type = InteractionOptionType::CHANNEL,
            },
        ],
    )]
    async fn purge(input: CommandInput, res: CommandResponder) {
        COMMAND.run(input, res).await?;
    }

    purge
}
