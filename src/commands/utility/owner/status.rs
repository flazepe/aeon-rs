use crate::{
    functions::{format_timestamp, label_num},
    statics::{CACHE, REDIS, colors::PRIMARY_EMBED_COLOR},
    structs::{command_context::AeonCommandContext, database::redis::keys::RedisKey},
};
use anyhow::{Context, Error, Result};
use slashook::structs::embeds::Embed;
use std::sync::Arc;
use sysinfo::{System, get_current_pid};

pub async fn run(ctx: Arc<AeonCommandContext>) -> Result<()> {
    ctx.defer(false).await?;

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

fn get_discord_cache_list() -> [String; 2] {
    [
        label_num(CACHE.discord.guilds.read().unwrap().len(), "server", "servers"),
        label_num(CACHE.discord.song_activities.read().unwrap().len(), "Spotify activity", "Spotify activities"),
    ]
}

fn get_db_cache_list() -> [String; 1] {
    [label_num(CACHE.db.guilds.read().unwrap().len(), "server", "servers")]
}

async fn get_redis_cache_list() -> Result<[String; 9]> {
    let redis = REDIS.get().unwrap();

    let messages = redis.scan_match(RedisKey::GuildChannelMessage("*".into(), "*".into(), "*[0-9]".into())).await?;
    let snipes = redis.scan_match(RedisKey::GuildChannelSnipes("*".into(), "*".into())).await?;
    let edit_snipes = redis.scan_match(RedisKey::GuildChannelEditSnipes("*".into(), "*".into())).await?;
    let reaction_snipes = redis.scan_match(RedisKey::GuildChannelMessageReactionSnipes("*".into(), "*".into(), "*".into())).await?;
    let cooldowns = redis.scan_match(RedisKey::UserCooldown("*".into())).await?;
    let command_responses = redis.scan_match(RedisKey::GuildChannelMessageCommandResponse("*".into(), "*".into(), "*".into())).await?;
    let embed_fix_responses = redis.scan_match(RedisKey::GuildChannelMessageEmbedFixResponse("*".into(), "*".into(), "*".into())).await?;
    let last_piston_programming_languages = redis.scan_match(RedisKey::UserLastPistonProgrammingLanguage("*".into())).await?;
    let total_keys = redis.scan_match("*").await?;

    Ok([
        label_num(messages, "message", "messages"),
        label_num(snipes, "snipe hash", "snipe hashes"),
        label_num(edit_snipes, "edit snipe hash", "edit snipe hashes"),
        label_num(reaction_snipes, "reaction snipe hash", "reaction snipe hashes"),
        label_num(cooldowns, "cooldown", "cooldowns"),
        label_num(command_responses, "command response", "command responses"),
        label_num(embed_fix_responses, "embed fix response", "embed fix responses"),
        label_num(last_piston_programming_languages, "last piston programming language", "last piston programming languages"),
        label_num(total_keys, "total key", "total keys"),
    ])
}

fn get_other_cache_list() -> [String; 1] {
    [label_num(CACHE.ordr_rendering_users.read().unwrap().len(), "o!rdr rendering user", "o!rdr rendering users")]
}
