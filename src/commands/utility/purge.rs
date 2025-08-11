use crate::{
    functions::label_num,
    statics::{CONFIG, FLAZEPE_ID},
    structs::{
        command::AeonCommand,
        command_context::{AeonCommandContext, AeonCommandInput},
    },
    traits::UserExt,
};
use anyhow::{Context, bail};
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
use std::sync::{Arc, LazyLock};

pub static COMMAND: LazyLock<AeonCommand> = LazyLock::new(|| {
    AeonCommand::new("purge", &[]).set_main(|ctx: Arc<AeonCommandContext>| async move {
        let AeonCommandInput::ApplicationCommand(input, _) = &ctx.command_input else { return Ok(()) };
        let bot_has_permission = input.app_permissions.contains(Permissions::MANAGE_MESSAGES);
        let is_self_purge = ctx.get_user_arg("user").is_ok_and(|user| user.id == CONFIG.bot.client_id);

        if !bot_has_permission && !is_self_purge {
            bail!("I do not have the Manage Messages permission to purge messages.");
        }

        let member_has_permission = input
            .member
            .as_ref()
            .is_some_and(|member| member.permissions.is_some_and(|permissions| permissions.contains(Permissions::MANAGE_MESSAGES)));
        let is_flazepe = input.user.id == FLAZEPE_ID;

        if !member_has_permission && !is_flazepe {
            bail!("You do not have the Manage Messages permission to purge messages.");
        }

        let channel = ctx.get_channel_arg("channel").unwrap_or(input.channel.as_ref().unwrap());
        let mut messages = channel
            .fetch_messages(&input.rest, MessageFetchOptions::new().set_limit(100))
            .await
            .context("An error occurred while trying to fetch messages. Please make sure I have the permission to view the channel and its messages.")?;
        let substring = ctx.get_string_arg("has-substring", 0, false).map(|substring| substring.to_lowercase());
        let keywords = ctx.get_string_arg("has-keywords", 0, false).map(|keywords| {
            keywords.split(",").map(|entry| entry.trim().to_lowercase()).filter(|entry| !entry.is_empty()).collect::<Vec<String>>()
        });

        messages.retain(|message| {
            if !ctx.get_user_arg("user").map_or(true, |user| message.author.as_ref().is_some_and(|author| author.id == user.id)) {
                return false;
            }

            if message.timestamp < Utc::now() - Duration::weeks(2) {
                return false;
            }

            if !substring.as_ref().map_or(true, |substring| message.content.to_lowercase().contains(substring)) {
                return false;
            }

            if !keywords.as_ref().map_or(true, |keywords| {
                let lowercased_content = message.content.to_lowercase();
                keywords.iter().any(|keyword| lowercased_content.contains(keyword))
            }) {
                return false;
            }

            if ctx.get_bool_arg("has-attachments").unwrap_or_default() && message.attachments.is_empty() {
                return false;
            }

            if ctx.get_bool_arg("has-embeds").unwrap_or_default() && message.embeds.is_empty() {
                return false;
            }

            true
        });

        messages.drain((ctx.get_i64_arg("amount", 0).unwrap_or(1) as usize).min(messages.len())..);

        if messages.is_empty() {
            bail!("No messages found.");
        }

        match messages.len() {
            1 => messages[0].delete(&input.rest).await?,
            _ => {
                if is_self_purge && !input.app_permissions.contains(Permissions::MANAGE_MESSAGES) {
                    ctx.defer(true).await?;

                    for message in &messages {
                        message.delete(&input.rest).await?;
                    }
                } else {
                    let messages = messages.iter().map(|message| message.id.clone().unwrap_or_default()).collect::<Vec<String>>();
                    channel.bulk_delete_messages(&input.rest, messages).await?
                }
            },
        };

        ctx.respond_success(
            format!(
                "Deleted {}{}{}.",
                label_num(messages.len(), "message", "messages"),
                ctx.get_user_arg("user").map(|user| format!(" from {}", user.label())).as_deref().unwrap_or_default(),
                if channel.id != *input.channel_id.as_ref().unwrap() { format!(" in <#{}>", channel.id) } else { "".into() },
            ),
            true,
        )
        .await
    })
});

pub fn get_slashook_command() -> SlashookCommand {
    #[command(
        name = COMMAND.name.clone(),
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
                name = "has-substring",
                description = "Whether to only delete messages that have this case-insensitive substring",
                option_type = InteractionOptionType::STRING,
            },
            {
                name = "has-keywords",
                description = "Whether to only delete messages that have these case-insensitive comma-separated keywords",
                option_type = InteractionOptionType::STRING,
            },
            {
                name = "has-attachments",
                description = "Whether to only delete messages that have attachments",
                option_type = InteractionOptionType::BOOLEAN,
            },
            {
                name = "has-embeds",
                description = "Whether to only delete messages that have embeds",
                option_type = InteractionOptionType::BOOLEAN,
            },
			{
                name = "channel",
                description = "The channel to purge",
                option_type = InteractionOptionType::CHANNEL,
            },
        ],
    )]
    async fn func(input: CommandInput, res: CommandResponder) {
        COMMAND.run(AeonCommandInput::ApplicationCommand(input, res)).await?;
    }

    func
}
