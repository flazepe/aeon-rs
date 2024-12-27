use crate::{statics::FLAZEPE_ID, structs::command_context::CommandContext};
use anyhow::Result;
use futures::{future::BoxFuture, Future};
use slashook::commands::{CommandInput, CommandResponder};
use std::{collections::HashMap, fmt::Display};

pub trait CommandFn: Send + Sync {
    fn call(&self, ctx: CommandContext) -> BoxFuture<'static, Result<()>>;
}

impl<T: Fn(CommandContext) -> U + Send + Sync, U: Future<Output = Result<()>> + Send + 'static> CommandFn for T {
    fn call(&self, ctx: CommandContext) -> BoxFuture<'static, Result<()>> {
        Box::pin(self(ctx))
    }
}

pub struct Command {
    owner_only: bool,
    main: Option<Box<dyn CommandFn>>,
    subcommands: HashMap<String, Box<dyn CommandFn>>,
}

impl Command {
    pub fn new() -> Self {
        Self { owner_only: false, main: None, subcommands: HashMap::new() }
    }

    pub fn owner_only(mut self) -> Self {
        self.owner_only = true;
        self
    }

    pub fn main<T: CommandFn + 'static>(mut self, func: T) -> Self {
        self.main = Some(Box::new(func));
        self
    }

    pub fn subcommand<T: Display, U: CommandFn + 'static>(mut self, name: T, func: U) -> Self {
        self.subcommands.insert(name.to_string(), Box::new(func));
        self
    }

    pub async fn run(&self, input: CommandInput, res: CommandResponder) -> Result<()> {
        let Ok(ctx) = CommandContext::new(input, res).verify().await else {
            return Ok(());
        };

        if self.owner_only && ctx.input.user.id != FLAZEPE_ID {
            return ctx.respond_error("This command is owner-only.", true).await;
        }

        if let Some(main) = &self.main {
            if let Err(error) = main.call(ctx).await {
                println!("{error}");
            }

            return Ok(());
        }

        if let Some(subcommand) = self.subcommands.get(ctx.input.subcommand.as_deref().or(ctx.input.custom_id.as_deref()).unwrap_or("")) {
            if let Err(error) = subcommand.call(ctx).await {
                println!("{error}");
            }
        };

        Ok(())
    }
}
