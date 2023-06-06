use crate::{
    statics::{CONFIG, REQWEST},
    structs::command_context::CommandContext,
};
use anyhow::Result;
use serde_json::{json, Value};

pub async fn run(ctx: CommandContext) -> Result<()> {
    let mut url = ctx.get_string_arg("url")?;

    if !url.starts_with("http") {
        url = format!("http://{url}");
    }

    let json = REQWEST
        .post("https://api.waa.ai/v2/links")
        .header("authorization", format!("API-Key {}", CONFIG.api.waaai_key))
        .json(&json!({
            "url": url,
            "custom_code": ctx.get_string_arg("custom-id").unwrap_or("".into()),
            "private": ctx.get_bool_arg("hash").unwrap_or(false),
        }))
        .send()
        .await?
        .json::<Value>()
        .await?;

    match json["data"]["link"].as_str() {
        Some(url) => ctx.respond_success(format!("<{url}>"), true).await,
        None => ctx.respond_error("Custom ID already exists.", true).await,
    }
}
