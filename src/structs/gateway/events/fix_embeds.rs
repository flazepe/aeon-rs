use crate::{
    statics::{CACHE, REQWEST, REST, regex::URL_REGEX},
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

        if !guild.fix_embeds.enabled || message.author.bot {
            return Ok(());
        }

        let discord_urls = URL_REGEX
            .find_iter(&message.content)
            .map(|entry| {
                let string = message.content.chars().take(entry.start()).collect::<String>();
                let mut chars = string.trim().chars();
                let previous_char = chars.by_ref().next_back().unwrap_or_default();
                let second_previous_char = chars.by_ref().next_back().unwrap_or_default();

                DiscordURL {
                    url: entry.as_str().to_string(),
                    spoilered: previous_char == '|' && second_previous_char == '|',
                    suppressed: previous_char == '<',
                }
            })
            .collect::<Vec<DiscordURL>>();

        let mut urls = vec![];

        for discord_url in discord_urls {
            let url = &discord_url.url;

            // Skip suppressed embeds
            if discord_url.suppressed {
                continue;
            }

            let Some(domain) = url.split('/').nth(2).map(|domain| domain.trim_start_matches("www.")) else { continue };

            // Skip Bluesky/X posts that have a valid image
            if ["bsky.app", "x.com", "twitter.com"].contains(&domain) {
                let body = REQWEST.get(url).header("user-agent", "discordbot").send().await?.text().await?;
                let image_url = get_meta_content(body.as_str(), "og:image");

                // Make sure the URL contains the "media" path. Otherwise, it is most likely a thumbnail for a video, which should be fixed
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
            let is_valid_response =
                REQWEST.head(&new_url).header("user-agent", "discordbot").send().await?.headers().iter().any(|header| {
                    let value = format!("{:?}", header.1);
                    header.0 == "content-type" && (value.contains("image") || value.contains("video"))
                }) || {
                    let body = REQWEST.get(&new_url).header("user-agent", "discordbot").send().await?.text().await?;
                    ["og:image", "og:video", "twitter:card", "twitter:image", "twitter:video"]
                        .iter()
                        .any(|entry| !get_meta_content(&body, entry).is_empty())
                };

            // Only fix posts that were supposed to have an image or video
            if is_valid_response {
                urls.push(if discord_url.spoilered { format!("||{new_url}||") } else { new_url });
            }
        }

        let response = MessageResponse::from(format!("<@{}> {}", message.author.id, urls.join("\n")))
            .set_message_reference(MessageReference::new_reply(message.id))
            .set_allowed_mentions(AllowedMentions::new().set_replied_user(false));
        let embed_fix_response = CACHE.embed_fix_responses.read().unwrap().get(message.id.to_string().as_str()).cloned();

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
                CACHE.embed_fix_responses.write().unwrap().insert(message.id.to_string(), embed_fix_response);
            }
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

struct DiscordURL {
    pub url: String,
    pub spoilered: bool,
    pub suppressed: bool,
}
