pub mod statics;

use crate::{
    statics::{CACHE, CONFIG, REQWEST},
    structs::api::ordr::statics::ORDR_SKINS,
};
use anyhow::{bail, Result};
use futures::FutureExt;
use serde::Deserialize;
use serde_json::{from_str, from_value, json};
use slashook::commands::{CommandInput, CommandResponder};
use socketio_rs::{ClientBuilder, Payload};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::time::sleep;

#[derive(Deserialize)]
pub struct OrdrRender {
    #[serde(rename = "renderID")]
    pub render_id: Option<u64>,

    pub message: String,
}

#[derive(Deserialize, Debug)]
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

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrdrWsRenderFailed {
    #[serde(rename = "renderID")]
    pub render_id: u64,

    #[serde(rename = "errorMessage")]
    pub error_message: String,
}

impl OrdrRender {
    pub async fn new<T: ToString, U: ToString>(replay_url: T, skin: Option<U>) -> Result<Self> {
        let mut skin = skin.map(|skin| skin.to_string()).unwrap_or("".into());

        if !ORDR_SKINS.contains_key(skin.as_str()) {
            skin = "whitecat_2_1_old_ck".into();
        }

        let text = REQWEST
            .post("https://apis.issou.best/ordr/renders")
            .header("content-type", "application/json")
            .body(
                json!({
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
                })
                .to_string(),
            )
            .send()
            .await?
            .text()
            .await?;

        match from_str::<Self>(text.as_str()) {
            Ok(render) => {
                // If render_id is None, then message should be returned as it would contain the error message
                if render.render_id.is_none() {
                    bail!(render.message);
                }

                Ok(render)
            },
            Err(_) => bail!(text), // Sometimes it returns the error as plain text, so we just send the text as the error
        }
    }

    pub async fn poll_progress(&self, input: &CommandInput, res: &CommandResponder) -> Result<()> {
        CACHE.ordr_renders.write().unwrap().insert(self.render_id.unwrap(), "Rendering... (0%)".into());
        CACHE.ordr_rendering_users.write().unwrap().insert(input.user.id.clone(), true);

        let start_time = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();

        let mut renders = CACHE.ordr_renders.read().unwrap().clone();
        let mut state = renders.get(&self.render_id.unwrap()).unwrap();

        while
        // 8 minutes timeout
        SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() - start_time < 480 &&
            // Break if the state is no longer a progress
            state.contains("%")
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
        ClientBuilder::new("https://ordr-ws.issou.best")
            .on("render_progress_json", |payload, _, _| {
                async {
                    match payload {
                        Some(Payload::Json(value)) => {
                            let render = from_value::<OrdrWsRenderProgress>(value).unwrap();

                            if render.progress.contains("0%") && CACHE.ordr_renders.read().unwrap().contains_key(&render.render_id) {
                                CACHE.ordr_renders.write().unwrap().insert(render.render_id, render.progress);
                            }
                        },
                        _ => {},
                    }
                }
                .boxed()
            })
            .on("render_done_json", |payload, _, _| {
                async {
                    match payload {
                        Some(Payload::Json(value)) => {
                            let render = from_value::<OrdrWsRenderDone>(value).unwrap();

                            if CACHE.ordr_renders.read().unwrap().contains_key(&render.render_id) {
                                CACHE.ordr_renders.write().unwrap().insert(render.render_id, render.video_url);
                            }
                        },
                        _ => {},
                    }
                }
                .boxed()
            })
            .on("render_failed_json", |payload, _, _| {
                async {
                    match payload {
                        Some(Payload::Json(value)) => {
                            let render = from_value::<OrdrWsRenderFailed>(value).unwrap();

                            if CACHE.ordr_renders.read().unwrap().contains_key(&render.render_id) {
                                CACHE.ordr_renders.write().unwrap().insert(render.render_id, render.error_message);
                            }
                        },
                        _ => {},
                    }
                }
                .boxed()
            })
            .connect()
            .await?;

        Ok(())
    }
}
