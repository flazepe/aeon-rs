use crate::{
    statics::{CONFIG, REQWEST},
    structs::command_context::{AeonCommandContext, AeonCommandInput},
};
use anyhow::{Context, Result};
use serde_json::{Value, json};
use std::sync::Arc;

pub async fn run(ctx: Arc<AeonCommandContext>) -> Result<()> {
    let AeonCommandInput::ApplicationCommand(..) = &ctx.command_input else { return Ok(()) };
    let mut url = ctx.get_string_arg("url", 0, true)?;

    if !url.starts_with("http") {
        url = format!("http://{url}");
    }

    let json = REQWEST
        .post("https://api.waa.ai/v2/links")
        .header("authorization", format!("API-Key {}", CONFIG.api.waaai_key))
        .json(&json!({
            "url": url,
            "custom_code": ctx.get_string_arg("custom-id", 0, true).as_deref().unwrap_or_default(),
            "private": ctx.get_bool_arg("hash").unwrap_or(false),
        }))
        .send()
        .await?
        .json::<Value>()
        .await?;
    let shortened_url = json["data"]["link"].as_str().context("Custom ID already exists")?;

    ctx.respond_success(format!("<{shortened_url}>"), true).await
}
