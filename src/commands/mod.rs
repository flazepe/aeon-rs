pub mod search;
pub mod utility;

use crate::structs::{command::Command, command_context::Input};
use anyhow::Result;
use slashook::commands::Command as SlashookCommand;
use std::{fmt::Display, sync::LazyLock};
use twilight_gateway::MessageSender;
use twilight_model::channel::Message;

pub static COMMANDS: LazyLock<Vec<&'static LazyLock<Command>>> = LazyLock::new(|| {
    let mut commands = vec![];

    commands.append(&mut search::get_commands());
    commands.append(&mut utility::get_commands());

    commands
});

pub fn get_slashook_commands() -> Vec<SlashookCommand> {
    let mut commands = vec![];

    commands.append(&mut search::get_slashook_commands());
    commands.append(&mut utility::get_slashook_commands());

    commands
}

pub async fn run<T: Display, U: Display>(message: &Message, sender: &MessageSender, command_name: T, args: U) -> Result<()> {
    let command_name = command_name.to_string();

    let command = COMMANDS.iter().find(|command| command.name == command_name || command.aliases.contains(&command_name));

    if let Some(command) = command {
        return command.run(Input::MessageCommand { message: message.clone(), sender: sender.clone(), args: args.to_string() }).await;
    }

    Ok(())
}
