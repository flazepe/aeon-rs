use crate::constants::GOOGLE_TRANSLATE_LANGUAGES;
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

pub struct Translation {
    pub from_language: String,
    pub to_language: String,
    pub translation: String,
}

impl Translation {
    pub async fn get(text: &str, from_language: &str, to_language: &str) -> Result<Self> {
        if text.is_empty() {
            bail!("Text is empty.");
        }

        let from_language = GOOGLE_TRANSLATE_LANGUAGES
            .iter()
            .find(|[language, _]| language == &from_language.to_lowercase())
            .context("Invalid language.")?;

        let to_language = GOOGLE_TRANSLATE_LANGUAGES
            .iter()
            .find(|[language, _]| language == &to_language.to_lowercase())
            .context("Invalid language.")?;

        let google_translate_response  = get(format!("https://translate.googleapis.com/translate_a/single?client=gtx&dj=1&dt=t&sl={}&tl={}&q={text}", from_language[0], to_language[0])).await?.json::<GoogleTranslateResponse>().await?;

        let detected_language = GOOGLE_TRANSLATE_LANGUAGES
            .iter()
            .find(|[language, _]| language == &google_translate_response.src)
            .context("Unexpectd language code from API")?[1];

        Ok(Self {
            from_language: format!(
                "{detected_language}{}",
                if from_language[0] == "auto" {
                    " (detected)"
                } else {
                    ""
                }
            ),
            to_language: to_language[1].to_string(),
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
            .set_title(format!("{} to {}", self.from_language, self.to_language))
            .set_description(self.translation)
    }
}
