use crate::{
    statics::{
        CACHE, REQWEST, REST,
        regex::{DISCORD_URL_REGEX, SPOILER_REGEX},
    },
    structs::{database::guilds::Guilds, gateway::events::EventHandler},
};
use anyhow::Result;
use nipper::Document;
use regex::Captures;
use reqwest::StatusCode;
use serde_json::json;
use slashook::{
    commands::MessageResponse,
    structs::messages::{AllowedMentions, Message as SlashookMessage, MessageFlags, MessageReference},
};
use std::collections::HashMap;
use twilight_model::channel::Message;

impl EventHandler {
    pub async fn handle_fix_embeds(message: &Message) -> Result<()> {
        let Some(guild_id) = &message.guild_id else { return Ok(()) };
        let guild = Guilds::get(guild_id).await?;

        if !guild.fix_embeds.enabled || message.author.bot {
            return Ok(());
        }

        let mut discord_urls: HashMap<String, DiscordURL> = HashMap::new();

        let mut process_captures = |captures: Captures<'_>, is_spoiler: bool| {
            let suppressed_url = captures.name("suppressed_url");
            let normal_url = captures.name("normal_url");

            let url = suppressed_url.or(normal_url);
            let Some(url) = url else { return };

            if let Some(discord_url) = discord_urls.get_mut(url.as_str()) {
                discord_url.spoilered = is_spoiler;
                discord_url.suppressed = suppressed_url.is_some();
            } else {
                discord_urls.insert(
                    url.as_str().to_string(),
                    DiscordURL { url: url.as_str().to_string(), spoilered: is_spoiler, suppressed: suppressed_url.is_some() },
                );
            }
        };

        for captures in DISCORD_URL_REGEX.captures_iter(&message.content) {
            process_captures(captures, false);
        }

        for spoiler in SPOILER_REGEX.find_iter(&message.content) {
            for captures in DISCORD_URL_REGEX.captures_iter(spoiler.as_str()) {
                process_captures(captures, true);
            }
        }

        let mut urls = vec![];

        for discord_url in discord_urls.values() {
            // Discord only embeds up to 5 URLs
            if urls.len() == 5 {
                break;
            }

            // Skip suppressed URLs
            if discord_url.suppressed {
                continue;
            }

            let url = &discord_url.url;
            let Some(domain) = url.split('/').nth(2).map(|domain| domain.trim_start_matches("www.")) else { continue };

            // Skip X posts that have a valid image
            if ["x.com", "twitter.com"].contains(&domain) {
                let html = REQWEST.get(url).header("user-agent", "discordbot").send().await?.text().await?;
                let image_url = get_meta_contents(html, &["og:image"]).into_values().next().unwrap_or_default();

                // Make sure the URL contains the "media" path. Otherwise, it is most likely a thumbnail for a video, which should be fixed
                // Also make sure that it's a valid media (status code OK). Sometimes it likes to return a placeholder URL that leads to a 404
                if !image_url.is_empty()
                    && image_url.contains("/media/")
                    && REQWEST.head(image_url).send().await.is_ok_and(|res| res.status() == StatusCode::OK)
                {
                    continue;
                }
            }

            let new_domain = match domain {
                "bilibili.com" => "vxbilibili.com",
                "instagram.com" => "eeinstagram.com",
                "pixiv.net" => "phixiv.net",
                "reddit.com" | "old.reddit.com" => "rxddit.com",
                "tiktok.com" => "vxtiktok.com",
                "vt.tiktok.com" => "vt.vxtiktok.com",
                "x.com" | "twitter.com" => "fixupx.com",
                _ => continue,
            };
            let path = url.split('/').skip(3).map(|str| str.to_string()).collect::<Vec<String>>().join("/");
            let path = path.split("?").next().unwrap_or_default(); // Trim query string
            let new_url = format!("https://{new_domain}/{path}");

            if urls.contains(&new_url) {
                continue;
            }

            let has_media_content_type =
                REQWEST.head(&new_url).header("user-agent", "discordbot").send().await?.headers().iter().any(|header| {
                    let value = format!("{:?}", header.1);
                    header.0 == "content-type" && (value.contains("image") || value.contains("video"))
                });

            let has_media_meta_content = {
                let html = REQWEST.get(&new_url).header("user-agent", "discordbot").send().await?.text().await?;
                !get_meta_contents(html, &["og:image", "og:video", "twitter:card", "twitter:image", "twitter:video"]).is_empty()
            };

            // Only fix posts that were supposed to have an image or video
            if has_media_content_type || has_media_meta_content {
                // The space before the closing spoiler is intentional because Discord (could be the website) sometimes includes the || inside the URL when unfurling, which causes the website to return a 404 and not embed
                urls.push(if discord_url.spoilered { format!("||{new_url} ||") } else { new_url });
            }
        }

        let response = MessageResponse::from(format!("<@{}> {}", message.author.id, urls.join("\n")))
            .set_message_reference(MessageReference::new_reply(message.id))
            .set_allowed_mentions(AllowedMentions::new().set_replied_user(false));
        let embed_fix_response = CACHE.discord.embed_fix_responses.read().unwrap().get(message.id.to_string().as_str()).cloned();

        if let Some(embed_fix_response) = embed_fix_response {
            if embed_fix_response.content == response.content.as_deref().unwrap_or_default() {
                return Ok(());
            }

            if urls.is_empty() {
                _ = embed_fix_response.delete(&REST).await;
            } else {
                _ = embed_fix_response.edit(&REST, response).await;
            }
        } else if !urls.is_empty() {
            _ = REST
                .patch::<(), _>(
                    format!("channels/{}/messages/{}", message.channel_id, message.id),
                    json!({ "flags": MessageFlags::SUPPRESS_EMBEDS }),
                )
                .await;

            if let Ok(embed_fix_response) = SlashookMessage::create(&REST, message.channel_id, response).await {
                CACHE.discord.embed_fix_responses.write().unwrap().insert(message.id.to_string(), embed_fix_response);
            }
        }

        Ok(())
    }
}

fn get_meta_contents(html: String, names: &[&str]) -> HashMap<String, String> {
    let document = Document::from(&html);
    let mut contents = HashMap::new();

    for name in names {
        let name_content = document.select(&format!("meta[name='{name}']")).attr("content");
        let property_content = document.select(&format!("meta[property='{name}']")).attr("content");
        let Some(content) = name_content.or(property_content) else { continue };

        // Ignore some invalid contents
        if ["0", "undefined"].contains(&content.to_string().as_str()) || content.starts_with("https://pbs.twimg.com/profile_images/") {
            continue;
        }

        contents.insert(name.to_string(), content.to_string());
    }

    contents
}

struct DiscordURL {
    pub url: String,
    pub spoilered: bool,
    pub suppressed: bool,
}
