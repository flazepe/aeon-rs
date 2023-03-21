pub mod steam;

use slashook::commands::Command;

pub fn get_commands() -> Vec<Command> {
    vec![steam::get_command()]
}
