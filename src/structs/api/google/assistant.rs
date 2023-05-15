use crate::{
    statics::CONFIG,
    structs::api::google::{
        protos::{
            assistant::embedded::v1alpha2::{
                assist_config, assist_request, audio_out_config::Encoding, device_location,
                embedded_assistant_client::EmbeddedAssistantClient, screen_out_config::ScreenMode, AssistConfig, AssistRequest,
                AudioOutConfig, DeviceConfig, DeviceLocation, DialogStateIn, ScreenOutConfig,
            },
            r#type::LatLng,
        },
        Google,
    },
};
use anyhow::{bail, Result};
use chromiumoxide::{
    browser::{Browser, BrowserConfig},
    handler::viewport::Viewport,
    page::ScreenshotParams,
};
use futures::{stream, StreamExt};
use gouth::Builder;
use serde_json::json;
use tokio::{fs::read, spawn};
use tonic::{
    metadata::MetadataValue,
    transport::{Certificate, Channel, ClientTlsConfig},
    Request,
};

impl Google {
    pub async fn query_assistant<T: ToString>(query: T) -> Result<Vec<u8>> {
        let token = Builder::new()
            .json(
                json!({
                    "client_id": CONFIG.api.google_assistant.client_id,
                    "client_secret": CONFIG.api.google_assistant.client_secret,
                    "refresh_token": CONFIG.api.google_assistant.refresh_token,
                })
                .to_string(),
            )
            .scopes(&["https://www.googleapis.com/auth/assistant-sdk-prototype"])
            .build()?;

        let mut response = EmbeddedAssistantClient::with_interceptor(
            Channel::from_static("https://embeddedassistant.googleapis.com")
                .tls_config(
                    ClientTlsConfig::new()
                        .ca_certificate(Certificate::from_pem(read("google/roots.pem").await?.as_slice()))
                        .domain_name("embeddedassistant.googleapis.com"),
                )?
                .connect()
                .await?,
            move |mut req: Request<()>| {
                req.metadata_mut().insert("authorization", MetadataValue::try_from(&*token.header_value().unwrap()).unwrap());
                Ok(req)
            },
        )
        .assist(Request::new(stream::iter(vec![AssistRequest {
            r#type: Some(assist_request::Type::Config(AssistConfig {
                r#type: Some(assist_config::Type::TextQuery(query.to_string())),
                device_config: Some(DeviceConfig {
                    device_id: CONFIG.api.google_assistant.device_id.clone(),
                    device_model_id: CONFIG.api.google_assistant.device_model_id.clone(),
                }),
                dialog_state_in: Some(DialogStateIn {
                    conversation_state: vec![0],
                    language_code: "en-US".to_string(),
                    device_location: Some(DeviceLocation {
                        r#type: Some(device_location::Type::Coordinates(
                            LatLng { latitude: 37.895582, longitude: 41.0967176 }, // Batman, Türkiye
                        )),
                    }),
                    is_new_conversation: false,
                }),
                screen_out_config: Some(ScreenOutConfig { screen_mode: ScreenMode::Playing.into() }),
                audio_out_config: Some(AudioOutConfig {
                    encoding: Encoding::Linear16.into(),
                    sample_rate_hertz: 16000,
                    volume_percentage: 0,
                }),
                debug_config: None,
            })),
        }])))
        .await?
        .into_inner();

        while let Some(res) = response.message().await? {
            if let Some(screen_out) = res.screen_out {
                let (mut browser, mut handler) = Browser::launch(
                    BrowserConfig::builder()
                        .no_sandbox()
                        .viewport(Viewport {
                            width: 1920,
                            height: 1080,
                            device_scale_factor: None,
                            emulating_mobile: false,
                            is_landscape: true,
                            has_touch: false,
                        })
                        .build()
                        .unwrap(),
                )
                .await?;

                let handle = spawn(async move {
                    while let Some(handle) = handler.next().await {
                        if handle.is_err() {
                            break;
                        }
                    }
                });

                let page = browser.new_page("about:blank").await?;

                page.set_content(
                    String::from_utf8(screen_out.data)?
                        .replace("<html>", r#"<html style="background-image: url(https://picsum.photos/1920/1080);">"#)
                        .replace(r#"style="display:none""#, ""),
                )
                .await?;

                let screenshot = page.screenshot(ScreenshotParams::builder().build()).await?;

                browser.close().await?;
                handle.await?;

                return Ok(screenshot);
            }

            /*
            if let Some(dialog_state_out) = res.dialog_state_out {
                if !dialog_state_out.supplemental_display_text.is_empty() {
                    println!({}", dialog_state_out.supplemental_display_text);
                }
            }
            */
        }

        bail!("Could not get response.")
    }
}
