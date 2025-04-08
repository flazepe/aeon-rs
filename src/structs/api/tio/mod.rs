pub mod statics;

use crate::{
    functions::hastebin,
    statics::{REQWEST, colors::PRIMARY_EMBED_COLOR},
    structs::api::tio::statics::TIO_PROGRAMMING_LANGUAGES,
};
use anyhow::{Context, Result};
use flate2::{
    Compression,
    write::{DeflateEncoder, GzDecoder},
};
use slashook::structs::embeds::Embed;
use statics::TIO_PROGRAMMING_LANGUAGE_CODES;
use std::{fmt::Display, io::Write};

#[derive(Debug)]
pub struct Tio {
    pub programming_language: String,
    pub code: String,
    pub code_url: Option<String>,
    pub output: Option<String>,
    pub output_url: Option<String>,
}

impl Tio {
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
        // Turn language codes to IDs
        if let Some((_, programming_language)) = TIO_PROGRAMMING_LANGUAGE_CODES.get_key_value(self.programming_language.as_str()) {
            self.programming_language = programming_language.to_string();
        }

        let (programming_language_id, programming_language_name) =
            TIO_PROGRAMMING_LANGUAGES.get_key_value(self.programming_language.as_str()).context("Invalid programming language.")?;

        self.programming_language = programming_language_name.to_string();
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
                    vec!["args", "0"],
                ]
                .iter_mut()
                .map(|values| {
                    let key = values.remove(0);
                    format!("{}{key}\0{}", if key.starts_with('.') { 'F' } else { 'V' }, values.join("\0"))
                })
                .collect::<Vec<String>>()
                .join("\0"),
            )
            .as_bytes(),
        )?;

        let mut output = vec![];
        GzDecoder::new(&mut output).write_all(&REQWEST.post("https://tio.run/cgi-bin/run/api/").body(body).send().await?.bytes().await?)?;

        let output = String::from_utf8_lossy(&output);
        let output = output.replace(&output.chars().take(16).collect::<String>(), "");

        self.output = Some(output.clone());

        if output.len() > 3900 {
            self.output_url = Some(hastebin(output).await?);
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
