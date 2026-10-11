use crate::{
    statics::FLAZEPE_ID,
    structs::{
        command_context::{AeonCommandContext, AeonCommandInput},
        database::{Database, redis::keys::RedisKey},
        pagination::Pagination,
    },
};
use anyhow::Result;
use futures::{Future, future::BoxFuture};
use serde_json::Value;
use slashook::chrono::Utc;
use std::{fmt::Display, sync::Arc};
use tracing::error;

pub trait AeonCommandFn: Send + Sync {
    fn call(&self, ctx: Arc<AeonCommandContext>) -> BoxFuture<'static, Result<()>>;
}

impl<T: Fn(Arc<AeonCommandContext>) -> U + Send + Sync, U: Future<Output = Result<()>> + Send + 'static> AeonCommandFn for T {
    fn call(&self, ctx: Arc<AeonCommandContext>) -> BoxFuture<'static, Result<()>> {
        Box::pin(self(ctx))
    }
}

pub struct AeonCommand {
    pub name: String,
    pub aliases: Vec<String>,
    pub owner_only: bool,
    pub func: Option<Box<dyn AeonCommandFn>>,
    pub subcommands: Vec<AeonSubcommand>,
}

pub struct AeonSubcommand {
    pub name: String,
    pub aliases: Vec<String>,
    pub func: Box<dyn AeonCommandFn>,
}

impl AeonCommand {
    pub fn new<T: Display>(name: T, aliases: &[&str]) -> Self {
        Self {
            name: name.to_string(),
            aliases: aliases.iter().map(|alias| alias.to_string()).collect(),
            owner_only: false,
            func: None,
            subcommands: vec![],
        }
    }

    pub fn set_owner_only(mut self, owner_only: bool) -> Self {
        self.owner_only = owner_only;
        self
    }

    pub fn set_main<T: AeonCommandFn + 'static>(mut self, func: T) -> Self {
        self.func = Some(Box::new(func));
        self
    }

    pub fn add_subcommand<T: Display, U: AeonCommandFn + 'static>(mut self, name: T, aliases: &[&str], func: U) -> Self {
        self.subcommands.push(AeonSubcommand {
            name: name.to_string(),
            aliases: aliases.iter().map(|alias| alias.to_string()).collect(),
            func: Box::new(func),
        });
        self
    }

    pub async fn run(&self, command_input: AeonCommandInput) -> Result<()> {
        let mut ctx = AeonCommandContext::new(command_input);

        // Only reply to interactions that match the original interaction's user ID (if applicable)
        if let AeonCommandInput::ApplicationCommand(input, _) = &ctx.command_input
            && let Some(interaction_metadata) = input.message.as_ref().and_then(|message| message.interaction_metadata.as_ref())
            && input.user.id != interaction_metadata.user.id
        {
            return ctx.respond_error("This isn't your interaction.", true).await;
        }

        // Handle owner-only commands
        if self.owner_only && ctx.get_user_id() != FLAZEPE_ID {
            return ctx.respond_error("This command is owner-only.", true).await;
        }

        let mut func = self.func.as_ref();

        // If there are subcommands, try to find one that matches the input so that we can run its function
        if !self.subcommands.is_empty() {
            match &mut ctx.command_input {
                AeonCommandInput::MessageCommand(_, args, _) => {
                    let (subcommand, new_args) = args.get_content().split_once(char::is_whitespace).unwrap_or((args.get_content(), ""));
                    let subcommand = subcommand.to_lowercase();
                    let subcommand = self.subcommands.iter().find(|entry| entry.name == subcommand || entry.aliases.contains(&subcommand));

                    if let Some(subcommand) = subcommand {
                        *args = new_args.into();
                        func = Some(&subcommand.func);
                    } else {
                        let subcommands = self
                            .subcommands
                            .iter()
                            .map(|entry| {
                                format!(
                                    "`{}{}`",
                                    entry.name,
                                    if entry.aliases.is_empty() { "".into() } else { format!("|{}", entry.aliases.join("|")) },
                                )
                            })
                            .collect::<Vec<String>>()
                            .join(", ");

                        return ctx.respond_error(format!("Invalid subcommand. Valid subcommands: {subcommands}"), false).await;
                    }
                },
                AeonCommandInput::ApplicationCommand(input, _) => {
                    let subcommand = self
                        .subcommands
                        .iter()
                        .find(|entry| entry.name == input.subcommand.as_ref().or(input.custom_id.as_ref()).cloned().unwrap_or_default());

                    if let Some(subcommand) = subcommand {
                        func = Some(&subcommand.func);
                    }
                },
            }
        }

        let Some(func) = func else { return Ok(()) };

        // Handle existing cooldowns
        let redis = Database::get_redis()?;
        let cooldown_key = RedisKey::UserCooldown(ctx.get_user_id());

        if redis.get::<Value>(&cooldown_key).await.is_ok() {
            return ctx.respond_error("You are under a cooldown. Try again later.", true).await;
        }

        // Handle new cooldowns. For application commands, only add them if the input has no `search` option. For message commands, always add them
        let add_cooldown = match &ctx.command_input {
            AeonCommandInput::ApplicationCommand(input, _) => input.is_command() && !ctx.get_bool_arg("search").unwrap_or(false),
            AeonCommandInput::MessageCommand(_, _, _) => true,
        };

        if add_cooldown {
            redis.set(&cooldown_key, Utc::now().timestamp(), Some(3)).await?;
        }

        // Handle pagination (if applicable)
        if let AeonCommandInput::ApplicationCommand(input, res) = &ctx.command_input
            && let Some(custom_id) = &input.custom_id
            && custom_id.starts_with("pagination/")
        {
            let trimmed_custom_id = custom_id.replace("pagination/", "");
            let (id, action) = trimmed_custom_id.split_once('/').unwrap_or_default();
            let pagination_key = RedisKey::Paginations(id.to_string());

            let Ok(mut pagination) = redis.get::<Pagination>(&pagination_key).await else {
                return ctx.respond_error("This pagination is expired.", true).await;
            };

            match action {
                "first" => pagination.go_to_first_page(),
                "previous" => pagination.go_to_previous_page(),
                "counter" => {
                    let modal = pagination.generate_counter_modal();
                    res.open_modal(modal).await?;
                    return Ok(());
                },
                "go-to" => {
                    let page = ctx.get_string_arg("page", 0, false)?;

                    // Make sure that the parsed page is over 0, since we will be subtracting 1 to match the actual index
                    if let Ok(parsed_page) = page.parse::<usize>()
                        && parsed_page > 0
                    {
                        pagination.go_to_page(parsed_page - 1);
                    }

                    res.defer_update().await?;
                },
                "next" => pagination.go_to_next_page(),
                "last" => pagination.go_to_last_page(),
                _ => return Ok(()),
            }

            redis.set(&pagination_key, &pagination, Some(60 * 5)).await?;
            res.update_message(pagination.get_message_response()).await?;
            return Ok(());
        }

        let ctx_arc = Arc::new(ctx);

        if let Err(error) = func.call(ctx_arc.clone()).await {
            let command_name = &self.name;
            error!(target: "Command", "An error occurred while running command {command_name}: {error:#?}");
            return ctx_arc.respond_error(error, true).await;
        }

        Ok(())
    }
}
