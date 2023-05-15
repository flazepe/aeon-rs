use crate::{
    statics::CONFIG,
    structs::api::google::{
        protos::{
            assistant::embedded::v1alpha2::{
                assist_config, assist_request, device_location, embedded_assistant_client::EmbeddedAssistantClient, AssistConfig,
                AssistRequest, AudioOutConfig, DeviceConfig, DeviceLocation, DialogStateIn, ScreenOutConfig,
            },
            r#type::LatLng,
        },
        Google,
    },
};
use anyhow::{bail, Result};
use futures::stream;
use gouth::Builder;
use headless_chrome::{protocol::cdp::Page::CaptureScreenshotFormatOption, Browser, LaunchOptionsBuilder};
use serde_json::json;
use tokio::fs::read;
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
            .build()
            .unwrap();

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
                screen_out_config: {
                    Some(ScreenOutConfig {
                        screen_mode: 3, // https://github.com/googleapis/googleapis/blob/master/google/assistant/embedded/v1alpha2/embedded_assistant.proto#L276
                    })
                },
                audio_out_config: Some(AudioOutConfig { encoding: 1, sample_rate_hertz: 16000, volume_percentage: 0 }),
                debug_config: None,
            })),
        }])))
        .await?
        .into_inner();

        while let Some(res) = response.message().await? {
            if let Some(screen_out) = res.screen_out {
                let browser = Browser::new(LaunchOptionsBuilder::default().window_size(Some((1920, 1080))).build()?)?;
                let tab = browser.new_tab()?;

                tab.navigate_to("https://picsum.photos/1920/1080")?;

                tab.evaluate(
                    format!(
                        r#"document.querySelector("html").innerHTML += `{}`;"#,
                        String::from_utf8(screen_out.data)?.replace(r#"style="display:none""#, ""),
                    )
                    .as_str(),
                    false,
                )?;

                return Ok(tab.capture_screenshot(CaptureScreenshotFormatOption::Png, None, None, true)?);
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
