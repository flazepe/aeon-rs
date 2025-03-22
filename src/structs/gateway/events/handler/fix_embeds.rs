use crate::{
    statics::{
        regex::{OG_IMAGE_TAG_REGEX, URL_REGEX},
        REQWEST, REST,
    },
    structs::{database::guilds::Guilds, gateway::events::handler::EventHandler},
};
use anyhow::Result;
use reqwest::StatusCode;
use serde_json::json;
use slashook::structs::messages::{Message, MessageFlags};
use twilight_model::channel::Message as TwilightMessage;

impl EventHandler {
    pub async fn handle_fix_embeds(message: &TwilightMessage) -> Result<()> {
        let Some(guild_id) = &message.guild_id else { return Ok(()) };
        let guild = Guilds::get(guild_id).await?;

        if !guild.fix_embeds || message.author.bot {
            return Ok(());
        }

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
