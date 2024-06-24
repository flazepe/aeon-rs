use crate::{
    functions::label_num,
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
        if !ctx.input.app_permissions.contains(Permissions::MANAGE_MESSAGES) {
            return ctx.respond_error("I do not have the Manage Messages permission to purge messages.", true).await;
        }

        if !ctx.input.member.as_ref().unwrap().permissions.as_ref().unwrap_or(&Permissions::empty()).contains(Permissions::MANAGE_MESSAGES)
        {
            return ctx.respond_error("You do not have the Manage Messages permission to purge messages.", true).await;
        }

        let channel = ctx.get_channel_arg("channel").unwrap_or(ctx.input.channel.as_ref().unwrap());

        let Ok(mut messages) = channel.fetch_messages(&ctx.input.rest, MessageFetchOptions::new().set_limit(100)).await else {
            return ctx.respond_error("An error occurred while trying to fetch messages. Please make sure I have the permission to view the channel and its messages.", true).await;
        };

        messages.retain(|message| {
            (match ctx.get_user_arg("user") {
                Ok(user) => message.author.id == user.id,
                Err(_) => true,
            }) && message.timestamp > Utc::now() - Duration::try_weeks(2).unwrap()
        });

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

        ctx.respond_success(
            format!(
                "Deleted {}{}.",
                label_num(messages.len(), "message", "messages"),
                match ctx.get_user_arg("user") {
                    Ok(user) => format!(" from {}", user.label()),
                    Err(_) => "".into(),
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
                required = true,
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
