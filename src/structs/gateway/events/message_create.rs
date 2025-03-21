use crate::{
    statics::{regex::URL_REGEX, CACHE, REST},
    structs::{database::guilds::Guilds, gateway::events::handler::EventHandler},
    traits::LimitedVec,
};
use anyhow::Result;
use serde_json::json;
use slashook::structs::messages::MessageFlags;
use twilight_gateway::MessageSender;
use twilight_model::{channel::Message, gateway::payload::incoming::MessageCreate};

impl EventHandler {
    pub async fn on_message_create(message: Box<MessageCreate>, sender: MessageSender) -> Result<()> {
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
        if let Some(guild_id) = &message.guild_id {
            let guild = Guilds::get(guild_id).await?;

            if !message.author.bot && guild.fix_embeds {
                Self::fix_embed(message.clone()).await?;
            }
        }

        // Handle owner commands
        Self::handle_owner(message, sender).await?;

        Ok(())
    }

    async fn fix_embed(message: Message) -> Result<()> {
        let urls = URL_REGEX.find_iter(&message.content).map(|entry| entry.as_str()).collect::<Vec<&str>>();
        let valid_x_embeds = message.embeds.iter().filter(|embed| {
            let is_x = embed.footer.as_ref().map_or(false, |footer| footer.text == "X");
            let has_image = embed.image.as_ref().and_then(|image| image.width).map_or(false, |width| width > 0);
            is_x && has_image
        });

        if urls.len() == valid_x_embeds.count() {
            return Ok(());
        }

        let mut new_urls = vec![];

        for url in urls {
            let Some(domain) = url.split('/').nth(2) else { continue };
            let new_domain = match domain.trim_start_matches("www.") {
                "bsky.app" => "fxbsky.app",
                "instagram.com" => "ddinstagram.com",
                "pixiv.net" => "phixiv.net",
                "reddit.com" | "old.reddit.com" => "rxddit.com",
                "tiktok.com" => "vxtiktok.com",
                "vt.tiktok.com" => "vt.vxtiktok.com",
                "x.com" | "twitter.com" => "fixupx.com",
                _ => continue,
            };
            let path = url.split('/').skip(3).map(|str| str.to_string()).collect::<Vec<String>>().join("/");

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
