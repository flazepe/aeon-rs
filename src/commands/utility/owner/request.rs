use crate::{
    statics::{colors::PRIMARY_COLOR, CONFIG, REQWEST},
    structs::command_context::CommandContext,
};
use anyhow::Result;
use reqwest::Method;
use serde_json::{from_str, Value};
use slashook::structs::embeds::Embed;

pub async fn run(ctx: CommandContext) -> Result<()> {
    let method = match ctx.get_string_arg("method").as_deref().unwrap_or("GET") {
        "GET" => Method::GET,
        "POST" => Method::POST,
        "PUT" => Method::PUT,
        "DELETE" => Method::DELETE,
        "HEAD" => Method::HEAD,
        "OPTIONS" => Method::OPTIONS,
        "CONNECT" => Method::CONNECT,
        "PATCH" => Method::PATCH,
        "TRACE" => Method::TRACE,
        _ => Method::GET,
    };
    let url: String = format!("https://discord.com/api/{}", ctx.get_string_arg("endpoint")?);
    let mut request = REQWEST.request(method, url).header("authorization", format!("Bot {}", CONFIG.bot.token));

    if let Ok(body) = ctx.get_string_arg("body") {
        let content_type = from_str::<Value>(&body).map_or("application/x-www-form-urlencoded", |_| "application/json");
        request = request.header("content-type", content_type).body(body);
    }

    match request.send().await {
        Ok(response) => {
            let text = response.text().await.unwrap_or_else(|_| "An error occurred while encoding text.".into());
            let result = match from_str::<Value>(text.as_str()) {
                Ok(json) => format!("{json:#}"),
                Err(_) => {
                    if text.is_empty() {
                        "No response.".into()
                    } else {
                        text
                    }
                },
            }
            .chars()
            .take(4087)
            .collect::<String>();
            let description = format!("```js\n{result}```");
            let embed = Embed::new().set_color(PRIMARY_COLOR)?.set_description(description);

            ctx.respond(embed, true).await
        },
        Err(error) => ctx.respond_error(format!("```{error}```"), true).await,
    }
}
