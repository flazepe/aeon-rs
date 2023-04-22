mod code;
mod convert_currency;
mod google;
mod google_translate_message;
mod heliohost;
mod ip;
mod remind;
mod remind_message;
mod sauce;
mod snipe;
mod calculate;
mod snipe_message_reactions;
mod tag;
mod timeout;
mod unicode;
mod unicode_message;

use slashook::commands::Command;

pub fn get_commands() -> Vec<Command> {
    vec![
        calculate::get_command(),
        convert_currency::get_command(),
        code::get_command(),
        google::get_command(),
        google_translate_message::get_command(),
        heliohost::get_command(),
        ip::get_command(),
        remind::get_command(),
        remind_message::get_command(),
        sauce::get_command(),
        snipe::get_command(),
        snipe_message_reactions::get_command(),
        tag::get_command(),
        timeout::get_command(),
        unicode::get_command(),
        unicode_message::get_command(),
    ]
}
