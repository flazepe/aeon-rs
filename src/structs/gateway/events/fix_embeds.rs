use crate::{
    statics::{
        REQWEST, REST,
        regex::{DISCORD_URL_REGEX, SPOILER_REGEX},
    },
    structs::{
        database::{Database, redis::keys::RedisKey},
        gateway::events::EventHandler,
    },
};
use anyhow::Result;
use nipper::Document;
use regex::Captures;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::json;
use slashook::{
    commands::MessageResponse,
    structs::messages::{AllowedMentions, Message as SlashookMessage, MessageReference},
};
use std::{collections::HashMap, sync::LazyLock};
use tracing::warn;
use twilight_model::channel::message::{Message, MessageFlags};

const DISCORD_USER_AGENT: &str = "Mozilla/5.0 (compatible; Discordbot/2.0; +https://discordapp.com)";

static EMBED_FIXER_MAPPINGS: LazyLock<Vec<(Vec<&str>, Vec<&str>)>> = LazyLock::new(|| {
    vec![
        (vec!["bilibili.com"], vec!["vxbilibili.com"]),
        (vec!["facebook.com"], vec!["facebed.com"]),
        (vec!["instagram.com"], vec!["eeinstagram.com", "kkinstagram.com"]),
        (vec!["pixiv.net"], vec!["phixiv.net"]),
        (vec!["reddit.com"], vec!["rxddit.com"]),
        (vec!["tiktok.com"], vec!["a.tnktok.com", "kktiktok.com"]),
        (vec!["weibo.com", "weibo.cn"], vec!["fxweibo.com"]),
        (vec!["x.com", "twitter.com"], vec!["fixupx.com", "fixvx.com"]),
    ]
});

