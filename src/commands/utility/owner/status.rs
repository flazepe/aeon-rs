use crate::{
    functions::{TimestampFormat, format_timestamp, label_num},
    statics::{CACHE, colors::PRIMARY_COLOR},
    structs::command_context::CommandContext,
};
use anyhow::{Context, Result};
use slashook::structs::embeds::Embed;
use std::fmt::Display;
use sysinfo::{System, get_current_pid};

pub async fn run(ctx: CommandContext) -> Result<()> {
    match get_current_pid() {
        Ok(pid) => {
            let system = System::new_all();
            let process = system.process(pid).context("Could not get process.")?;
            let process_started = format_timestamp(process.start_time(), TimestampFormat::Full);
            let memory = bytes_to_mb(process.memory());
            let virtual_memory = bytes_to_mb(process.virtual_memory());
            let cache = get_cache_list().join("\n");
            let embed = Embed::new()
                .set_color(PRIMARY_COLOR)?
                .add_field("Process Started", process_started, false)
                .add_field("Memory", memory, false)
                .add_field("Virtual Memory", virtual_memory, false)
                .add_field("Cache", cache, false);

            ctx.respond(embed, false).await
        },
        Err(error) => ctx.respond_error(error, true).await,
    }
}

fn bytes_to_mb(bytes: u64) -> String {
    format!("{} MB", bytes / 1024 / 1024)
}

fn sum_cache_len<T: Iterator<Item = (U, V)>, U: Display, V: IntoIterator<Item = W>, W: Clone>(iterable: T) -> usize {
    iterable.map(|(_, vec)| vec.into_iter().count()).reduce(|acc, cur| acc + cur).unwrap_or(0)
}

fn get_cache_list() -> [String; 7] {
    [
        label_num(CACHE.guilds.read().unwrap().len(), "server", "servers"),
        label_num(CACHE.channels.read().unwrap().len(), "channel", "channels"),
        label_num(sum_cache_len(CACHE.channels.read().unwrap().iter()), "message", "messages"),
        label_num(sum_cache_len(CACHE.snipes.read().unwrap().iter()), "snipe", "snipes"),
        label_num(sum_cache_len(CACHE.edit_snipes.read().unwrap().iter()), "edit snipe", "edit snipes"),
        label_num(sum_cache_len(CACHE.reaction_snipes.read().unwrap().iter()), "reaction snipe", "reaction snipes"),
        label_num(CACHE.song_activities.read().unwrap().len(), "Spotify activity", "Spotify activities"),
    ]
}
