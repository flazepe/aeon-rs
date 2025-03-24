use crate::{
    statics::FLAZEPE_ID,
    structs::command_context::{CommandContext, Input},
};
use anyhow::Result;
use futures::{Future, future::BoxFuture};
use std::fmt::Display;

pub trait CommandFn: Send + Sync {
    fn call(&self, ctx: CommandContext) -> BoxFuture<'static, Result<()>>;
}

impl<T: Fn(CommandContext) -> U + Send + Sync, U: Future<Output = Result<()>> + Send + 'static> CommandFn for T {
    fn call(&self, ctx: CommandContext) -> BoxFuture<'static, Result<()>> {
        Box::pin(self(ctx))
    }
}

pub struct Command {
    pub name: String,
    pub aliases: Vec<String>,
    pub owner_only: bool,
    func: Option<Box<dyn CommandFn>>,
    subcommands: Vec<Subcommand>,
}

pub struct Subcommand {
    pub name: String,
    pub aliases: Vec<String>,
    func: Box<dyn CommandFn>,
}

impl Command {
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

    pub fn main<T: CommandFn + 'static>(mut self, func: T) -> Self {
        self.func = Some(Box::new(func));
        self
    }

    pub fn subcommand<T: Display, U: CommandFn + 'static>(mut self, name: T, aliases: &[&str], func: U) -> Self {
        self.subcommands.push(Subcommand {
            name: name.to_string(),
            aliases: aliases.iter().map(|alias| alias.to_string()).collect(),
            func: Box::new(func),
        });
        self
    }

    pub async fn run(&self, input: Input) -> Result<()> {
        let Ok(mut ctx) = CommandContext::new(input).verify().await else {
            return Ok(());
        };

        if self.owner_only {
            let is_owner = match &ctx.input {
                Input::ApplicationCommand { input, res: _ } => input.user.id == FLAZEPE_ID,
                Input::MessageCommand { message, sender: _, args: _ } => message.author.id.to_string() == FLAZEPE_ID,
            };

            if !is_owner {
                return ctx.respond_error("This command is owner-only.", true).await;
            }
        }

        if let Some(main) = &self.func {
            if let Err(error) = main.call(ctx).await {
                println!("{error}");
            }

            return Ok(());
        }

        match &mut ctx.input {
            Input::MessageCommand { message: _, sender: _, args } => {
                let (subcommand, new_args) = args.split_once(' ').unwrap_or((args, ""));
                let subcommand = self
                    .subcommands
                    .iter()
                    .find(|entry| entry.name == subcommand.to_lowercase() || entry.aliases.contains(&subcommand.to_lowercase()));

                if let Some(subcommand) = subcommand {
                    *args = new_args.to_string();

                    if let Err(error) = subcommand.func.call(ctx).await {
                        println!("{error}");
                    }
                }
            },
            Input::ApplicationCommand { input, res: _ } => {
                let subcommand = self
                    .subcommands
                    .iter()
                    .find(|entry| entry.name == input.subcommand.as_ref().or(input.custom_id.as_ref()).cloned().unwrap_or_default());

                if let Some(subcommand) = subcommand {
                    if let Err(error) = subcommand.func.call(ctx).await {
                        println!("{error}");
                    }
                }
            },
        }

        Ok(())
    }
}
