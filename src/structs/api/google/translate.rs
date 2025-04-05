use crate::{
    statics::REQWEST,
    structs::api::google::{
        Google,
        statics::{GOOGLE_EMBED_AUTHOR_ICON_URL, GOOGLE_EMBED_AUTHOR_URL, GOOGLE_EMBED_COLOR, GOOGLE_TRANSLATE_LANGUAGES},
    },
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

        Embed::new()
            .set_color(GOOGLE_EMBED_COLOR)
            .unwrap_or_default()
            .set_author("Google  •  Translate", Some(GOOGLE_EMBED_AUTHOR_URL), Some(GOOGLE_EMBED_AUTHOR_ICON_URL))
            .set_title(title)
            .set_description(translation)
    }
}

impl Google {
    pub async fn translate<T: Display, U: Display, V: Display>(
        text: T,
        origin_language: U,
        target_language: V,
    ) -> Result<GoogleTranslateTranslation> {
        let text = text.to_string();

        if text.is_empty() {
            bail!("Text is empty.");
        }

        let origin_language = origin_language.to_string().to_lowercase();
        let origin_language = GOOGLE_TRANSLATE_LANGUAGES
            .iter()
            .find(|(k, v)| [k.to_lowercase(), v.to_lowercase()].contains(&origin_language))
            .context("Invalid origin language.")?;

        let target_language = target_language.to_string().to_lowercase();
        let target_language = GOOGLE_TRANSLATE_LANGUAGES
            .iter()
            .find(|(k, v)| [k.to_lowercase(), v.to_lowercase()].contains(&target_language))
            .context("Invalid target language.")?;

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
                if *origin_language.0 == "auto" { " (detected)" } else { "" },
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
