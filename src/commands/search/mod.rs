mod anilist;
mod distrowatch;
mod jisho;
mod lyricfind;
mod lyrics;
mod novel_updates;
mod spotify;
mod steam;
mod stock;
mod time;
mod ufret;
mod vndb;
mod youtube;

use slashook::commands::Command as SlashookCommand;

pub fn get_commands() -> Vec<SlashookCommand> {
    vec![
        anilist::get_command(),
        distrowatch::get_command(),
        jisho::get_command(),
        lyricfind::get_command(),
        lyrics::get_command(),
        novel_updates::get_command(),
        spotify::get_command(),
        steam::get_command(),
        stock::get_command(),
        time::get_command(),
        ufret::get_command(),
        vndb::get_command(),
        youtube::get_command(),
    ]
}
