pub mod statics;

use crate::{
    functions::eien,
    statics::{colors::PRIMARY_COLOR, REQWEST},
    structs::{
        api::google::{statics::GOOGLE_TRANSLATE_LANGUAGES, Google},
        ocr::statics::OCR_LANGUAGES,
    },
};
use anyhow::{Context, Result};
use serde_json::json;
use slashook::{
    commands::MessageResponse,
    structs::{embeds::Embed, utils::File as SlashookFile},
};
use std::{
    fs::{write, File},
    process::Stdio,
};
use tokio::{io::AsyncWriteExt, process::Command};

pub struct Ocr {
    pub image_url: String,
    pub language: String,
    pub text: String,
    pub visual_file: SlashookFile,
}

impl Ocr {
    pub async fn read<T: ToString, U: ToString, V: ToString>(image_url: T, origin_language: U, target_language: V) -> Result<Self> {
        let image_url = image_url.to_string();

        let (origin_language_code, origin_language_name) =
            OCR_LANGUAGES.get_key_value(origin_language.to_string().to_lowercase().as_str()).context("Invalid origin language.")?;

        let (target_language_code, target_language_name) = GOOGLE_TRANSLATE_LANGUAGES
            .get_key_value(target_language.to_string().to_lowercase().as_str())
            .context("Invalid target language.")?;

        let mut child = Command::new("tesseract")
            .args([
                "stdin",
                "stdout",
                "-c",
                "tessedit_create_tsv=1",
                "-c",
                "tessedit_create_txt=1",
                "--tessdata-dir",
                "../tessdata",
                "-l",
                origin_language_code,
                "--dpi",
                "150",
                "--psm",
                "3",
            ])
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
            println!("[OCR] {}", String::from_utf8(output.stderr)?);
        }

        let stdout = String::from_utf8(output.stdout)?.replace('\r', "");
        let (tsv, txt) = stdout.split_once("\n\n").context("Invalid image.")?;

        Ok(Self {
            image_url: image_url.clone(),
            language: match origin_language_name == target_language_name {
                true => origin_language_name.to_string(),
                false => format!("{origin_language_name} to {target_language_name}"),
            },
            text: match origin_language_name == target_language_name {
                true => txt.to_string(),
                false => {
                    Google::translate(
                        txt,
                        GOOGLE_TRANSLATE_LANGUAGES.iter().find(|(_, language_name)| language_name == &origin_language_name).unwrap().0, // Get key by value, jank
                        target_language_code,
                    )
                    .await?
                    .translation
                },
            },
            visual_file: eien("ocr", &[&json!({ "image": image_url, "tsv": tsv }).to_string()]).await?,
        })
    }

    pub fn format(&self) -> MessageResponse {
        let text = self.text.chars().take(4090).collect::<String>().trim().replace('`', "｀");

        MessageResponse::from(
            Embed::new()
                .set_color(PRIMARY_COLOR)
                .unwrap_or_default()
                .set_title(&self.language)
                .set_url(&self.image_url)
                .set_image("attachment://image.png")
                .set_description(format!(
                    "```{}```",
                    match text.is_empty() {
                        true => "<empty>",
                        false => &text,
                    },
                )),
        )
        .add_file(self.visual_file.clone())
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
