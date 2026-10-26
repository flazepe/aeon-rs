pub mod statics;

use crate::{
    functions::paginate_strings,
    statics::{CONFIG, REQWEST, colors::PRIMARY_EMBED_COLOR},
    structs::{api::piston::statics::PISTON_RUNTIMES, pagination::Pagination},
};
use anyhow::{Context, Result};
use serde::Deserialize;
use serde_json::{from_str, json};
use slashook::structs::embeds::Embed;
use std::fmt::Display;

#[derive(Debug)]
pub struct Piston {
    pub programming_language: String,
    pub input: String,
    pub output: Option<String>,
    pub exit_code: Option<u64>,
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
    pub code: Option<u64>,
}

impl Piston {
    pub fn new<T: Display, U: Display>(programming_language: T, code: U) -> Self {
        Self {
            programming_language: programming_language.to_string().to_lowercase(),
            input: code.to_string(),

            output: None,
            exit_code: None,
        }
    }

    pub async fn run(mut self) -> Result<Self> {
        let runtime = PISTON_RUNTIMES
            .get(&self.programming_language)
            .or_else(|| PISTON_RUNTIMES.values().find(|runtime| runtime.aliases.contains(&self.programming_language)))
            .context("Invalid programming language.")?;

        self.programming_language = runtime.label();

        let body = REQWEST
            .post("https://emkc.org/api/v2/piston/execute")
            .header("authorization", &CONFIG.api.piston_key)
            .json(&json!({
                "language": runtime.language,
                "version": runtime.version,
                "files": [{ "content": self.input }],
            }))
            .send()
            .await?
            .text()
            .await?;

        if let Ok(result) = from_str::<PistonExecute>(&body) {
            self.exit_code = result.run.code;
            self.output = Some(result.run.output.clone());
        } else {
            self.output = Some(body.clone());
        }

        Ok(self)
    }

    pub fn format_pagination<T: Display>(&self, command: T) -> Pagination {
        let title = &self.programming_language;

        let output_pages = paginate_strings(self.output.as_deref().unwrap_or_default().chars(), "", 4089)
            .into_iter()
            .map(|entry| {
                let mut embed = Embed::new()
                    .set_color(PRIMARY_EMBED_COLOR)
                    .unwrap_or_default()
                    .set_title(title)
                    .set_description(format!("```\n{entry}```"));

                if let Some(exit_code) = self.exit_code {
                    embed = embed.set_footer(format!("Exit Code: {exit_code}"), None::<String>);
                }

                embed
            })
            .collect::<Vec<_>>();

        let input_pages = paginate_strings(self.input.chars(), "", 4089)
            .into_iter()
            .map(|entry| {
                Embed::new().set_color(PRIMARY_EMBED_COLOR).unwrap_or_default().set_title(title).set_description(format!("```\n{entry}```"))
            })
            .collect::<Vec<_>>();

        Pagination::new(command).add_section("Output", output_pages).add_section("Input", input_pages)
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
