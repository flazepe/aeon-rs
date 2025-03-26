use crate::{
    statics::REQWEST,
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

    let json =
        REQWEST.post("https://api.zws.im").header("user-agent", "yes").json(&json!({ "url": url })).send().await?.json::<Value>().await?;
    let shortened_url = json["url"].as_str().context("Invalid URL.")?;

    ctx.respond_success(format!("`{shortened_url}`"), true).await
}
