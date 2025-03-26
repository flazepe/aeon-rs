pub mod statics;

use crate::{
    functions::now,
    statics::{CACHE, CONFIG, REQWEST},
    structs::{
        api::ordr::statics::ORDR_SKINS,
        command_context::{AeonCommandContext, AeonCommandInput},
    },
};
use anyhow::{Result, bail};
use futures::FutureExt;
use rust_socketio::{Payload, asynchronous::ClientBuilder};
use serde::Deserialize;
use serde_json::{from_str, from_value, json};
use std::{fmt::Display, sync::Arc, time::Duration};
use tokio::time::sleep;

#[derive(Deserialize, Debug)]
pub struct OrdrRender {
    #[serde(rename = "renderID")]
    pub render_id: Option<u64>,

    pub message: String,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct OrdrWsRenderProgress {
    #[serde(rename = "renderID")]
    pub render_id: u64,

    pub username: String,
    pub renderer: String,
    pub progress: String,
    pub description: String,
}

#[derive(Deserialize, Debug)]
pub struct OrdrWsRenderDone {
    #[serde(rename = "renderID")]
    pub render_id: u64,

    #[serde(rename = "videoUrl")]
    pub video_url: String,
}

#[derive(Deserialize, Debug)]
pub struct OrdrWsRenderFailed {
    #[serde(rename = "renderID")]
    pub render_id: u64,

    #[serde(rename = "errorMessage")]
    pub error_message: String,
}

impl OrdrRender {
    pub async fn new<T: Display, U: Display>(replay_url: T, skin: Option<U>) -> Result<Self> {
        let skin = skin.map(|skin| skin.to_string());
        let mut skin = skin.as_deref().unwrap_or("");

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

        let Ok(render) = from_str::<Self>(text.as_str()) else { bail!(text) }; // Sometimes it returns the error as plain text, so we just send the text as the error

        // If render_id is None, then message should be returned as it would contain the error message
        if render.render_id.is_none() {
            bail!(render.message);
        }

        Ok(render)
    }

    #[allow(dead_code)]
    pub async fn poll_progress(&self, ctx: Arc<AeonCommandContext>) -> Result<()> {
        let AeonCommandInput::ApplicationCommand(input, res) = &ctx.command_input else { return Ok(()) };

        CACHE.ordr_renders.write().unwrap().insert(self.render_id.unwrap(), "Rendering... (0%)".into());
        CACHE.ordr_rendering_users.write().unwrap().insert(input.user.id.clone(), true);

        let start_time = now();

        let mut renders = CACHE.ordr_renders.read().unwrap().clone();
        let mut state = renders.get(&self.render_id.unwrap()).unwrap();

        while
        // 8 minutes timeout
        now() - start_time < 480 &&
            // Break if the state is no longer a progress
            state.contains('%')
        {
            renders = CACHE.ordr_renders.read().unwrap().clone();
            state = renders.get(&self.render_id.unwrap()).unwrap();

            res.edit_original_message(state.clone()).await?;

            // To prevent rate limits
            sleep(Duration::from_secs(3)).await;
        }

        CACHE.ordr_renders.write().unwrap().remove(&self.render_id.unwrap());
        CACHE.ordr_rendering_users.write().unwrap().remove(&input.user.id);

        Ok(())
    }

    pub async fn connect() -> Result<()> {
        ClientBuilder::new("https://apis.issou.best")
            .namespace("/ordr/ws")
            .on("render_progress_json", |payload, _| {
                async move {
                    let Payload::Text(mut values) = payload else { return };
                    let value = if values.is_empty() { return } else { values.remove(0) };
                    let render = from_value::<OrdrWsRenderProgress>(value).unwrap();

                    if render.progress.contains("0%") && CACHE.ordr_renders.read().unwrap().contains_key(&render.render_id) {
                        CACHE.ordr_renders.write().unwrap().insert(render.render_id, render.progress);
                    }
                }
                .boxed()
            })
            .on("render_done_json", |payload, _| {
                async move {
                    let Payload::Text(mut values) = payload else { return };
                    let value = if values.is_empty() { return } else { values.remove(0) };
                    let render = from_value::<OrdrWsRenderDone>(value).unwrap();

                    if CACHE.ordr_renders.read().unwrap().contains_key(&render.render_id) {
                        CACHE.ordr_renders.write().unwrap().insert(render.render_id, render.video_url);
                    }
                }
                .boxed()
            })
            .on("render_failed_json", |payload, _| {
                async move {
                    let Payload::Text(mut values) = payload else { return };
                    let value = if values.is_empty() { return } else { values.remove(0) };
                    let render = from_value::<OrdrWsRenderFailed>(value).unwrap();

                    if CACHE.ordr_renders.read().unwrap().contains_key(&render.render_id) {
                        CACHE.ordr_renders.write().unwrap().insert(render.render_id, render.error_message);
                    }
                }
                .boxed()
            })
            .reconnect_on_disconnect(true)
            .connect()
            .await?;

        Ok(())
    }
}
