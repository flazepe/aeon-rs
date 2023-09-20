pub mod statics;

use crate::{
    statics::{colors::PRIMARY_COLOR, REQWEST},
    structs::ocr::statics::OCR_LANGUAGES,
};
use anyhow::{bail, Context, Result};
use slashook::structs::embeds::Embed;
use std::{
    fs::{write, File},
    process::Stdio,
};
use tokio::{io::AsyncWriteExt, process::Command};

pub struct Ocr {
    pub image_url: String,
    pub language: String,
    pub text: String,
}

impl Ocr {
    pub async fn read<T: ToString, U: ToString>(image_url: T, language: U) -> Result<Self> {
        let image_url = image_url.to_string();

        let (language_code, language_name) =
            OCR_LANGUAGES.get_key_value(language.to_string().to_lowercase().as_str()).context("Invalid language.")?;

        let mut child = Command::new("tesseract")
            .args(["stdin", "stdout", "--tessdata-dir", "../tessdata", "-l", language_code, "--dpi", "150", "--psm", "6"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        child
            .stdin
            .take()
            .unwrap()
            .write_all(&REQWEST.get("https://external-content.duckduckgo.com/iu/").query(&[("u", &image_url)]).send().await?.bytes().await?)
            .await?;

        let output = child.wait_with_output().await?;

        if !output.stderr.is_empty() {
            bail!("Invalid image.");
        }

        Ok(Self { image_url, language: language_name.to_string(), text: String::from_utf8(output.stdout)? })
    }

    pub fn format(&self) -> Embed {
        let text = self.text.chars().take(4090).collect::<String>().trim().replace('`', "｀");

        Embed::new().set_color(PRIMARY_COLOR).unwrap_or_default().set_title(&self.language).set_url(&self.image_url).set_description(
            format!(
                "```{}```",
                match text.is_empty() {
                    true => "<empty>",
                    false => &text,
                },
            ),
        )
    }

    #[allow(dead_code)]
    pub async fn download_trained_data() -> Result<()> {
        for (index, (language_code, language_name)) in OCR_LANGUAGES.iter().enumerate() {
            let path = format!("../tessdata/{language_code}.traineddata");

            if File::open(&path).is_err() {
                write(
                    path,
                    REQWEST
                        .get(format!("https://github.com/tesseract-ocr/tessdata/raw/main/{language_code}.traineddata"))
                        .send()
                        .await?
                        .bytes()
                        .await?,
                )?;
            }

            println!("[OCR] [{}/{}] Downloaded {language_name} trained data.", index + 1, OCR_LANGUAGES.len());
        }

        println!("[OCR] Downloaded all trained data.");

        Ok(())
    }
}
