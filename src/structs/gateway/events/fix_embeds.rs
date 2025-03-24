use crate::{
    statics::{
        REQWEST, REST,
        regex::{OG_IMAGE_TAG_REGEX, URL_REGEX},
    },
    structs::{database::guilds::Guilds, gateway::events::EventHandler},
};
use anyhow::Result;
use reqwest::StatusCode;
use serde_json::json;
use slashook::structs::messages::{Message, MessageFlags};
use twilight_model::gateway::payload::incoming::MessageCreate;

impl EventHandler {
    pub async fn handle_fix_embeds(event: &MessageCreate) -> Result<()> {
        let Some(guild_id) = &event.guild_id else { return Ok(()) };
        let guild = Guilds::get(guild_id).await?;

        if !guild.fix_embeds || event.author.bot {
            return Ok(());
        }

        let urls = URL_REGEX.find_iter(&event.content).map(|entry| entry.as_str()).collect::<Vec<&str>>();
        let mut new_urls = vec![];

        for url in urls {
            // Skip suppressed embeds
            if event.content.contains(&format!("<{url}>")) {
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

                if REQWEST.get(image_url).send().await.is_ok_and(|res| res.status() == StatusCode::OK) {
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
                    format!("channels/{}/messages/{}", event.channel_id, event.id),
                    json!({ "flags": MessageFlags::SUPPRESS_EMBEDS }),
                )
                .await;
            let _ = REST
                .post::<Message, _>(
                    format!("channels/{}/messages", event.channel_id),
                    json!({
                        "content": format!("<@{}> {}", event.author.id, new_urls.join("\n")),
                        "message_reference": { "message_id": event.id.to_string() },
                        "allowed_mentions": { "replied_user": false },
                    }),
                )
                .await;
        }

        Ok(())
    }
}
