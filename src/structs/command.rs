use crate::{statics::FLAZEPE_ID, structs::command_context::CommandContext};
use anyhow::Result;
use futures::{future::BoxFuture, Future};
use slashook::commands::{CommandInput, CommandResponder};
use std::collections::HashMap;
use tokio::spawn;

pub trait AeonCommandFn: Send + Sync {
    fn call(&self, ctx: CommandContext) -> BoxFuture<'static, Result<()>>;
}

impl<T: Fn(CommandContext) -> U + Send + Sync, U: Future<Output = Result<()>> + Send + 'static> AeonCommandFn for T {
    fn call(&self, ctx: CommandContext) -> BoxFuture<'static, Result<()>> {
        Box::pin(self(ctx))
    }
}

pub struct AeonCommand {
    owner_only: bool,
    main: Option<Box<dyn AeonCommandFn>>,
    subcommands: HashMap<String, Box<dyn AeonCommandFn>>,
}

impl AeonCommand {
    pub fn new() -> Self {
        Self { owner_only: false, main: None, subcommands: HashMap::new() }
    }

    pub fn owner_only(mut self) -> Self {
        self.owner_only = true;
        self
    }

    pub fn main<T: AeonCommandFn + 'static>(mut self, func: T) -> Self {
        self.main = Some(Box::new(func));
        self
    }

    pub fn subcommand<T: ToString, U: AeonCommandFn + 'static>(mut self, name: T, func: U) -> Self {
        self.subcommands.insert(name.to_string(), Box::new(func));
        self
    }

    pub async fn run(&self, input: CommandInput, res: CommandResponder) -> Result<()> {
        let Ok(ctx) = CommandContext::new(input,res).verify().await else { return Ok(()); };

        if self.owner_only && ctx.input.user.id != FLAZEPE_ID {
            return ctx.respond_error("This command is owner-only.", true).await;
        }

        if let Some(main) = self.main.as_ref() {
            spawn(main.call(ctx));
            return Ok(());
        }

        if let Some(sub_command) =
            self.subcommands.get(&ctx.input.subcommand.as_deref().or(ctx.input.custom_id.as_deref()).unwrap_or("").to_string())
        {
            spawn(sub_command.call(ctx));
            return Ok(());
        };

        Ok(())
    }
}
