pub mod statics;

use crate::{
    functions::hastebin,
    statics::{colors::PRIMARY_COLOR, REQWEST},
    structs::api::tio::statics::TIO_PROGRAMMING_LANGUAGES,
};
use anyhow::{Context, Result};
use flate2::{
    write::{DeflateEncoder, GzDecoder},
    Compression,
};
use slashook::structs::embeds::Embed;
use std::io::Write;

pub struct Tio {
    pub programming_language: String,
    pub code: String,
    pub code_url: Option<String>,
    pub result: Option<String>,
    pub result_url: Option<String>,
}

impl Tio {
    pub fn new<T: ToString, U: ToString>(programming_language: T, code: U) -> Self {
        Self {
            programming_language: programming_language.to_string().to_lowercase(),
            code: code.to_string(),
            code_url: None,
            result: None,
            result_url: None,
        }
    }

    pub async fn run(mut self) -> Result<Self> {
        let (programming_language_id, programming_language_name) =
            TIO_PROGRAMMING_LANGUAGES.get_key_value(self.programming_language.as_str()).context("Invalid programming language.")?;

        // Set to real programming language name
        self.programming_language = programming_language_name.to_string();

        // Upload code to hastebin
        self.code_url = Some(hastebin(&self.code).await?);

        let mut body = vec![];

        DeflateEncoder::new(&mut body, Compression::default()).write_all(
            format!(
                "{}\0R",
                [
                    vec!["lang", "1", programming_language_id],
                    vec!["TIO_OPTIONS", "0"],
                    vec![".code.tio", &self.code.len().to_string(), &self.code],
                    vec![".input.tio", "0"],
                    vec!["args", "0"]
                ]
                .iter_mut()
                .map(|values| {
                    let key = values.remove(0);

                    format!(
                        "{}{key}\0{}",
                        match key.starts_with(".") {
                            true => "F",
                            false => "V",
                        },
                        values.join("\0")
                    )
                })
                .collect::<Vec<String>>()
                .join("\0")
            )
            .as_bytes(),
        )?;

        let mut result = vec![];

        GzDecoder::new(&mut result)
            .write_all(&REQWEST.post("https://tio.run/cgi-bin/run/api/").body(body).send().await?.bytes().await?.to_vec())?;

        let result = String::from_utf8(result)?;
        let result = result.replace(&result.chars().take(16).collect::<String>(), "");

        self.result = Some(result.clone());

        if result.len() > 3900 {
            self.result_url = Some(hastebin(result).await?);
        }

        Ok(self)
    }

    pub fn format(&self) -> Embed {
        Embed::new()
            .set_color(PRIMARY_COLOR)
            .unwrap_or_default()
            .set_title(&self.programming_language)
            .set_url(self.code_url.as_ref().unwrap_or(&"".into()))
            .set_description(format!(
                "{}```\n{}```",
                self.result_url.as_ref().map_or("".into(), |result_url| format!("[Full Result]({result_url})")),
                self.result.as_ref().unwrap_or(&"No output.".into()).chars().take(3900).collect::<String>(),
            ))
    }
}
