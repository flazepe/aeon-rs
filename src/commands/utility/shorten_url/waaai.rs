use crate::{
    statics::{CONFIG, REQWEST},
    structs::command_context::CommandContext,
};
use anyhow::Result;
use serde_json::{json, Value};

pub async fn run(ctx: CommandContext) -> Result<()> {
    let json = REQWEST
        .post("https://api.waa.ai/v2/links")
        .header("authorization", format!("API-Key {}", CONFIG.api.waaai_key))
        .json(&json!({
            "url": ctx.get_string_arg("url")?,
            "custom_code": ctx.get_string_arg("custom-id").unwrap_or("".into()),
        }))
        .send()
        .await?
        .json::<Value>()
        .await?;

    match json["success"].as_bool().unwrap_or(false) {
        true => ctx.respond_success(format!("<{}>", json["data"]["link"].as_str().unwrap()), false).await,
        false => ctx.respond_error("Custom ID already exists.", true).await,
    }
}
