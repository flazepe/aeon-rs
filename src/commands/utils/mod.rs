mod code;
mod convert_currency;
mod dns;
mod heliohost;
mod ip;
mod remind;
mod sauce;
mod snipe;
mod snipe_message_reactions;
mod timeout;
mod translate;
mod translate_message;
mod unicode;
mod unicode_message;

use slashook::commands::Command;

pub fn get_commands() -> Vec<Command> {
    vec![
        convert_currency::get_command(),
        code::get_command(),
        dns::get_command(),
        heliohost::get_command(),
        ip::get_command(),
        remind::get_command(),
        sauce::get_command(),
        snipe::get_command(),
        snipe_message_reactions::get_command(),
        timeout::get_command(),
        translate::get_command(),
        translate_message::get_command(),
        unicode::get_command(),
        unicode_message::get_command(),
    ]
}
