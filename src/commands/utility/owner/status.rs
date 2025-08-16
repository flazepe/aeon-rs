use crate::{
    functions::{format_timestamp, label_num},
    statics::{CACHE, colors::PRIMARY_EMBED_COLOR},
    structs::command_context::AeonCommandContext,
};
use anyhow::{Context, Error, Result};
use slashook::structs::embeds::Embed;
use std::{fmt::Display, sync::Arc};
use sysinfo::{System, get_current_pid};

pub async fn run(ctx: Arc<AeonCommandContext>) -> Result<()> {
    let system = System::new_all();
    let pid = get_current_pid().map_err(Error::msg)?;
    let process = system.process(pid).context("Could not get process.")?;
    let process_started = format_timestamp(process.start_time(), true);
    let memory = bytes_to_mb(process.memory());
    let virtual_memory = bytes_to_mb(process.virtual_memory());
    let discord_cache_list = get_discord_cache_list().join("\n");
    let db_cache_list = get_db_cache_list().join("\n");
    let other_cache_list = get_other_cache_list().join("\n");
    let embed = Embed::new()
        .set_color(PRIMARY_EMBED_COLOR)?
        .add_field("Process Started", process_started, false)
        .add_field("Memory", memory, false)
        .add_field("Virtual Memory", virtual_memory, false)
        .add_field("Discord Cache", discord_cache_list, false)
        .add_field("Database Cache", db_cache_list, false)
        .add_field("Other Cache", other_cache_list, false);

    ctx.respond(embed, false).await
}

fn bytes_to_mb(bytes: u64) -> String {
    format!("{} MB", bytes / 1024 / 1024)
}

fn sum_cache_len<T: Iterator<Item = (U, V)>, U: Display, V: IntoIterator<Item = W>, W: Clone>(iterable: T) -> usize {
    iterable.map(|(_, vec)| vec.into_iter().count()).reduce(|acc, cur| acc + cur).unwrap_or(0)
}

fn get_discord_cache_list() -> [String; 9] {
    [
        label_num(CACHE.discord.guilds.read().unwrap().len(), "server", "servers"),
        label_num(CACHE.discord.channels.read().unwrap().len(), "channel", "channels"),
        label_num(sum_cache_len(CACHE.discord.channels.read().unwrap().iter()), "message", "messages"),
        label_num(sum_cache_len(CACHE.discord.snipes.read().unwrap().iter()), "snipe", "snipes"),
        label_num(sum_cache_len(CACHE.discord.edit_snipes.read().unwrap().iter()), "edit snipe", "edit snipes"),
        label_num(sum_cache_len(CACHE.discord.reaction_snipes.read().unwrap().iter()), "reaction snipe", "reaction snipes"),
        label_num(CACHE.discord.song_activities.read().unwrap().len(), "Spotify activity", "Spotify activities"),
        label_num(CACHE.discord.command_responses.read().unwrap().len(), "command response", "command responses"),
        label_num(CACHE.discord.embed_fix_responses.read().unwrap().len(), "embed fix response", "embed fix responses"),
    ]
}

fn get_db_cache_list() -> [String; 1] {
    [label_num(CACHE.db.guilds.read().unwrap().len(), "server", "servers")]
}

fn get_other_cache_list() -> [String; 5] {
    [
        label_num(CACHE.cooldowns.read().unwrap().len(), "cooldown", "cooldowns"),
        label_num(
            CACHE.last_piston_programming_languages.read().unwrap().len(),
            "last piston programming language",
            "last piston programming languages",
        ),
        label_num(
            CACHE.last_tio_programming_languages.read().unwrap().len(),
            "last tio programming language",
            "last tio programming languages",
        ),
        label_num(CACHE.ordr_renders.read().unwrap().len(), "o!rdr render", "o!rdr renders"),
        label_num(CACHE.ordr_rendering_users.read().unwrap().len(), "o!rdr rendering user", "o!rdr rendering users"),
    ]
}
