use crate::{
    statics::{colors::PRIMARY_COLOR, CONFIG, REQWEST},
    structs::command_context::CommandContext,
};
use anyhow::Result;
use reqwest::Method;
use serde_json::{from_str, Value};
use slashook::structs::embeds::Embed;

pub async fn run(ctx: CommandContext) -> Result<()> {
    let method = Method::from_bytes(ctx.get_string_arg("method").as_deref().unwrap_or("GET").as_bytes()).unwrap_or(Method::GET);
    let url: String = format!("https://discord.com/api/{}", ctx.get_string_arg("endpoint")?);
    let mut request = REQWEST.request(method, url).header("authorization", format!("Bot {}", CONFIG.bot.token));

    if let Ok(body) = ctx.get_string_arg("body") {
        let content_type = from_str::<Value>(&body).map_or("application/x-www-form-urlencoded", |_| "application/json");
        request = request.header("content-type", content_type).body(body);
    }

    match request.send().await {
        Ok(response) => {
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
            let embed = Embed::new().set_color(PRIMARY_COLOR)?.set_description(description);

            ctx.respond(embed, true).await
        },
        Err(error) => ctx.respond_error(format!("```{error}```"), true).await,
    }
}
