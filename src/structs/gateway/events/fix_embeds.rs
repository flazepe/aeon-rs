use crate::{
    statics::{REQWEST, REST, regex::URL_REGEX},
    structs::{database::guilds::Guilds, gateway::events::EventHandler},
};
use anyhow::Result;
use regex::Regex;
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
                let body = REQWEST.get(url).header("user-agent", "discordbot").send().await?.text().await?;
                let image_url = get_meta_content(body.as_str(), "og:image");

                if image_url.contains("/media/") && REQWEST.head(image_url).send().await.is_ok_and(|res| res.status() == StatusCode::OK) {
                    continue;
                }
            }

            let new_domain = match domain {
                // "bsky.app" => "fxbsky.app",
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
            let body = REQWEST.get(&new_url).header("user-agent", "discordbot").send().await?.text().await?;

            // Only fix posts that were supposed to have an image or video
            if ["og:image", "og:video", "twitter:card", "twitter:image", "twitter:video"]
                .iter()
                .all(|entry| get_meta_content(&body, entry).is_empty())
            {
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

fn get_meta_content(html: &str, property: &str) -> String {
    let Ok(regex) = Regex::new(
        format!(r#"<meta\s*content="(\S+)"\s*property="{property}"\s*/?>|<meta\s*property="{property}"\s*content="(\S+)"\s*/?>"#).as_str(),
    ) else {
        return "".into();
    };

    let content =
        regex.captures(html).and_then(|captures| captures.get(1).or(captures.get(2))).map(|capture| capture.as_str()).unwrap_or_default();

    // Ignore some invalid contents
    if ["0", "undefined"].contains(&content) || content.starts_with("https://pbs.twimg.com/profile_images/") {
        return "".into();
    }

    content.into()
}
