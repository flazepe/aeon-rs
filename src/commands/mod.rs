use slashook::commands::Command;

pub mod gaming;
pub mod general;
pub mod utils;

pub fn get_commands() -> Vec<Command> {
    let mut commands = vec![];

    commands.append(&mut gaming::get_commands());
    commands.append(&mut general::get_commands());
    commands.append(&mut utils::get_commands());

    commands
}
