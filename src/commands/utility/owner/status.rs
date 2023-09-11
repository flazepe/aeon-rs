use crate::{
    functions::{format_timestamp, plural, TimestampFormat},
    statics::{colors::PRIMARY_COLOR, CACHE},
    structs::command_context::CommandContext,
};
use anyhow::{Context, Result};
use slashook::structs::embeds::Embed;
use std::collections::hash_map::Iter;
use sysinfo::{get_current_pid, ProcessExt, System, SystemExt};

pub async fn run(ctx: CommandContext) -> Result<()> {
    match get_current_pid() {
        Ok(pid) => {
            let mut system = System::new();
            system.refresh_process(pid);

            let process = system.process(pid).context("Could not get process.")?;

            ctx.respond(
                Embed::new()
                    .set_color(PRIMARY_COLOR)?
                    .add_field("Process Started", format_timestamp(process.start_time(), TimestampFormat::Full), false)
                    .add_field("Memory", bytes_to_mb(process.memory()), false)
                    .add_field("Virtual Memory", bytes_to_mb(process.virtual_memory()), false)
                    .add_field("Cache", get_cache_list().join("\n"), false),
                false,
            )
            .await
        },
        Err(error) => ctx.respond_error(error, true).await,
    }
}

fn bytes_to_mb(bytes: u64) -> String {
    format!("{} MB", bytes / 1024 / 1024)
}

fn sum_cache_len<T: Clone>(iter: Iter<String, Vec<T>>) -> usize {
    iter.map(|(_, vec)| vec.len()).reduce(|acc, cur| acc + cur).unwrap_or(0)
}

fn get_cache_list() -> [String; 6] {
    [
        plural(CACHE.channels.read().unwrap().len(), "channel"),
        plural(sum_cache_len(CACHE.channels.read().unwrap().iter()), "message"),
        plural(sum_cache_len(CACHE.snipes.read().unwrap().iter()), "snipe"),
        plural(sum_cache_len(CACHE.edit_snipes.read().unwrap().iter()), "edit snipe"),
        plural(sum_cache_len(CACHE.reaction_snipes.read().unwrap().iter()), "reaction snipe"),
        plural(CACHE.spotify.read().unwrap().len(), "Spotify activity"),
    ]
}
