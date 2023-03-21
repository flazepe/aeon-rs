pub mod convert_currency;
pub mod dns;
pub mod ip;
pub mod sauce;
pub mod snipe;
pub mod translate;
pub mod translate_message;
pub mod unicode;
pub mod unicode_message;

use slashook::commands::Command;

pub fn get_commands() -> Vec<Command> {
    vec![
        convert_currency::get_command(),
        dns::get_command(),
        ip::get_command(),
        sauce::get_command(),
        snipe::get_command(),
        translate::get_command(),
        translate_message::get_command(),
        unicode::get_command(),
        unicode_message::get_command(),
    ]
}
