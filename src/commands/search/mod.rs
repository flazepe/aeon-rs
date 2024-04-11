use crate::macros::group;
use slashook::commands::Command as SlashookCommand;

pub fn get_commands() -> Vec<SlashookCommand> {
    group! {
        anilist,
        distrowatch,
        jisho,
        lyrics,
        novel_updates,
        spotify,
        steam,
        stock,
        time,
        vndb,
        youtube,
    }
}
