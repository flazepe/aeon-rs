mod anilist;
mod distrowatch;
mod jisho;
mod novel_updates;
mod spotify;
mod steam;
mod stock;
mod time;
mod vndb;
mod youtube;

use slashook::commands::Command as SlashookCommand;

pub fn get_commands() -> Vec<SlashookCommand> {
    vec![
        anilist::get_command(),
        distrowatch::get_command(),
        jisho::get_command(),
        novel_updates::get_command(),
        spotify::get_command(),
        steam::get_command(),
        stock::get_command(),
        time::get_command(),
        vndb::get_command(),
        youtube::get_command(),
    ]
}
