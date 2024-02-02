mod calculate;
mod code;
mod convert_currency;
mod google;
mod google_translate_context;
mod heliohost;
mod ip;
mod ocr;
mod osu;
mod owner;
mod reminder;
mod reminder_set_context;
mod sauce;
mod shorten_url;
mod snipe;
mod snipe_reaction_context;
mod tag;
mod timeout;
mod unicode;
mod unicode_context;
mod user;
mod voice_message;
mod voice_message_context;

use slashook::commands::Command as SlashookCommand;

pub fn get_commands() -> Vec<SlashookCommand> {
    vec![
        calculate::get_command(),
        convert_currency::get_command(),
        code::get_command(),
        google::get_command(),
        google_translate_context::get_command(),
        heliohost::get_command(),
        ip::get_command(),
        ocr::get_command(),
        osu::get_command(),
        owner::get_command(),
        reminder::get_command(),
        reminder_set_context::get_command(),
        sauce::get_command(),
        shorten_url::get_command(),
        snipe::get_command(),
        snipe_reaction_context::get_command(),
        tag::get_command(),
        timeout::get_command(),
        unicode::get_command(),
        unicode_context::get_command(),
        user::get_command(),
        voice_message::get_command(),
        voice_message_context::get_command(),
    ]
}
