use crate::{
    statics::{
        regex::{OG_IMAGE_TAG_REGEX, URL_REGEX},
        CACHE, REQWEST, REST,
    },
    structs::{database::guilds::Guilds, gateway::events::handler::EventHandler},
    traits::LimitedVec,
};
use anyhow::Result;
use reqwest::StatusCode;
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

            if guild.fix_embeds && !message.author.bot {
                Self::fix_embed(message.clone()).await?;
            }
        }

        // Handle owner commands
        Self::handle_owner(message, sender).await?;

        Ok(())
    }

    async fn fix_embed(message: Message) -> Result<()> {
        let urls = URL_REGEX.find_iter(&message.content).map(|entry| entry.as_str()).collect::<Vec<&str>>();
        let mut new_urls = vec![];

        for url in urls {
            // Skip suppressed embeds
            if message.content.contains(&format!("<{url}>")) {
                continue;
            }

            let Some(domain) = url.split('/').nth(2).map(|domain| domain.trim_start_matches("www.")) else { continue };

            // Skip Bluesky/X posts that have a valid image
            if ["bsky.app", "x.com", "twitter.com"].contains(&domain) {
                let body = REQWEST
                    .get(url)
                    .header("user-agent", "Mozilla/5.0 (compatible; Discordbot/2.0; +https://discordapp.com)")
                    .send()
                    .await?
                    .text()
                    .await?;

                let image_url = OG_IMAGE_TAG_REGEX
                    .find(&body)
                    .and_then(|og_image_tag| og_image_tag.as_str().split('"').find(|entry| entry.contains("https")))
                    .unwrap_or_default();

                if REQWEST.get(image_url).send().await.map_or(false, |res| res.status() == StatusCode::OK) {
                    continue;
                }
            }

            let new_domain = match domain {
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
            let new_url = format!("https://{new_domain}/{path}");
            let body = REQWEST
                .get(&new_url)
                .header("user-agent", "Mozilla/5.0 (compatible; Discordbot/2.0; +https://discordapp.com)")
                .send()
                .await?
                .text()
                .await?;

            // Only fix posts that were supposed to have an image or video
            if !["og:image", "og:video", "twitter:image", "twitter:video"].iter().any(|entry| body.contains(entry)) {
                continue;
            }

            new_urls.push(new_url);
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
