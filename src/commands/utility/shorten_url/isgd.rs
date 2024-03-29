use crate::{statics::REQWEST, structs::command_context::CommandContext};
use anyhow::Result;

pub async fn run(ctx: CommandContext) -> Result<()> {
    let body = REQWEST
        .post("https://is.gd/create.php")
        .header("user-agent", "yes")
        .query(&[
            ("format", "simple"),
            ("url", ctx.get_string_arg("url").as_deref().unwrap()),
            ("shorturl", ctx.get_string_arg("custom-id").as_deref().unwrap_or("")),
        ])
        .form(&[(
            "opt",
            if ctx.get_bool_arg("lowercase").unwrap_or(false) {
                1
            } else if ctx.get_bool_arg("pronounceable").unwrap_or(false) {
                2
            } else {
                0
            },
        )])
        .send()
        .await?
        .text()
        .await?;

    match body.starts_with("http") {
        true => ctx.respond_success(format!("<{body}>"), true).await,
        false => ctx.respond_error(body.trim_start_matches("Error: "), true).await,
    }
}
