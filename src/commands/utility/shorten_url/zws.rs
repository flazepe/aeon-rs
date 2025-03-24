use crate::{
    statics::REQWEST,
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

    match REQWEST.post("https://api.zws.im").header("user-agent", "yes").json(&json!({ "url": url })).send().await?.json::<Value>().await?
        ["url"]
        .as_str()
    {
        Some(url) => ctx.respond_success(format!("`{url}`"), true).await,
        None => ctx.respond_error("Invalid URL.", true).await,
    }
}
