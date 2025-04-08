pub mod statics;

use crate::{
    functions::hastebin,
    statics::{REQWEST, colors::PRIMARY_EMBED_COLOR},
    structs::api::piston::statics::PISTON_RUNTIMES,
};
use anyhow::{Context, Result};
use serde::Deserialize;
use serde_json::{from_str, json};
use slashook::structs::embeds::Embed;
use std::fmt::Display;

#[derive(Debug)]
pub struct Piston {
    pub programming_language: String,
    pub code: String,
    pub code_url: Option<String>,
    pub output: Option<String>,
    pub output_url: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct PistonExecute {
    pub run: PistonExecuteRun,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct PistonExecuteRun {
    pub stdout: String,
    pub stderr: String,
    pub output: String,
    pub code: u64,
}

impl Piston {
    pub fn new<T: Display, U: Display>(programming_language: T, code: U) -> Self {
        Self {
            programming_language: programming_language.to_string().to_lowercase(),
            code: code.to_string(),
            code_url: None,
            output: None,
            output_url: None,
        }
    }

    pub async fn run(mut self) -> Result<Self> {
        let runtime = PISTON_RUNTIMES
            .iter()
            .find(|runtime| runtime.language == self.programming_language || runtime.aliases.contains(&self.programming_language))
            .context("Invalid programming language.")?;

        self.programming_language = runtime.label();
        self.code_url = Some(hastebin(&self.code).await?);

        let body = REQWEST
            .post("https://emkc.org/api/v2/piston/execute")
            .json(&json!({
                "language": runtime.language,
                "version": runtime.version,
                "files": [{ "content": self.code }],
            }))
            .send()
            .await?
            .text()
            .await?;

        if let Ok(result) = from_str::<PistonExecute>(&body) {
            let output = format!("{}\n\nexit code: {}", result.run.output.trim(), result.run.code).trim().to_string();

            self.output = Some(output.clone());

            if output.len() > 3900 {
                self.output_url = Some(hastebin(output).await?);
            }
        } else {
            self.output = Some(body.clone());
        }

        Ok(self)
    }

    pub fn format(&self) -> Embed {
        let title = &self.programming_language;
        let url = self.code_url.as_deref().unwrap_or_default();
        let description = format!(
            "{}```\n{}```",
            self.output_url.as_ref().map(|output_url| format!("[Full Output]({output_url})")).as_deref().unwrap_or_default(),
            self.output.as_deref().unwrap_or("No output.").chars().take(3900).collect::<String>(),
        );

        Embed::new().set_color(PRIMARY_EMBED_COLOR).unwrap_or_default().set_title(title).set_url(url).set_description(description)
    }
}

pub struct PistonRuntime {
    pub language: String,
    pub version: String,
    pub aliases: Vec<String>,
    pub runtime: Option<String>,
}

impl PistonRuntime {
    fn new(language: &str, version: &str, aliases: &[&str], runtime: Option<&str>) -> Self {
        Self {
            language: language.to_string(),
            version: version.to_string(),
            aliases: aliases.iter().map(|alias| alias.to_string()).collect(),
            runtime: runtime.map(|runtime| runtime.to_string()),
        }
    }

    pub fn label(&self) -> String {
        format!(
            "{} ({}v{})",
            self.language,
            self.runtime.as_ref().map(|runtime| format!("{runtime} ")).as_deref().unwrap_or_default(),
            self.version,
        )
    }
}
