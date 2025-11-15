mod calculate;
mod code;
mod convert_currency;
mod define;
mod edited;
mod eval;
mod google;
mod google_translate_context;
mod heliohost;
mod inspect_message;
mod ip;
mod latex;
mod osu;
mod owner;
mod purge;
mod reminder;
mod reminder_set_context;
mod server_config;
mod shorten_url;
mod snipe;
mod snowflake;
mod tag;
mod timeout;
mod unicode;
mod unicode_list_context;
mod user;

use crate::structs::command::AeonCommand;
use slashook::commands::Command as SlashookCommand;
use std::sync::LazyLock;

pub fn get_aeon_commands() -> Vec<&'static LazyLock<AeonCommand>> {
    vec![
        &calculate::COMMAND,
        &convert_currency::COMMAND,
        &code::COMMAND,
        &define::COMMAND,
        &edited::COMMAND,
        &eval::COMMAND,
        &google::COMMAND,
        &google_translate_context::COMMAND,
        &heliohost::COMMAND,
        &inspect_message::COMMAND,
        &ip::COMMAND,
        &latex::COMMAND,
        &osu::COMMAND,
        &owner::COMMAND,
        &purge::COMMAND,
        &reminder::COMMAND,
        &reminder_set_context::COMMAND,
        &server_config::COMMAND,
        &shorten_url::COMMAND,
        &snipe::COMMAND,
        &snowflake::COMMAND,
        &tag::COMMAND,
        &timeout::COMMAND,
        &unicode::COMMAND,
        &unicode_list_context::COMMAND,
        &user::COMMAND,
    ]
}

pub fn get_slashook_commands() -> Vec<SlashookCommand> {
    vec![
        calculate::get_slashook_command(),
        convert_currency::get_slashook_command(),
        code::get_slashook_command(),
        define::get_slashook_command(),
        edited::get_slashook_command(),
        eval::get_slashook_command(),
        google::get_slashook_command(),
        google_translate_context::get_slashook_command(),
        heliohost::get_slashook_command(),
        inspect_message::get_slashook_command(),
        ip::get_slashook_command(),
        latex::get_slashook_command(),
        osu::get_slashook_command(),
        owner::get_slashook_command(),
        purge::get_slashook_command(),
        reminder::get_slashook_command(),
        reminder_set_context::get_slashook_command(),
        server_config::get_slashook_command(),
        shorten_url::get_slashook_command(),
        snipe::get_slashook_command(),
        snowflake::get_slashook_command(),
        tag::get_slashook_command(),
        timeout::get_slashook_command(),
        unicode::get_slashook_command(),
        unicode_list_context::get_slashook_command(),
        user::get_slashook_command(),
    ]
}
