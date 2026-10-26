pub mod statics;

use crate::{
    functions::paginate_strings,
    statics::{CONFIG, REQWEST, colors::PRIMARY_EMBED_COLOR},
    structs::{api::piston::statics::PISTON_RUNTIMES, pagination::Pagination},
};
use anyhow::{Result, bail};
use base64::{Engine, prelude::BASE64_STANDARD};
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use slashook::{
    chrono::Utc,
    commands::CommandInput,
    structs::{embeds::Embed, interactions::ApplicationCommandOptionChoice},
};
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug)]
pub struct Piston {
    pub runtime: &'static PistonRuntime,
    pub code: Option<String>,
    pub files: Vec<PistonExecuteInputFile>,
    pub output: Option<String>,
    pub exit_code: Option<u64>,
}

#[derive(Serialize, Debug)]
pub struct PistonExecuteInput {
    language: String,
    version: String,
    files: Vec<PistonExecuteInputFile>,
}

#[derive(Serialize, Clone, Debug)]
pub struct PistonExecuteInputFile {
    pub name: String,
    pub content: String,
    pub encoding: PistonExecuteInputFileEncoding,
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum PistonExecuteInputFileEncoding {
    Utf8,
    Base64,
}

#[derive(Deserialize, Debug)]
pub struct PistonExecuteOutput {
    pub run: PistonExecuteOutputRun,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct PistonExecuteOutputRun {
    pub stdout: String,
    pub stderr: String,
    pub output: String,
    pub code: Option<u64>,
}

impl Piston {
    pub fn new<T: Display>(runtime: &'static PistonRuntime, code: Option<T>) -> Self {
        Self { runtime, code: code.map(|code| code.to_string()), files: vec![], output: None, exit_code: None }
    }

    pub fn get_runtime<T: Display>(query: T) -> Option<&'static PistonRuntime> {
        let query = query.to_string().trim().to_lowercase();

        PISTON_RUNTIMES
            .get(&query)
            .or_else(|| PISTON_RUNTIMES.values().find(|runtime| runtime.aliases.contains(&query) || runtime.to_string() == query))
    }

    pub fn get_runtimes<T: Display>(query: T) -> Vec<&'static PistonRuntime> {
        let query = query.to_string().trim().to_lowercase();

        PISTON_RUNTIMES.values().filter(|runtime| runtime.aliases.contains(&query) || runtime.to_string().contains(&query)).collect()
    }

    pub fn autocomplete_runtime(input: &CommandInput) -> Vec<ApplicationCommandOptionChoice> {
        let Some(query) =
            input.focused.as_ref().and_then(|focused| input.args.get(focused)).and_then(|option_value| option_value.as_string())
        else {
            return vec![];
        };

        Self::get_runtimes(query)
            .iter()
            .map(|runtime| ApplicationCommandOptionChoice::new(runtime.to_string(), runtime.to_string()))
            .take(25)
            .collect()
    }

    pub fn add_file<T: Display, U: AsRef<[u8]>>(mut self, name: T, content: U) -> Self {
        self.files.push(PistonExecuteInputFile {
            name: name.to_string(),
            content: BASE64_STANDARD.encode(content),
            encoding: PistonExecuteInputFileEncoding::Base64,
        });

        self
    }

    pub async fn run(mut self) -> Result<Self> {
        if self.code.as_ref().is_some_and(|code| code.trim().is_empty()) {
            self.code = None;
        }

        let main_file = if let Some(input) = &self.code {
            PistonExecuteInputFile {
                name: format!("{}.code", Utc::now().timestamp()),
                content: input.clone(),
                encoding: PistonExecuteInputFileEncoding::Utf8,
            }
        } else if !self.files.is_empty() {
            let first_file = self.files.remove(0);
            let content = BASE64_STANDARD.decode(first_file.content).unwrap_or_default();
            PistonExecuteInputFile {
                name: first_file.name,
                content: String::from_utf8_lossy(&content).to_string(),
                encoding: PistonExecuteInputFileEncoding::Utf8,
            }
        } else {
            bail!("Please provide a code input or file.");
        };

        // Main file has to be the first file, so we insert from index 0
        self.files.insert(0, main_file);

        let input = PistonExecuteInput {
            language: self.runtime.language.clone(),
            version: self.runtime.version.clone(),
            files: self.files.clone(),
        };

        let body = REQWEST
            .post("https://emkc.org/api/v2/piston/execute")
            .header("authorization", &CONFIG.api.piston_key)
            .json(&input)
            .send()
            .await?
            .text()
            .await?;

        if let Ok(result) = from_str::<PistonExecuteOutput>(&body) {
            self.exit_code = result.run.code;
            self.output = Some(result.run.output.clone());
        } else {
            self.output = Some(body.clone());
        }

        Ok(self)
    }

    pub fn format_pagination<T: Display>(&self, command: T) -> Pagination {
        let output_pages = paginate_strings(self.output.as_deref().unwrap_or_default().chars(), "", 4089)
            .into_iter()
            .map(|entry| {
                let mut embed = Embed::new()
                    .set_color(PRIMARY_EMBED_COLOR)
                    .unwrap_or_default()
                    .set_title(self.runtime)
                    .set_description(format!("```\n{entry}```"));

                if let Some(exit_code) = self.exit_code {
                    embed = embed.set_footer(format!("Exit Code: {exit_code}"), None::<String>);
                }

                embed
            })
            .collect::<Vec<_>>();

        let code_pages = paginate_strings(self.files.first().map(|file| file.content.as_str()).unwrap_or_default().chars(), "", 4089)
            .into_iter()
            .map(|entry| {
                Embed::new()
                    .set_color(PRIMARY_EMBED_COLOR)
                    .unwrap_or_default()
                    .set_title(self.runtime)
                    .set_description(format!("```\n{entry}```"))
            })
            .collect::<Vec<_>>();

        Pagination::new(command).add_section("Output", output_pages).add_section("Code", code_pages)
    }
}

#[derive(Clone, Debug)]
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
}

impl Display for PistonRuntime {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "{} ({}v{})",
            self.language,
            self.runtime.as_ref().map(|runtime| format!("{runtime} ")).as_deref().unwrap_or_default(),
            self.version,
        )
    }
}
