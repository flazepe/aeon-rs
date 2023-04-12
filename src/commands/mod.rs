mod general;
mod search;
mod utility;

use slashook::commands::Command;

pub fn get_commands() -> Vec<Command> {
    let mut commands = vec![];

    commands.append(&mut general::get_commands());
    commands.append(&mut search::get_commands());
    commands.append(&mut utility::get_commands());

    commands
}
