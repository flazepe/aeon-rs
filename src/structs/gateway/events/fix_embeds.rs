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
use slashook::{
    commands::MessageResponse,
    structs::messages::{AllowedMentions, Message as SlashookMessage, MessageFlags, MessageReference},
};
use twilight_model::channel::Message;

impl EventHandler {
    pub async fn handle_fix_embeds(message: &Message) -> Result<()> {
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

                if !image_url.contains("amplify_video_thumb")
                    && REQWEST.get(image_url).send().await.is_ok_and(|res| res.status() == StatusCode::OK)
                {
                    continue;
                }
            }

            let new_domain = match domain {
                "bsky.app" => "vxbsky.app",
                "instagram.com" => "ddinstagram.com",
                "pixiv.net" => "phixiv.net",
                "reddit.com" | "old.reddit.com" => "rxddit.com",
                "tiktok.com" => "vxtiktok.com",
                "vt.tiktok.com" => "vt.vxtiktok.com",
                "x.com" | "twitter.com" => "fixvx.com",
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

            let spoiler = message.content.contains(&format!("||{url}||"));
            new_urls.push(if spoiler { format!("||{new_url}||") } else { new_url });
        }

        if !new_urls.is_empty() {
            _ = REST
                .patch::<(), _>(
                    format!("channels/{}/messages/{}", message.channel_id, message.id),
                    json!({ "flags": MessageFlags::SUPPRESS_EMBEDS }),
                )
                .await;
            _ = SlashookMessage::create(
                &REST,
                message.channel_id,
                MessageResponse::from(format!("<@{}> {}", message.author.id, new_urls.join("\n")))
                    .set_message_reference(MessageReference::new_reply(message.id))
                    .set_allowed_mentions(AllowedMentions::new().set_replied_user(false)),
            )
            .await;
        }

        Ok(())
    }
}
