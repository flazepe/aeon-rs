mod gaming;
mod general;
mod search;
mod utils;

use slashook::commands::Command;

pub fn get_commands() -> Vec<Command> {
    let mut commands = vec![];

    commands.append(&mut gaming::get_commands());
    commands.append(&mut general::get_commands());
    commands.append(&mut search::get_commands());
    commands.append(&mut utils::get_commands());

    commands
}
