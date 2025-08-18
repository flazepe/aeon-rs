mod anilist;
mod anison;
mod distrowatch;
mod help;
mod jisho;
mod lyrics;
mod spotify;
mod steam;
mod time;
mod vndb;
mod youtube;

use crate::structs::command::AeonCommand;
use slashook::commands::Command as SlashookCommand;
use std::sync::LazyLock;

pub fn get_aeon_commands() -> Vec<&'static LazyLock<AeonCommand>> {
    vec![
        &anilist::COMMAND,
        &anison::COMMAND,
        &distrowatch::COMMAND,
        &help::COMMAND,
        &jisho::COMMAND,
        &lyrics::COMMAND,
        &spotify::COMMAND,
        &steam::COMMAND,
        &time::COMMAND,
        &vndb::COMMAND,
        &youtube::COMMAND,
    ]
}

pub fn get_slashook_commands() -> Vec<SlashookCommand> {
    vec![
        anilist::get_slashook_command(),
        anison::get_slashook_command(),
        distrowatch::get_slashook_command(),
        help::get_slashook_command(),
        jisho::get_slashook_command(),
        lyrics::get_slashook_command(),
        spotify::get_slashook_command(),
        steam::get_slashook_command(),
        time::get_slashook_command(),
        vndb::get_slashook_command(),
        youtube::get_slashook_command(),
    ]
}
