use crate::{
    functions::{format_timestamp, label_num},
    statics::{CACHE, REDIS, colors::PRIMARY_EMBED_COLOR},
    structs::command_context::AeonCommandContext,
};
use anyhow::{Context, Error, Result};
use slashook::structs::embeds::Embed;
use std::sync::Arc;
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
    let redis_cache_list = get_redis_cache_list().await?.join("\n");
    let other_cache_list = get_other_cache_list().join("\n");
    let embed = Embed::new()
        .set_color(PRIMARY_EMBED_COLOR)?
        .add_field("Process Started", process_started, false)
        .add_field("Memory", memory, false)
        .add_field("Virtual Memory", virtual_memory, false)
        .add_field("Discord Cache", discord_cache_list, false)
        .add_field("Database Cache", db_cache_list, false)
        .add_field("Redis Cache", redis_cache_list, false)
        .add_field("Other Cache", other_cache_list, false);

    ctx.respond(embed, false).await
}

fn bytes_to_mb(bytes: u64) -> String {
    format!("{} MB", bytes / 1024 / 1024)
}

fn get_discord_cache_list() -> [String; 4] {
    [
        label_num(CACHE.discord.guilds.read().unwrap().len(), "server", "servers"),
        label_num(CACHE.discord.song_activities.read().unwrap().len(), "Spotify activity", "Spotify activities"),
        label_num(CACHE.discord.command_responses.read().unwrap().len(), "command response", "command responses"),
        label_num(CACHE.discord.embed_fix_responses.read().unwrap().len(), "embed fix response", "embed fix responses"),
    ]
}

fn get_db_cache_list() -> [String; 1] {
    [label_num(CACHE.db.guilds.read().unwrap().len(), "server", "servers")]
}

async fn get_redis_cache_list() -> Result<[String; 6]> {
    let redis = REDIS.get().unwrap();

    let messages = redis.scan_match("guilds_*_channels_*_messages_*[0-9]").await?;
    let snipes = redis.scan_match("guilds_*_channels_*_snipes").await?;
    let edit_snipes = redis.scan_match("guilds_*_channels_*_edit-snipes").await?;
    let reaction_snipes = redis.scan_match("guilds_*_channels_*_messages_*_reaction-snipes").await?;
    let cooldowns = redis.scan_match("users_*_cooldown").await?;
    let keys = redis.scan_match("*").await?;

    Ok([
        label_num(messages, "message", "messages"),
        label_num(snipes, "snipe hash", "snipe hashes"),
        label_num(edit_snipes, "edit snipe hash", "edit snipe hashes"),
        label_num(reaction_snipes, "reaction snipe hash", "reaction snipe hashes"),
        label_num(cooldowns, "cooldown", "cooldowns"),
        label_num(keys, "total key", "total keys"),
    ])
}

fn get_other_cache_list() -> [String; 2] {
    [
        label_num(
            CACHE.last_piston_programming_languages.read().unwrap().len(),
            "last piston programming language",
            "last piston programming languages",
        ),
        label_num(CACHE.ordr_rendering_users.read().unwrap().len(), "o!rdr rendering user", "o!rdr rendering users"),
    ]
}
