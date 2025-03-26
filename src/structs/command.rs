use crate::{
    functions::now,
    statics::{CACHE, FLAZEPE_ID},
    structs::command_context::{AeonCommandContext, AeonCommandInput, CommandInputExt},
};
use anyhow::Result;
use futures::{Future, future::BoxFuture};
use std::{fmt::Display, sync::Arc};

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

    pub fn owner_only(mut self) -> Self {
        self.owner_only = true;
        self
    }

    pub fn main<T: AeonCommandFn + 'static>(mut self, func: T) -> Self {
        self.func = Some(Box::new(func));
        self
    }

    pub fn subcommand<T: Display, U: AeonCommandFn + 'static>(mut self, name: T, aliases: &[&str], func: U) -> Self {
        self.subcommands.push(AeonSubcommand {
            name: name.to_string(),
            aliases: aliases.iter().map(|alias| alias.to_string()).collect(),
            func: Box::new(func),
        });
        self
    }

    pub async fn run(&self, command_input: AeonCommandInput) -> Result<()> {
        let mut ctx = AeonCommandContext::new(command_input);

        if let Err(error) = ctx.verify() {
            return ctx.respond_error(error, true).await;
        }

        if self.owner_only {
            let is_owner = match &ctx.command_input {
                AeonCommandInput::ApplicationCommand(input, _) => input.user.id == FLAZEPE_ID,
                AeonCommandInput::MessageCommand(message, _, _) => message.author.id.to_string() == FLAZEPE_ID,
            };

            if !is_owner {
                return ctx.respond_error("This command is owner-only.", true).await;
            }
        }

        let mut func = self.func.as_ref();

        match &mut ctx.command_input {
            AeonCommandInput::MessageCommand(_, args, _) => {
                let (subcommand_name, new_args) = args.split_once(' ').unwrap_or((args, ""));

                if !subcommand_name.is_empty() {
                    let subcommand_name = subcommand_name.to_lowercase();

                    let subcommand_exact_match =
                        self.subcommands.iter().find(|entry| entry.name == subcommand_name || entry.aliases.contains(&subcommand_name));
                    let subcommand_starts_with_match = self.subcommands.iter().find(|entry| {
                        entry.name.starts_with(&subcommand_name) || entry.aliases.iter().any(|alias| alias.starts_with(&subcommand_name))
                    });

                    if let Some(subcommand) = subcommand_exact_match.or(subcommand_starts_with_match) {
                        *args = new_args.to_string();
                        func = Some(&subcommand.func);
                    }
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

        let Some(func) = func else { return Ok(()) };

        let user_id = match &ctx.command_input {
            AeonCommandInput::ApplicationCommand(input, _) => input.user.id.clone(),
            AeonCommandInput::MessageCommand(message, _, _) => message.author.id.to_string(),
        };

        if CACHE.cooldowns.read().unwrap().get(&user_id).unwrap_or(&0) > &now() {
            return ctx.respond_error("You are under a cooldown. Try again later.", true).await;
        }

        match &ctx.command_input {
            AeonCommandInput::ApplicationCommand(input, _) => {
                // Only add cooldown to non-search commands
                if !input.get_bool_arg("search").unwrap_or(false) {
                    CACHE.cooldowns.write().unwrap().insert(input.user.id.clone(), now() + 3);
                }
            },
            AeonCommandInput::MessageCommand(message, _, _) => {
                CACHE.cooldowns.write().unwrap().insert(message.author.id.to_string(), now() + 3);
            },
        }

        let ctx_arc = Arc::new(ctx);

        if let Err(error) = func.call(ctx_arc.clone()).await {
            return ctx_arc.respond_error(error, true).await;
        }

        Ok(())
    }
}
