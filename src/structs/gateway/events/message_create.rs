use crate::{
    statics::{regex::URL_REGEX, CACHE, REST},
    structs::{database::guilds::Guilds, gateway::events::handler::EventHandler},
    traits::LimitedVec,
};
use anyhow::Result;
use serde_json::json;
use slashook::structs::messages::MessageFlags;
use twilight_model::{channel::Message, gateway::payload::incoming::MessageCreate};

impl EventHandler {
    pub async fn on_message_create(message: Box<MessageCreate>) -> Result<()> {
        let message = message.0;

        {
            let mut channels = CACHE.channels.write().unwrap();
            let channel_id = message.channel_id.to_string();

            if !channels.contains_key(&channel_id) {
                channels.insert(channel_id.clone(), vec![]);
            }

            channels.get_mut(&channel_id).unwrap().push_limited(message.clone(), 50);
        }

        // Fix embeds
        if let Some(guild_id) = message.guild_id {
            let guild = Guilds::get(guild_id).await?;

            if guild.fix_embeds {
                Self::fix_embed(message).await?;
            }
        }

        Ok(())
    }

    pub async fn fix_embed(message: Message) -> Result<()> {
        let mut new_urls = vec![];

        for url in URL_REGEX.find_iter(&message.content) {
            let Some(domain) = url.as_str().split('/').nth(2) else { continue };
            let new_domain = match domain.trim_start_matches("www.") {
                "instagram.com" => "ddinstagram.com",
                "twitter.com" | "x.com" => "fixupx.com",
                "pixiv.net" => "phixiv.net",
                "reddit.com" | "old.reddit.com" => "rxddit.com",
                _ => continue,
            };
            let path = url.as_str().split('/').skip(3).map(|str| str.to_string()).collect::<Vec<String>>().join("/");

            if !path.is_empty() {
                new_urls.push(format!("https://{new_domain}/{path}"));
            }
        }

        if !new_urls.is_empty() {
            let _ = REST
                .patch::<Message, _>(
                    format!("channels/{}/messages/{}", message.channel_id, message.id),
                    json!({ "flags": MessageFlags::SUPPRESS_EMBEDS }),
                )
                .await;

            let _ = REST
                .post::<Message, _>(
                    format!("channels/{}/messages", message.channel_id),
                    json!({
                        "content": new_urls.join("\n"),
                        "message_reference": { "message_id": message.id.to_string() },
                        "allowed_mentions": { "replied_user": false },
                    }),
                )
                .await;
        }

        Ok(())
    }
}
