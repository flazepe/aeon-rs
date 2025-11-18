use crate::{
    statics::REQWEST,
    structs::api::google::{
        Google,
        statics::{GOOGLE_EMBED_AUTHOR_ICON_URL, GOOGLE_EMBED_AUTHOR_URL, GOOGLE_EMBED_COLOR, GOOGLE_TRANSLATE_LANGUAGES},
    },
};
use anyhow::{Context, Result, bail};
use serde_json::json;
use slashook::structs::embeds::Embed;
use std::fmt::Display;

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
            .post("https://translate-pa.googleapis.com/v1/translateHtml")
            .header("content-type", "application/json+protobuf")
            .header("x-goog-api-key", "AIzaSyATBXajvzQLTDHEQbcpq0Ihe0vWDHmO520")
            .body(json!([[[text], origin_language.0, target_language.0], "wt_lib"]).to_string())
            .send()
            .await?
            .json::<((String,), (String,))>()
            .await?;

        Ok(GoogleTranslateTranslation {
            origin_language: format!(
                "{}{}",
                GOOGLE_TRANSLATE_LANGUAGES.get(google_translate_response.1.0.as_str()).context("Unexpected language code from API.")?,
                if *origin_language.0 == "auto" { " (detected)" } else { "" },
            ),
            target_language: target_language.1.to_string(),
            translation: google_translate_response.0.0,
        })
    }
}
