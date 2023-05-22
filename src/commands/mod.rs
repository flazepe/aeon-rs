mod search;
mod utility;

use slashook::commands::Command as SlashookCommand;

pub fn get_commands() -> Vec<SlashookCommand> {
    let mut commands = vec![];

    commands.append(&mut search::get_commands());
    commands.append(&mut utility::get_commands());

    commands
}
