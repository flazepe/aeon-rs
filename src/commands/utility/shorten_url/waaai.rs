use crate::{
    statics::{CONFIG, REQWEST},
    structs::command_context::{AeonCommandContext, AeonCommandInput, CommandInputExt},
};
use anyhow::{Context, Result};
use serde_json::{Value, json};
use std::sync::Arc;

pub async fn run(ctx: Arc<AeonCommandContext>) -> Result<()> {
    let AeonCommandInput::ApplicationCommand(input, _) = &ctx.command_input else { return Ok(()) };
    let mut url = input.get_string_arg("url")?;

    if !url.starts_with("http") {
        url = format!("http://{url}");
    }

    let json = REQWEST
        .post("https://api.waa.ai/v2/links")
        .header("authorization", format!("API-Key {}", CONFIG.api.waaai_key))
        .json(&json!({
            "url": url,
            "custom_code": input.get_string_arg("custom-id").as_deref().unwrap_or(""),
            "private": input.get_bool_arg("hash").unwrap_or(false),
        }))
        .send()
        .await?
        .json::<Value>()
        .await?;
    let shortened_url = json["data"]["link"].as_str().context("Custom ID already exists")?;

    ctx.respond_success(format!("<{shortened_url}>"), true).await
}
