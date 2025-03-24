use crate::{
    statics::REQWEST,
    structs::command_context::{CommandContext, CommandInputExt, Input},
};
use anyhow::Result;
use serde_json::Value;

pub async fn run(ctx: CommandContext) -> Result<()> {
    let Input::ApplicationCommand(input,  _) = &ctx.input else { return Ok(()) };
    let mut url = input.get_string_arg("url")?;

    if !url.starts_with("http") {
        url = format!("http://{url}");
    }

    let json = REQWEST.post("https://cleanuri.com/api/v1/shorten").form(&[("url", url)]).send().await?.json::<Value>().await?;

    match json["result_url"].as_str() {
        Some(shortened_url) => ctx.respond_success(format!("<{shortened_url}>"), true).await,
        None => ctx.respond_error("Invalid URL.", true).await,
    }
}
