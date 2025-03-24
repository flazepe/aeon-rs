use crate::{
    statics::FLAZEPE_ID,
    structs::command_context::{AeonCommandContext, AeonCommandInput},
};
use anyhow::Result;
use futures::{Future, future::BoxFuture};
use std::fmt::Display;

pub trait AeonCommandFn: Send + Sync {
    fn call(&self, ctx: AeonCommandContext) -> BoxFuture<'static, Result<()>>;
}

impl<T: Fn(AeonCommandContext) -> U + Send + Sync, U: Future<Output = Result<()>> + Send + 'static> AeonCommandFn for T {
    fn call(&self, ctx: AeonCommandContext) -> BoxFuture<'static, Result<()>> {
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
        let Ok(mut ctx) = AeonCommandContext::new(command_input).verify().await else { return Ok(()) };

        if self.owner_only {
            let is_owner = match &ctx.command_input {
                AeonCommandInput::ApplicationCommand(input, _) => input.user.id == FLAZEPE_ID,
                AeonCommandInput::MessageCommand(message, _, _) => message.author.id.to_string() == FLAZEPE_ID,
            };

            if !is_owner {
                return ctx.respond_error("This command is owner-only.", true).await;
            }
        }

        if let Some(main) = &self.func {
            if let Err(error) = main.call(ctx).await {
                println!("{error:?}");
            }

            return Ok(());
        }

        match &mut ctx.command_input {
            AeonCommandInput::MessageCommand(_, args, _) => {
                let (subcommand, new_args) = args.split_once(' ').unwrap_or((args, ""));
                let subcommand = self
                    .subcommands
                    .iter()
                    .find(|entry| entry.name == subcommand.to_lowercase() || entry.aliases.contains(&subcommand.to_lowercase()));

                if let Some(subcommand) = subcommand {
                    *args = new_args.to_string();

                    if let Err(error) = subcommand.func.call(ctx).await {
                        println!("{error:?}");
                    }
                }
            },
            AeonCommandInput::ApplicationCommand(input, _) => {
                let subcommand = self
                    .subcommands
                    .iter()
                    .find(|entry| entry.name == input.subcommand.as_ref().or(input.custom_id.as_ref()).cloned().unwrap_or_default());

                if let Some(subcommand) = subcommand {
                    if let Err(error) = subcommand.func.call(ctx).await {
                        println!("{error:?}");
                    }
                }
            },
        }

        Ok(())
    }
}
