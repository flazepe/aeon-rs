use crate::{functions::label_num, statics::colors::ERROR_EMBED_COLOR, structs::database::Database};
use anyhow::Result;
use slashook::{
    chrono::Utc,
    commands::MessageResponse,
    structs::{embeds::Embed, utils::File},
};
use twilight_model::gateway::payload::incoming::MessageDeleteBulk;

pub async fn handle(event: &MessageDeleteBulk) -> Result<()> {
    let Some(guild_id) = event.guild_id else { return Ok(()) };

    let embed = Embed::new()
        .set_color(ERROR_EMBED_COLOR)
        .unwrap_or_default()
        .set_title(format!("{} Deleted", label_num(event.ids.len(), "Message", "Messages")))
        .add_field("Channel", format!("<#{channel_id}> ({channel_id})", channel_id = event.channel_id), false)
        .set_timestamp(Utc::now());

    let file = File::new(
        "messages.txt",
        event
            .ids
            .iter()
            .map(|id| format!("https://discord.com/channels/{guild_id}/{channel_id}/{id}", channel_id = event.channel_id))
            .collect::<Vec<String>>()
            .join("\n"),
    );

    let response: MessageResponse = MessageResponse::from(embed).add_file(file);

    let mongodb = Database::get_mongodb()?;
    mongodb.guilds.send_log(guild_id, response, false).await
}
