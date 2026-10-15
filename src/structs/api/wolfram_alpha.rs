use crate::{
    statics::{
        CONFIG, REQWEST,
        colors::{PRIMARY_EMBED_COLOR, SUCCESS_EMBED_COLOR},
    },
    structs::pagination::Pagination,
};
use anyhow::Result;
use serde::Deserialize;
use slashook::structs::components::{Components, Container, MediaGallery, MediaGalleryItem, TextDisplay};
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

    pub fn format_pagination<T: Display>(&self, command: T) -> Pagination {
        let Some(pods) = &self.query_result.pods else {
            let components = Components::empty().add_component(TextDisplay::new("No results found."));
            return Pagination::new(command).add_section("Main", components.0);
        };

        // Sort pods by input interpretation and primary result first (if exists)
        let mut sorted_pods = vec![];

        for (index, pod) in pods.iter().enumerate() {
            // 0th index should usually be the input interpretation
            if index == 0 {
                sorted_pods.push(pod);
                continue;
            }

            // Always put the result after the input interpretation. The rest should be appended normally
            if pod.title == "Result" {
                sorted_pods.insert(1, pod);
            } else {
                sorted_pods.push(pod);
            }
        }

        let mut pages = vec![];
        let mut input_interpretation_container = None::<Container>;

        for (index, pod) in sorted_pods.iter().enumerate() {
            let Some(subpod) = pod.subpods.first() else { continue };

            let text_display = TextDisplay::new(format!(
                "# {}{}",
                pod.title,
                if !subpod.plain_text.is_empty() { format!("\n```\n{}```", subpod.plain_text) } else { "".into() },
            ));
            let media_gallery = MediaGallery::new().add_item(MediaGalleryItem::new(&subpod.img.src));

            let container = Container::new()
                .set_accent_color(if index == 0 { PRIMARY_EMBED_COLOR } else { SUCCESS_EMBED_COLOR })
                .unwrap_or_default()
                .add_component(text_display)
                .add_component(media_gallery);

            if let Some(input_interpretation_container) = &input_interpretation_container {
                let page = Components::empty().add_component(input_interpretation_container.clone()).add_component(container);
                pages.push(page.0);
            } else {
                input_interpretation_container = Some(container.clone());
                continue;
            }
        }

        Pagination::new("wolfram-alpha").add_section("Main", pages)
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
