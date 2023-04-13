mod anilist;
mod distro;
mod jisho;
mod spotify;
mod steam;
mod stock;
mod time;
mod vndb;

use slashook::commands::Command;

pub fn get_commands() -> Vec<Command> {
    vec![
        anilist::get_command(),
        distro::get_command(),
        jisho::get_command(),
        spotify::get_command(),
        steam::get_command(),
        stock::get_command(),
        time::get_command(),
        vndb::get_command(),
    ]
}
