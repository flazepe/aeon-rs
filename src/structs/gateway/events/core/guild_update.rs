use crate::statics::CACHE;
use anyhow::Result;
use twilight_model::gateway::payload::incoming::GuildUpdate;

pub async fn handle(event: &GuildUpdate) -> Result<()> {
    let Some(mut new_guild) = CACHE.discord_guilds.read().unwrap().get(&event.id.to_string()).cloned() else { return Ok(()) };
    let guild = event.0.clone();

    new_guild.afk_channel_id = guild.afk_channel_id;
    new_guild.afk_timeout = guild.afk_timeout;
    new_guild.application_id = guild.application_id;
    new_guild.banner = guild.banner;
    new_guild.default_message_notifications = guild.default_message_notifications;
    new_guild.description = guild.description;
    new_guild.discovery_splash = guild.discovery_splash;
    new_guild.emojis = guild.emojis;
    new_guild.explicit_content_filter = guild.explicit_content_filter;
    new_guild.features = guild.features;
    new_guild.icon = guild.icon;
    new_guild.id = guild.id;
    new_guild.max_members = guild.max_members;
    new_guild.max_presences = guild.max_presences;
    new_guild.member_count = guild.member_count;
    new_guild.mfa_level = guild.mfa_level;
    new_guild.name = guild.name;
    new_guild.nsfw_level = guild.nsfw_level;
    new_guild.owner = guild.owner;
    new_guild.owner_id = guild.owner_id;
    new_guild.permissions = guild.permissions;
    new_guild.preferred_locale = guild.preferred_locale;
    new_guild.premium_progress_bar_enabled = guild.premium_progress_bar_enabled;
    new_guild.premium_subscription_count = guild.premium_subscription_count;
    new_guild.premium_tier = guild.premium_tier;
    new_guild.public_updates_channel_id = guild.public_updates_channel_id;
    new_guild.roles = guild.roles;
    new_guild.rules_channel_id = guild.rules_channel_id;
    new_guild.splash = guild.splash;
    new_guild.system_channel_flags = guild.system_channel_flags;
    new_guild.system_channel_id = guild.system_channel_id;
    new_guild.vanity_url_code = guild.vanity_url_code;
    new_guild.verification_level = guild.verification_level;
    new_guild.widget_channel_id = guild.widget_channel_id;
    new_guild.widget_enabled = guild.widget_enabled;

    CACHE.discord_guilds.write().unwrap().insert(event.id.to_string(), new_guild);

    Ok(())
}