impl EventHandler {
    pub async fn handle_fix_embeds(message: &Message) -> Result<()> {
        let Some(guild_id) = &message.guild_id else { return Ok(()) };

        let mongodb = Database::get_mongodb()?;
        let guild = mongodb.guilds.get(guild_id).await?;

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

        // If the message has embeds suppressed, we really have to fix all URLs
        let force_fix_all = message.flags.is_some_and(|flags| flags.contains(MessageFlags::SUPPRESS_EMBEDS));

        let mut fixed_urls = vec![];

        for discord_url in discord_urls.values() {
            // Discord only embeds up to 5 URLs
            if fixed_urls.len() == 5 {
                break;
            }

            if discord_url.suppressed {
                continue;
            }

            let url = &discord_url.url;
            let Some(domain) = url.split('/').nth(2).map(|domain| domain.trim_start_matches("www.")) else { continue };
            let path = url.split("?").next().unwrap_or_default().split('/').skip(3).collect::<Vec<&str>>().join("/");

            // Skip X posts that have a valid image
            // Discord does not follow photo indices (it always only shows the first photo), so we don't skip fixing those
            if ["x.com", "twitter.com"].contains(&domain) && (!path.contains("/photo/") || path.contains("/photo/1")) && !force_fix_all {
                let html = REQWEST.get(url).header("user-agent", DISCORD_USER_AGENT).send().await?.text().await?;
                let image_url = get_meta_contents(&html, &["og:image"]).into_values().next().unwrap_or_default();

                // Make sure the URL contains the "media" path. Otherwise, it is most likely a thumbnail for a video, which should be fixed
                // Also make sure that it's a valid media (status code OK). Sometimes it likes to return a placeholder URL that leads to a 404
                if image_url.contains("/media/") && REQWEST.head(image_url).send().await.is_ok_and(|res| res.status() == StatusCode::OK) {
                    continue;
                }
            }

            let Some((_, fixed_domains)) = EMBED_FIXER_MAPPINGS.iter().find(|(original_domains, _)| {
                original_domains
                    .iter()
                    .any(|original_domain| *original_domain == domain || domain.ends_with(&format!(".{original_domain}")))
            }) else {
                continue;
            };

            let mut fixed_url = None;

            for fixed_domain in fixed_domains {
                let new_fixed_url = format!("https://{fixed_domain}/{path}");

                match check_valid_fixer_response(&new_fixed_url, force_fix_all).await {
                    Ok(is_valid) => {
                        if is_valid {
                            fixed_url = Some(new_fixed_url);
                            break;
                        }
                    },
                    Err(error) => {
                        warn!(target: "FixEmbeds", r#"An error occurred while checking fixer {fixed_domain} for "{url}": {error:?}"#);
                    },
                }
            }

            let Some(fixed_url) = fixed_url else { continue };

            // The space before the closing spoiler is intentional because Discord sometimes includes the || inside the URL when unfurling, which causes the website to return a 404 and not embed
            let formatted_fixed_url = if discord_url.spoilered { format!("||{fixed_url} ||") } else { fixed_url };

            if !fixed_urls.contains(&formatted_fixed_url) {
                fixed_urls.push(formatted_fixed_url);
            }
        }

        let response = MessageResponse::from(format!(
            "<@{}> {}",
            message.author.id,
            if fixed_urls.is_empty() { "_Message was edited to not contain any fixable URLs._".into() } else { fixed_urls.join("\n") },
        ))
        .set_message_reference(MessageReference::new_reply(message.id))
        .set_allowed_mentions(AllowedMentions::new());

        let redis = Database::get_redis()?;
        let Some(guild_id) = message.guild_id else { return Ok(()) };
        let channel_id = message.channel_id;
        let message_id = message.id;
        let embed_fix_response_key =
            RedisKey::GuildChannelMessageEmbedFixResponse(guild_id.to_string(), channel_id.to_string(), message_id.to_string());

        if let Ok(mut embed_fix_response) = redis.get::<EmbedFixResponse>(&embed_fix_response_key).await {
            let embed_fix_response_id = &embed_fix_response.id;

            if embed_fix_response.discord_urls.iter().ne(discord_urls.iter()) {
                _ = REST
                    .patch::<(), _>(
                        format!("channels/{channel_id}/messages/{embed_fix_response_id}"),
                        json!({ "content": response.content.unwrap_or_default(), "allowed_mentions": { "parse": [] } }),
                    )
                    .await;

                embed_fix_response.discord_urls = discord_urls;
                redis.set(&embed_fix_response_key, embed_fix_response, Some(60 * 5)).await?;
            }

            return Ok(());
        }

        if !fixed_urls.is_empty()
            && let Ok(embed_fix_response_message) = SlashookMessage::create(&REST, channel_id, response).await
        {
            _ = REST
                .patch::<(), _>(format!("channels/{channel_id}/messages/{message_id}"), json!({ "flags": MessageFlags::SUPPRESS_EMBEDS }))
                .await;

            redis
                .set(
                    &embed_fix_response_key,
                    EmbedFixResponse { id: embed_fix_response_message.id.unwrap_or_default(), discord_urls },
                    Some(60 * 5),
                )
                .await?;
        }

        Ok(())
    }
}

fn get_meta_contents(html: &str, names: &[&str]) -> HashMap<String, String> {
    let document = Document::from(html);
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

fn get_oembed_activity_url(html: &str) -> Option<String> {
    let document = Document::from(html);
    document.select("link[type='application/activity+json']").attr("href").map(|url| url.to_string())
}

async fn check_valid_mastodon_status(oembed_activity_url: &str) -> Result<bool> {
    let mut split = oembed_activity_url.split('/');
    let domain = split.by_ref().take(3).collect::<String>();
    let snowcode = split.by_ref().next_back().unwrap_or_default();
    let api_url = format!("{domain}/api/v1/statuses/{snowcode}");

    let res = REQWEST.get(api_url).header("user-agent", DISCORD_USER_AGENT).send().await?;
    let mastodon_status = res.json::<MastodonStatus>().await?;
    let has_image_or_video = mastodon_status.media_attachments.iter().any(|media_attachment| {
        [MastodonStatusMediaAttachmentType::Image, MastodonStatusMediaAttachmentType::Video].contains(&media_attachment.attachment_type)
    });

    Ok(has_image_or_video)
}

async fn check_valid_fixer_response(url: &str, force_valid: bool) -> Result<bool> {
    if force_valid {
        return Ok(true);
    }

    let mut res = REQWEST.head(url).header("user-agent", DISCORD_USER_AGENT).send().await?;
    let mut has_body = false;

    if res.status() == StatusCode::METHOD_NOT_ALLOWED {
        res = REQWEST.get(url).header("user-agent", DISCORD_USER_AGENT).send().await?;
        has_body = true;
    }

    let content_type = res.headers().get("content-type").map(|value| value.to_str().unwrap_or_default()).unwrap_or_default();

    if content_type.starts_with("image/") || content_type.starts_with("video/") {
        return Ok(true);
    }

    if content_type.starts_with("text/html") {
        let html = if has_body {
            res.text().await?
        } else {
            REQWEST.get(url).header("user-agent", DISCORD_USER_AGENT).send().await?.text().await?
        };

        if let Some(oembed_activity_url) = get_oembed_activity_url(&html) {
            return check_valid_mastodon_status(&oembed_activity_url).await;
        }

        let meta_contents = get_meta_contents(&html, &["og:image", "og:video", "twitter:card", "twitter:image", "twitter:video"]);
        return Ok(!meta_contents.is_empty());
    }

    Ok(false)
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct DiscordURL {
    pub url: String,
    pub spoilered: bool,
    pub suppressed: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EmbedFixResponse {
    pub id: String,
    pub discord_urls: HashMap<String, DiscordURL>,
}

#[derive(Deserialize, Debug)]
pub struct MastodonStatus {
    media_attachments: Vec<MastodonStatusMediaAttachment>,
}

#[derive(Deserialize, Debug)]
pub struct MastodonStatusMediaAttachment {
    #[serde(rename = "type")]
    attachment_type: MastodonStatusMediaAttachmentType,
}

#[derive(Deserialize, PartialEq, Debug)]
#[serde(rename_all = "lowercase")]
pub enum MastodonStatusMediaAttachmentType {
    Image,
    Video,
    Gifv,
    Audio,

    #[serde(other)]
    Unknown,
}
