mod distro;
mod stock;
mod time;

use slashook::commands::Command;

pub fn get_commands() -> Vec<Command> {
    vec![distro::get_command(), stock::get_command(), time::get_command()]
}
