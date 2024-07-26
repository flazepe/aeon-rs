use crate::{
    functions::label_num,
    statics::{CONFIG, FLAZEPE_ID},
    structs::{command::Command, command_context::CommandContext},
    traits::UserExt,
};
use once_cell::sync::Lazy;
use slashook::{
    chrono::{Duration, Utc},
    command,
    commands::{Command as SlashookCommand, CommandInput, CommandResponder},
    structs::{
        interactions::{IntegrationType, InteractionContextType, InteractionOptionType},
        messages::MessageFetchOptions,
        Permissions,
    },
};

static COMMAND: Lazy<Command> = Lazy::new(|| {
    Command::new().main(|ctx: CommandContext| async move {
        let is_self_purge = ctx.get_user_arg("user").map_or(false, |user| user.id == CONFIG.bot.client_id);

        if !is_self_purge && !ctx.input.app_permissions.contains(Permissions::MANAGE_MESSAGES) {
            return ctx.respond_error("I do not have the Manage Messages permission to purge messages.", true).await;
        }

        if ctx.input.user.id != FLAZEPE_ID
            && !ctx
                .input
                .member
                .as_ref()
                .map_or(false, |member| member.permissions.map_or(false, |permissions| permissions.contains(Permissions::MANAGE_MESSAGES)))
        {
            return ctx.respond_error("You do not have the Manage Messages permission to purge messages.", true).await;
        }

        let channel = ctx.get_channel_arg("channel").unwrap_or(ctx.input.channel.as_ref().unwrap());

        let Ok(mut messages) = channel.fetch_messages(&ctx.input.rest, MessageFetchOptions::new().set_limit(100)).await else {
            return ctx.respond_error("An error occurred while trying to fetch messages. Please make sure I have the permission to view the channel and its messages.", true).await;
        };

        messages.retain(|message| {
            ctx.get_user_arg("user").map_or(true, |user| user.id == message.author.id)
                && message.timestamp > Utc::now() - Duration::try_weeks(2).unwrap()
        });

        messages.drain((ctx.get_i64_arg("amount").unwrap_or(1) as usize).min(messages.len())..);

        if messages.is_empty() {
            return ctx.respond_error("No messages found.", true).await;
        }

        match messages.len() {
            1 => messages[0].delete(&ctx.input.rest).await?,
            _ => match is_self_purge && !ctx.input.app_permissions.contains(Permissions::MANAGE_MESSAGES) {
                true => {
                    ctx.res.defer(true).await?;

                    for message in &messages {
                        message.delete(&ctx.input.rest).await?;
                    }
                },
                false => {
                    channel
                        .bulk_delete_messages(&ctx.input.rest, messages.iter().map(|message| message.id.clone()).collect::<Vec<String>>())
                        .await?
                },
            },
        };

        ctx.respond_success(
            format!(
                "Deleted {}{}{}.",
                label_num(messages.len(), "message", "messages"),
                ctx.get_user_arg("user").map(|user| format!(" from {}", user.label())).as_deref().unwrap_or(""),
                match channel.id != *ctx.input.channel_id.as_ref().unwrap() {
                    true => format!(" in <#{}>", channel.id),
                    false => "".into(),
                },
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
