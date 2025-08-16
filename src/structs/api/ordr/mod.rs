pub mod statics;

use crate::{
    functions::now,
    statics::{CACHE, CONFIG, REQWEST},
    structs::{
        api::ordr::statics::ORDR_SKINS,
        command_context::{AeonCommandContext, AeonCommandInput},
    },
};
use anyhow::{Context, Result, bail};
use serde::Deserialize;
use serde_json::{from_str, json};
use std::{fmt::Display, sync::Arc, time::Duration};
use tokio::time::sleep;

#[derive(Deserialize, Debug)]
pub struct OrdrRenders {
    pub renders: Vec<OrdrRender>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrdrRender {
    #[serde(rename = "renderID")]
    pub render_id: u64,

    pub progress: String,
    pub error_code: u64,
    pub video_url: String,
}

#[derive(Deserialize, Debug)]
pub struct OrdrRenderSubmit {
    #[serde(rename = "renderID")]
    pub render_id: Option<u64>,

    pub message: String,
}

impl OrdrRender {
    pub async fn new<T: Display, U: Display>(replay_url: T, skin: Option<U>) -> Result<Self> {
        let skin = skin.map(|skin| skin.to_string());
        let mut skin = skin.as_deref().unwrap_or_default();

        if !ORDR_SKINS.contains_key(skin) {
            skin = "whitecat_2_1_old_ck";
        }

        let text = REQWEST
            .post("https://apis.issou.best/ordr/renders")
            .json(&json!({
                "replayURL": replay_url.to_string(),
                "username": "Aeon",
                "resolution": "1920x1080",

                "skin": skin,
                "useSkinColors": "true",
                "useBeatmapColors": "false",

                "introBGDim": "100",
                "inGameBGDim": "100",
                "breakBGDim": "50",

                "loadStoryboard": "false",
                "sliderSnakingOut": "false",

                "showDanserLogo": "false",
                "showHitCounter": "true",

                "verificationKey": CONFIG.api.ordr_key,
            }))
            .send()
            .await?
            .text()
            .await?;

        let render = from_str::<OrdrRenderSubmit>(text.as_str()).context(text)?; // Sometimes it returns the error as plain text, so we just send the text as the error

        // If render_id is None, then message should be returned as it would contain the error message
        let Some(render_id) = render.render_id else {
            bail!(render.message);
        };

        Self::get_render(render_id).await
    }

    pub async fn poll_progress(&self, ctx: Arc<AeonCommandContext>) -> Result<()> {
        let AeonCommandInput::ApplicationCommand(input, res) = &ctx.command_input else { return Ok(()) };

        res.send_message("In queue...").await?;

        let poll = async || -> Result<()> {
            sleep(Duration::from_secs(5)).await;

            let start_time = now();

            while now() - start_time < 480 {
                let render = Self::get_render(self.render_id).await?;

                if render.error_code != 0 {
                    bail!(render.progress.clone());
                }

                if render.video_url.starts_with("https://") {
                    res.edit_original_message(render.video_url.as_str()).await?;
                    break;
                } else {
                    res.edit_original_message(render.progress.as_str()).await?;
                    sleep(Duration::from_secs(5)).await;
                }
            }

            Ok(())
        };

        CACHE.ordr_rendering_users.write().unwrap().insert(input.user.id.clone(), true);

        match poll().await {
            Ok(()) => {
                CACHE.ordr_rendering_users.write().unwrap().remove(&input.user.id);
                Ok(())
            },
            Err(error) => {
                CACHE.ordr_rendering_users.write().unwrap().remove(&input.user.id);
                res.edit_original_message(format!("{error:?}").trim_matches('"')).await?;
                bail!(error)
            },
        }
    }

    pub async fn get_render(render_id: u64) -> Result<Self> {
        let json = REQWEST
            .get("https://apis.issou.best/ordr/renders")
            .query(&[("renderID", render_id)])
            .send()
            .await?
            .json::<OrdrRenders>()
            .await?;

        json.renders.into_iter().find(|render| render.render_id == render_id).context("Could not get render.")
    }
}
