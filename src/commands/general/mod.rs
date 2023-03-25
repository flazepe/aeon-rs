pub mod heliohost;
pub mod source;
pub mod status;

use slashook::commands::Command;

pub fn get_commands() -> Vec<Command> {
    vec![
        heliohost::get_command(),
        source::get_command(),
        status::get_command(),
    ]
}
