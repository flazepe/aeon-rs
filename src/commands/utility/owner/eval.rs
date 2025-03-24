use crate::structs::command_context::{AeonCommandContext, AeonCommandInput};
use anyhow::Result;
use serde_json::to_string;
use slashook::{commands::MessageResponse, structs::utils::File};
use std::{fmt::Display, process::Command};

pub async fn run(ctx: AeonCommandContext) -> Result<()> {
    let AeonCommandInput::MessageCommand(message, args, _) = &ctx.command_input else { return Ok(()) };

    let mut code = args.to_string();
    let mut flags = code.split(' ').last().unwrap_or("").to_string();

    if flags.starts_with('-') && flags.chars().skip(1).all(|char| char.is_alphabetic()) {
        code = code.trim_end_matches(&flags).trim_end().to_string();
    } else {
        flags.clear();
    }

    // Codeblock trim
    if code.starts_with("```") {
        code = code.trim_end_matches("```").chars().skip(if code.starts_with("```js") { 5 } else { 3 }).collect::<String>();
    }

    let mut text = "No result.".to_string();

    if let Ok(output) = Command::new("node")
        .args([
            if flags.contains('m') { "-e" } else { "-p" },
            &generate_eval_context(to_string(&message).unwrap_or_else(|_| "{}".into()), code),
            "--input-type",
            if flags.contains('m') { "module" } else { "commonjs" },
        ])
        .output()
    {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        text = format!("{}", output.status);

        if !stdout.trim().is_empty() {
            text += &format!("\nstdout:\n```js\n{}```", stdout.replace('`', "\u{200b}`"));
        }

        if !stderr.trim().is_empty() {
            text += &format!("\nstderr:\n```js\n{}```", stderr.replace('`', "\u{200b}`"));
        }
    }

    if !flags.contains('s') {
        return ctx
            .respond(
                if text.len() > 2000 { MessageResponse::from(File::new("result.txt", text)) } else { MessageResponse::from(text) },
                false,
            )
            .await;
    }

    Ok(())
}

fn generate_eval_context<T: Display>(message: T, code: String) -> String {
    format!(
        r#"
            function botFetch(url, options = {{}}) {{
                if (!url.startsWith("http")) url = `https://discord.com/api/${{url}}`;
                if (!options.headers) options.headers = {{}};

                try {{
                    options.headers.authorization = `Bot ${{fs.readFileSync("config.toml", "utf8").split("\n").find(line => line.startsWith("token = ")).split('"')[1]}}`;
                }} catch {{}}

                if (typeof options.body === "string" && !options.headers["content-type"]) options.headers["content-type"] = "application/json";
            
                return fetch(url, options);
            }}

            const message = {message};

            message.reply = (body, filename, extraPayload = {{}}) => {{
                if (filename) {{
                    const formData = new FormData();
                    formData.append("file", new Blob([body]), filename);
                    formData.append("payload_json", JSON.stringify(extraPayload))
                    body = formData;
                }} else {{
                    body = JSON.stringify({{ content: body, ...extraPayload }});
                }}

                return botFetch(`channels/${{message.channel_id}}/messages`, {{ method: "POST", body }});
            }};
            
            {code}
        "#,
    )
}
