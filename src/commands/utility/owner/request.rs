use crate::{
    statics::{CONFIG, REQWEST},
    structs::command_context::CommandContext,
};
use anyhow::Result;
use reqwest::Method;
use serde_json::{from_str, Value};

pub async fn run(ctx: CommandContext) -> Result<()> {
    let mut request = REQWEST
        .request(
            match ctx.get_string_arg("method").unwrap_or("GET".into()).as_ref() {
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
            let text = response.text().await.unwrap_or("No response.".into());

            ctx.respond(
                format!(
                    "```js\n{}```",
                    match from_str::<Value>(text.as_str()) {
                        Ok(json) => format!("{json:#}"),
                        Err(_) => text,
                    }
                    .chars()
                    .take(1991)
                    .collect::<String>()
                ),
                true,
            )
            .await
        },
        Err(error) => ctx.respond_error(format!("```{error}```"), true).await,
    }
}
