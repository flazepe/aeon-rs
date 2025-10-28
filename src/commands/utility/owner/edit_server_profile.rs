use crate::{
    statics::REQWEST,
    structs::command_context::{AeonCommandContext, AeonCommandInput},
};
use anyhow::{Result, bail};
use base64::{Engine, prelude::BASE64_STANDARD};
use reqwest::header::HeaderMap;
use serde_json::{Value, json};
use std::sync::Arc;

pub async fn run(ctx: Arc<AeonCommandContext>) -> Result<()> {
    let AeonCommandInput::ApplicationCommand(input, _) = &ctx.command_input else { return Ok(()) };

    let guild_id = ctx.get_string_arg("server-id", 0, false).unwrap_or_else(|_| ctx.get_guild_id().unwrap());
    let endpoint = format!("guilds/{guild_id}/members/@me");

    let get_image_content_type = |headers: &HeaderMap| -> Result<String> {
        let content_type = headers.get("content-type").and_then(|content_type| content_type.to_str().ok()).unwrap_or_default();
        let Some((_, format)) = content_type.split_once('/') else { bail!("Invalid image provided.") };

        if !["png", "jpeg", "gif"].contains(&format) {
            bail!("Invalid image provided.");
        }

        Ok(content_type.into())
    };

    if let Ok(avatar) = ctx.get_attachment_arg("avatar") {
        ctx.defer(true).await?;

        let res = REQWEST.get(&avatar.url).send().await?;
        let content_type = get_image_content_type(res.headers())?;
        let bytes = res.bytes().await?;
        let base64 = BASE64_STANDARD.encode(bytes);

        input
            .rest
            .patch::<Value, Value>(
                endpoint.clone(),
                json!({
                    "avatar": format!("data:{content_type};base64,{base64}"),
                }),
            )
            .await?;

        return ctx.respond_success("Successfuly set bot's server avatar.", true).await;
    }

    if let Ok(banner) = ctx.get_attachment_arg("banner") {
        ctx.defer(true).await?;

        let res = REQWEST.get(&banner.url).send().await?;
        let content_type = get_image_content_type(res.headers())?;
        let bytes = res.bytes().await?;
        let base64 = BASE64_STANDARD.encode(bytes);

        input
            .rest
            .patch::<Value, Value>(
                endpoint.clone(),
                json!({
                    "banner": format!("data:{content_type};base64,{base64}"),
                }),
            )
            .await?;

        return ctx.respond_success("Successfuly set bot's server banner.", true).await;
    }

    if let Ok(nickname) = ctx.get_string_arg("nickname", 0, false) {
        input
            .rest
            .patch::<Value, Value>(
                endpoint.clone(),
                json!({
                    "nick": nickname,
                }),
            )
            .await?;

        return ctx.respond_success("Successfuly set bot's server nickname.", true).await;
    }

    if let Ok(about_me) = ctx.get_string_arg("about-me", 0, false) {
        input
            .rest
            .patch::<Value, Value>(
                endpoint.clone(),
                json!({
                    "bio": about_me,
                }),
            )
            .await?;

        return ctx.respond_success("Successfuly set bot's server about me.", true).await;
    }

    if ctx.get_bool_arg("reset-all").unwrap_or(false) {
        input
            .rest
            .patch::<Value, Value>(
                endpoint.clone(),
                json!({
                    "avatar": null,
                    "banner": null,
                    "nick": null,
                    "bio": null,
                }),
            )
            .await?;

        return ctx.respond_success("Successfuly reset bot's server profile.", true).await;
    }

    if let Ok(reset) = ctx.get_string_arg("reset", 0, false) {
        let property = match reset.as_str() {
            "nickname" => "nick",
            "about-me" => "bio",
            property => property,
        };

        input.rest.patch::<Value, Value>(endpoint.clone(), json!({ property: null })).await?;

        return ctx.respond_success(format!("Successfuly reset bot's server {}.", reset.replace('-', " ")), true).await;
    }

    Ok(())
}
