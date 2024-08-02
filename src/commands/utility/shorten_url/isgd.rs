use crate::{statics::REQWEST, structs::command_context::CommandContext};
use anyhow::Result;

pub async fn run(ctx: CommandContext) -> Result<()> {
    let opt = if ctx.get_bool_arg("lowercase").unwrap_or(false) {
        1
    } else if ctx.get_bool_arg("pronounceable").unwrap_or(false) {
        2
    } else {
        0
    };
    let shortened_url = REQWEST
        .post("https://is.gd/create.php")
        .header("user-agent", "yes")
        .query(&[
            ("format", "simple"),
            ("url", ctx.get_string_arg("url").as_deref().unwrap()),
            ("shorturl", ctx.get_string_arg("custom-id").as_deref().unwrap_or("")),
        ])
        .form(&[("opt", opt)])
        .send()
        .await?
        .text()
        .await?;

    match shortened_url.starts_with("http") {
        true => ctx.respond_success(format!("<{shortened_url}>"), true).await,
        false => ctx.respond_error(shortened_url.trim_start_matches("Error: "), true).await,
    }
}
