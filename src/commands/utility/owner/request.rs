use crate::{
    statics::{CONFIG, REQWEST, colors::PRIMARY_EMBED_COLOR},
    structs::command_context::{AeonCommandContext, AeonCommandInput},
};
use anyhow::{Error, Result};
use reqwest::Method;
use serde_json::{Value, from_str};
use slashook::structs::embeds::Embed;
use std::sync::Arc;

pub async fn run(ctx: Arc<AeonCommandContext>) -> Result<()> {
    let AeonCommandInput::ApplicationCommand(..) = &ctx.command_input else { return Ok(()) };
    let method = Method::from_bytes(ctx.get_string_arg("method", 0, true).as_deref().unwrap_or("GET").as_bytes()).unwrap_or(Method::GET);
    let url: String = format!("https://discord.com/api/{}", ctx.get_string_arg("endpoint", 0, true)?);
    let mut request = REQWEST.request(method, url).header("authorization", format!("Bot {}", CONFIG.bot.token));

    if let Ok(body) = ctx.get_string_arg("body", 0, true) {
        let content_type = from_str::<Value>(&body).map_or("application/x-www-form-urlencoded", |_| "application/json");
        request = request.header("content-type", content_type).body(body);
    }

    let response = request.send().await.map_err(|error| Error::msg(format!("```{error}```")))?;
    let response_text = response.text().await;

    let plain_text = response_text.as_deref().unwrap_or("An error occurred while encoding text.");
    let json_text = from_str::<Value>(plain_text).map(|json| format!("{json:#}"));
    let final_text = json_text
        .as_deref()
        .unwrap_or(if plain_text.is_empty() { "No response." } else { plain_text })
        .chars()
        .take(4087)
        .collect::<String>();

    let description = format!("```js\n{final_text}```");
    let embed = Embed::new().set_color(PRIMARY_EMBED_COLOR)?.set_description(description);

    ctx.respond(embed, true).await
}
