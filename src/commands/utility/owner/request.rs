use crate::{
    statics::{colors::PRIMARY_COLOR, CONFIG, REQWEST},
    structs::command_context::CommandContext,
};
use anyhow::Result;
use reqwest::Method;
use serde_json::{from_str, Value};
use slashook::structs::embeds::Embed;

pub async fn run(ctx: CommandContext) -> Result<()> {
    let mut request = REQWEST
        .request(
            match ctx.get_string_arg("method").as_deref().unwrap_or("GET") {
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
            },
            format!("https://discord.com/api/{}", ctx.get_string_arg("endpoint")?),
        )
        .header("authorization", format!("Bot {}", CONFIG.bot.token));

    if let Ok(body) = ctx.get_string_arg("body") {
        request = request
            .header(
                "content-type",
                match from_str::<Value>(&body).is_ok() {
                    true => "application/json",
                    false => "application/x-www-form-urlencoded",
                },
            )
            .body(body);
    }

    match request.send().await {
        Ok(response) => {
            let text = response.text().await.unwrap_or_else(|_| "An error occurred while encoding text.".into());

            ctx.respond(
                Embed::new().set_color(PRIMARY_COLOR)?.set_description(format!(
                    "```js\n{}```",
                    match from_str::<Value>(text.as_str()) {
                        Ok(json) => format!("{json:#}"),
                        Err(_) => match text.is_empty() {
                            true => "No response.".into(),
                            false => text,
                        },
                    }
                    .chars()
                    .take(4087)
                    .collect::<String>(),
                )),
                true,
            )
            .await
        },
        Err(error) => ctx.respond_error(format!("```{error}```"), true).await,
    }
}
