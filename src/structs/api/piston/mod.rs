pub mod statics;

use crate::{
    functions::paginate_strings,
    statics::{CONFIG, REQWEST, colors::PRIMARY_EMBED_COLOR},
    structs::{api::piston::statics::PISTON_RUNTIMES, pagination::Pagination},
};
use anyhow::{Context, Result, bail};
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
    pub stdin: Option<String>,
    pub args: Option<String>,
    pub files: Vec<PistonExecuteInputFile>,
    pub output: Option<PistonExecuteOutputRun>,
}

#[derive(Serialize, Debug)]
pub struct PistonExecuteInput {
    language: String,
    version: String,
    files: Vec<PistonExecuteInputFile>,
    stdin: Option<String>,
    args: Vec<String>,
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
pub struct PistonExecuteOutputRun {
    pub stdout: String,
    pub stderr: String,
    pub output: String,
    pub code: Option<u64>,
}

impl Piston {
    pub fn new<T: Display, U: Display, V: Display>(
        runtime: &'static PistonRuntime,
        code: Option<T>,
        stdin: Option<U>,
        args: Option<V>,
    ) -> Self {
        Self {
            runtime,
            code: code.map(|code| code.to_string()),
            stdin: stdin.map(|stdin| stdin.to_string()),
            args: args.map(|args| args.to_string()),
            files: vec![],
            output: None,
        }
    }

    pub fn get_runtime<T: Display>(query: T) -> Option<&'static PistonRuntime> {
        let query = query.to_string().trim().to_lowercase();

        PISTON_RUNTIMES.get(&query).or_else(|| {
            PISTON_RUNTIMES.values().find(|runtime| {
                (runtime.runtime.is_none() && runtime.language == query) || runtime.to_string() == query || runtime.aliases.contains(&query)
            })
        })
    }

    pub fn get_runtimes<T: Display>(query: T) -> Vec<&'static PistonRuntime> {
        let query = query.to_string().trim().to_lowercase();

        PISTON_RUNTIMES
            .values()
            .filter(|runtime| {
                (runtime.runtime.is_none() && runtime.language.contains(&query))
                    || runtime.to_string().contains(&query)
                    || runtime.aliases.iter().any(|alias| alias.contains(&query))
            })
            .collect()
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
            stdin: self.stdin.clone(),
            args: self.args.as_ref().and_then(|args| from_str::<Vec<String>>(args).ok()).unwrap_or_default(),
        };

        let body = REQWEST
            .post("https://emkc.org/api/v2/piston/execute")
            .header("authorization", &CONFIG.api.piston_key)
            .json(&input)
            .send()
            .await?
            .text()
            .await?;

        let output = from_str::<PistonExecuteOutput>(&body).context(format!("An error occurred: {body}"))?;
        self.output = Some(output.run);

        Ok(self)
    }

    pub fn format_pagination<T: Display>(&self, command: T) -> Pagination {
        let paginate = |chars| {
            paginate_strings(chars, "", 4089)
                .into_iter()
                .map(|entry| {
                    let mut embed = Embed::new()
                        .set_color(PRIMARY_EMBED_COLOR)
                        .unwrap_or_default()
                        .set_title(self.runtime)
                        .set_description(format!("```\n{entry}```"));

                    if let Some(exit_code) = self.output.as_ref().and_then(|output| output.code) {
                        embed = embed.set_footer(format!("Exit Code: {exit_code}"), None::<String>);
                    }

                    embed
                })
                .collect::<Vec<_>>()
        };

        let output = self.output.as_ref().map(|output| output.output.as_str()).unwrap_or_default();
        let output_pages = paginate(output.chars());

        let stdout = self.output.as_ref().map(|output| output.stdout.as_str()).unwrap_or_default();
        let stdout_pages = paginate(stdout.chars());

        let stderr = self.output.as_ref().map(|output| output.stderr.as_str()).unwrap_or_default();
        let stderr_pages = paginate(stderr.chars());

        let code_pages = paginate(self.files.first().map(|file| file.content.as_str()).unwrap_or_default().chars());

        Pagination::new(command)
            .add_section("Output", output_pages)
            .add_section("stdout", stdout_pages)
            .add_section("stderr", stderr_pages)
            .add_section("Code", code_pages)
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

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct PistonState {
    pub runtime: Option<String>,
    pub code: Option<String>,
    pub stdin: Option<String>,
    pub args: Option<String>,
}
