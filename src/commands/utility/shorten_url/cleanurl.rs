use crate::{statics::REQWEST, structs::command_context::AeonCommandContext};
use anyhow::{Context, Result};
use serde_json::Value;
use std::sync::Arc;

pub async fn run(ctx: Arc<AeonCommandContext>) -> Result<()> {
    let mut url = ctx.get_string_arg("url", 0, true)?;

    if !url.starts_with("http") {
        url = format!("http://{url}");
    }

    let json = REQWEST.post("https://cleanuri.com/api/v1/shorten").form(&[("url", url)]).send().await?.json::<Value>().await?;
    let shortened_url = json["result_url"].as_str().context("Invalid URL.")?;

    ctx.respond_success(format!("<{shortened_url}>"), true).await
}
