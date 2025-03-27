use crate::{
    statics::{REQWEST, colors::PRIMARY_COLOR},
    structs::api::google::{Google, statics::GOOGLE_TRANSLATE_LANGUAGES},
};
use anyhow::{Context, Result, bail};
use serde::Deserialize;
use slashook::structs::embeds::Embed;
use std::fmt::Display;

#[derive(Deserialize, Debug)]
struct GoogleTranslateSentences {
    trans: String,
}

#[derive(Deserialize, Debug)]
struct GoogleTranslateResponse {
    sentences: Vec<GoogleTranslateSentences>,
    src: String,
}

#[derive(Debug)]
pub struct GoogleTranslateTranslation {
    pub origin_language: String,
    pub target_language: String,
    pub translation: String,
}

impl GoogleTranslateTranslation {
    pub fn format(&self) -> Embed {
        let title = format!("{} to {}", self.origin_language, self.target_language);
        let translation = self.translation.chars().take(4096).collect::<String>();

        Embed::new().set_color(PRIMARY_COLOR).unwrap_or_default().set_title(title).set_description(translation)
    }
}

impl Google {
    pub async fn translate<T: Display, U: Display, V: Display>(
        text: T,
        origin_language: U,
        target_language: V,
    ) -> Result<GoogleTranslateTranslation> {
        let text = text.to_string();
        let origin_language = origin_language.to_string();
        let target_language = target_language.to_string();

        if text.is_empty() {
            bail!("Text is empty.");
        }

        let origin_language =
            GOOGLE_TRANSLATE_LANGUAGES.get_key_value(origin_language.to_lowercase().as_str()).context("Invalid origin language.")?;

        let target_language =
            GOOGLE_TRANSLATE_LANGUAGES.get_key_value(target_language.to_lowercase().as_str()).context("Invalid target language.")?;

        let google_translate_response = REQWEST
            .get("https://translate.googleapis.com/translate_a/single")
            .query(&[
                ("client", "gtx"),
                ("dj", "1"),
                ("dt", "t"),
                ("sl", origin_language.0),
                ("tl", target_language.0),
                ("q", text.as_str()),
            ])
            .send()
            .await?
            .json::<GoogleTranslateResponse>()
            .await?;

        Ok(GoogleTranslateTranslation {
            origin_language: format!(
                "{}{}",
                GOOGLE_TRANSLATE_LANGUAGES.get(&google_translate_response.src.as_str()).context("Unexpected language code from API.")?,
                if origin_language.0 == &"auto" { " (detected)" } else { "" },
            ),
            target_language: target_language.1.to_string(),
            translation: google_translate_response
                .sentences
                .into_iter()
                .map(|sentence| sentence.trans) // 🏳️‍⚧️
                .collect::<Vec<String>>()
                .join(""),
        })
    }
}
