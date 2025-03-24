use crate::{
    statics::{CONFIG, REQWEST},
    structs::command_context::{CommandContext, CommandInputExt, Input},
};
use anyhow::Result;
use serde_json::{Value, json};

pub async fn run(ctx: CommandContext) -> Result<()> {
    let Input::ApplicationCommand(input,  _) = &ctx.input else { return Ok(()) };
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

    match json["data"]["link"].as_str() {
        Some(shortened_url) => ctx.respond_success(format!("<{shortened_url}>"), true).await,
        None => ctx.respond_error("Custom ID already exists.", true).await,
    }
}
