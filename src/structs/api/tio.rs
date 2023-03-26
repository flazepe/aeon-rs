use crate::{
    statics::{colors::*, tio_programming_languages::*},
    *,
};
use anyhow::{Context, Result};
use flate2::{write::*, Compression};
use reqwest::Client;
use slashook::structs::embeds::Embed;
use std::io::prelude::*;

pub struct TioProgrammingLanguage<'a> {
    pub name: &'a str,
    pub id: &'a str,
    pub alias: &'a [&'a str],
}

pub struct Tio {
    pub programming_language: String,
    pub code: String,
    pub result: Option<String>,
}

impl Tio {
    pub fn new<T: ToString, U: ToString>(programming_language: T, code: U) -> Self {
        Self {
            programming_language: programming_language.to_string().to_lowercase(),
            code: code.to_string(),
            result: None,
        }
    }

    pub async fn run(mut self) -> Result<Self> {
        let programming_language = TIO_PROGRAMMING_LANGUAGES
            .iter()
            .find(|entry| {
                entry.id == &self.programming_language
                    || entry.name.to_lowercase().contains(&self.programming_language)
                    || entry.alias.contains(&self.programming_language.as_str())
            })
            .context("Invalid programming language.")?;

        // Set to real programming language name
        self.programming_language = programming_language.name.to_string();

        let mut body = vec![];

        DeflateEncoder::new(&mut body, Compression::default()).write_all(
            format!(
                "{}\0R",
                [
                    vec!["lang", "1", programming_language.id],
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
                        if_else!(key.starts_with("."), "F", "V"),
                        values.join("\0")
                    )
                })
                .collect::<Vec<String>>()
                .join("\0")
            )
            .as_bytes(),
        )?;

        let mut result = vec![];

        GzDecoder::new(&mut result).write_all(
            &Client::new()
                .post("https://tio.run/cgi-bin/run/api/")
                .body(body)
                .send()
                .await?
                .bytes()
                .await?
                .to_vec(),
        )?;

        let result = String::from_utf8(result)?;
        self.result = Some(result.replace(&result.chars().take(16).collect::<String>(), ""));

        Ok(self)
    }

    pub fn format(self) -> Embed {
        Embed::new()
            .set_color(PRIMARY_COLOR)
            .unwrap_or_default()
            .set_title(self.programming_language)
            .set_description(format!(
                "```\n{}```",
                self.result
                    .unwrap_or("No output.".into())
                    .chars()
                    .take(3993)
                    .collect::<String>()
            ))
    }
}
