use crate::statics::REQWEST;
use anyhow::Result;
use serde::Serialize;
use slashook::structs::utils::File;
use std::fmt::Display;

pub struct LaTeX;

#[derive(Serialize)]
pub struct LaTeXOptions {
    pub formula: String,

    #[serde(rename = "fsize")]
    pub font_size: String,

    #[serde(rename = "bcolor")]
    pub background_color: String,

    #[serde(rename = "fcolor")]
    pub font_color: String,

    pub out: u8,
    pub preamble: String,
}

impl LaTeX {
    pub async fn render<T: Display, U: Display>(expression: T, preamble: Option<U>) -> Result<File> {
        let body = REQWEST
            .post("https://www.quicklatex.com/latex3.f")
            .form(&LaTeXOptions {
                formula: expression.to_string(),
                font_size: "99px".into(),
                background_color: "ffffff".into(),
                font_color: "000000".into(),
                out: 1,
                preamble: preamble.map_or_else(
                    || "\\usepackage{amsmath}\\usepackage{amsfonts}\\usepackage{amssymb}".into(),
                    |preamble| preamble.to_string(),
                ),
            })
            .send()
            .await?
            .text()
            .await?;

        let bytes = REQWEST.get(body.chars().skip(3).take(75).collect::<String>()).send().await?.bytes().await?;

        Ok(File::new("image.png", bytes))
    }
}
