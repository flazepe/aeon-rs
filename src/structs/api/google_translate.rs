use crate::{
    macros::if_else,
    statics::{colors::PRIMARY_COLOR, google_translate_languages::GOOGLE_TRANSLATE_LANGUAGES},
};
use anyhow::{bail, Context, Result};
use reqwest::get;
use serde::Deserialize;
use slashook::structs::embeds::Embed;

#[derive(Deserialize)]
pub struct GoogleTranslateSentences {
    pub trans: String,
    pub orig: String,
}

#[derive(Deserialize)]
pub struct GoogleTranslateResponse {
    pub sentences: Vec<GoogleTranslateSentences>,
    pub src: String,
}

pub struct GoogleTranslate {
    pub origin_language: String,
    pub target_language: String,
    pub translation: String,
}

impl GoogleTranslate {
    pub async fn translate<T: ToString, U: ToString, V: ToString>(
        text: T,
        origin_language: U,
        target_language: V,
    ) -> Result<Self> {
        let text = text.to_string();
        let origin_language = origin_language.to_string();
        let target_language = target_language.to_string();

        if text.is_empty() {
            bail!("Text is empty.");
        }

        let origin_language = GOOGLE_TRANSLATE_LANGUAGES
            .get_key_value(origin_language.to_lowercase().as_str())
            .context("Invalid origin language.")?;

        let target_language = GOOGLE_TRANSLATE_LANGUAGES
            .get_key_value(target_language.to_lowercase().as_str())
            .context("Invalid target language.")?;

        let google_translate_response = get(format!(
            "https://translate.googleapis.com/translate_a/single?client=gtx&dj=1&dt=t&sl={}&tl={}&q={text}",
            origin_language.0, target_language.0
        ))
        .await?
        .json::<GoogleTranslateResponse>()
        .await?;

        Ok(Self {
            origin_language: format!(
                "{}{}",
                GOOGLE_TRANSLATE_LANGUAGES
                    .get(&google_translate_response.src.as_str())
                    .context("Unexpected language code from API.")?,
                if_else!(origin_language.0 == &"auto", " (detected)", "")
            ),
            target_language: target_language.1.to_string(),
            translation: google_translate_response
                .sentences
                .into_iter()
                .map(|sentence| sentence.trans) // 🏳️‍⚧️
                .collect::<Vec<String>>()
                .join("")
                .chars()
                .take(4000)
                .collect::<String>(),
        })
    }

    pub fn format(self) -> Embed {
        Embed::new()
            .set_color(PRIMARY_COLOR)
            .unwrap_or_default()
            .set_title(format!("{} to {}", self.origin_language, self.target_language))
            .set_description(self.translation)
    }
}
