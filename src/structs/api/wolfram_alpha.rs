use crate::statics::{CONFIG, REQWEST};
use anyhow::Result;
use serde::Deserialize;
use slashook::{
    commands::MessageResponse,
    structs::components::{Components, MediaGallery, MediaGalleryItem, TextDisplay},
};
use std::fmt::Display;

#[derive(Deserialize, Debug)]
pub struct WolframAlpha {
    #[serde(rename = "queryresult")]
    pub query_result: WolframAlphaQueryResult,
}

impl WolframAlpha {
    pub async fn query<T: Display>(query: T) -> Result<Self> {
        let query = query.to_string();

        let result = REQWEST
            .get("http://api.wolframalpha.com/v2/query")
            .query(&[("appid", CONFIG.api.wolfram_alpha_app_id.as_str()), ("input", query.as_str()), ("output", "json")])
            .send()
            .await?
            .json::<Self>()
            .await?;

        Ok(result)
    }

    pub fn format_response(&self) -> MessageResponse {
        let Some(pods) = &self.query_result.pods else {
            let components = Components::empty().add_component(TextDisplay::new("No results found."));
            let response = MessageResponse::from(components).set_components_v2(true);
            return response;
        };

        let mut components = Components::empty();

        for pod in pods {
            let Some(subpod) = pod.subpods.first() else { continue };

            let text_display = TextDisplay::new(format!(
                "# {}{}",
                pod.title,
                if !subpod.plain_text.is_empty() { format!("\n```\n{}```", subpod.plain_text) } else { "".into() },
            ));
            let media_gallery = MediaGallery::new().add_item(MediaGalleryItem::new(&subpod.img.src));

            components = components.add_component(text_display).add_component(media_gallery);
        }

        MessageResponse::from(components).set_components_v2(true)
    }
}

#[derive(Deserialize, Debug)]
pub struct WolframAlphaQueryResult {
    pub pods: Option<Vec<WolframAlphaQueryResultPod>>,
}

#[derive(Deserialize, Debug)]
pub struct WolframAlphaQueryResultPod {
    pub title: String,
    pub subpods: Vec<WolframAlphaQueryResultSubPod>,
}

#[derive(Deserialize, Debug)]
pub struct WolframAlphaQueryResultSubPod {
    pub img: WolframAlphaQueryResultSubPodImage,

    #[serde(rename = "plaintext")]
    pub plain_text: String,
}

#[derive(Deserialize, Debug)]
pub struct WolframAlphaQueryResultSubPodImage {
    pub src: String,
}
