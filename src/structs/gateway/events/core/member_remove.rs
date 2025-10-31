use crate::statics::CACHE;
use anyhow::Result;
use twilight_model::gateway::payload::incoming::MemberRemove;

pub async fn handle(event: &MemberRemove) -> Result<()> {
    let mut guilds = CACHE.discord.guilds.write().unwrap();
    let Some(guild) = guilds.get_mut(&event.guild_id.to_string()) else { return Ok(()) };

    if let Some(member_count) = guild.member_count {
        guild.member_count = Some(member_count - 1);
    }

    Ok(())
}
