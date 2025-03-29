use crate::{
    statics::CONFIG,
    structs::api::google::{
        Google,
        protos::{
            assistant::embedded::v1alpha2::{
                AssistConfig, AssistRequest, AudioOutConfig, DeviceConfig, DeviceLocation, DialogStateIn, ScreenOutConfig,
                assist_config::Type as AssistConfigType, assist_request::Type as AssistRequestType, audio_out_config::Encoding,
                device_location::Type as DeviceLocationType, embedded_assistant_client::EmbeddedAssistantClient,
                screen_out_config::ScreenMode,
            },
            r#type::LatLng,
        },
    },
};
use anyhow::{Result, bail};
use futures::stream::iter;
use gouth::Builder;
use headless_chrome::{Browser, LaunchOptions, protocol::cdp::Page::CaptureScreenshotFormatOption};
use serde_json::json;
use std::fmt::Display;
use tokio::fs::read;
use tonic::{
    Request,
    metadata::MetadataValue,
    transport::{Certificate, Channel, ClientTlsConfig},
};

pub struct GoogleAssistant {
    pub card_image: Vec<u8>,
    pub suggestions: Vec<String>,
}

impl GoogleAssistant {
    async fn get_html_content<T: Display>(query: T) -> Result<String> {
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
        let certificate = Certificate::from_pem(read("google/gtsr1.pem").await?.as_slice());
        let tls_config =
            ClientTlsConfig::new().ca_certificate(certificate).with_native_roots().domain_name("embeddedassistant.googleapis.com");
        let interceptor = move |mut req: Request<()>| {
            req.metadata_mut().insert("authorization", MetadataValue::try_from(&*token.header_value().unwrap()).unwrap());
            Ok(req)
        };
        let mut client = EmbeddedAssistantClient::with_interceptor(
            Channel::from_static("https://embeddedassistant.googleapis.com").tls_config(tls_config)?.connect().await?,
            interceptor,
        );
        let query = Self::generate_request(query);
        let mut response = client.assist(Request::new(iter(vec![query]))).await?.into_inner();

        while let Some(message) = response.message().await? {
            if let Some(screen_out) = message.screen_out {
                return Ok(String::from_utf8(screen_out.data)?);
            }
        }

        bail!("Could not get response.");
    }

    fn generate_request<T: Display>(query: T) -> AssistRequest {
        let config_type: AssistConfigType = AssistConfigType::TextQuery(query.to_string());
        let device_config = DeviceConfig {
            device_id: CONFIG.api.google_assistant.device_id.clone(),
            device_model_id: CONFIG.api.google_assistant.device_model_id.clone(),
        };
        let device_coordinates = LatLng { latitude: 37.895582, longitude: 41.0967176 }; // Batman, Türkiye
        let device_location = DeviceLocation { r#type: Some(DeviceLocationType::Coordinates(device_coordinates)) };
        let dialog_state_in = DialogStateIn {
            conversation_state: vec![],
            language_code: "en-US".to_string(),
            device_location: Some(device_location),
            is_new_conversation: false,
        };
        let screen_out_config = ScreenOutConfig { screen_mode: ScreenMode::Playing.into() };
        let audio_out_config = AudioOutConfig { encoding: Encoding::Linear16.into(), sample_rate_hertz: 16000, volume_percentage: 0 };
        let assist_config = AssistConfig {
            r#type: Some(config_type),
            device_config: Some(device_config),
            dialog_state_in: Some(dialog_state_in),
            screen_out_config: Some(screen_out_config),
            audio_out_config: Some(audio_out_config),
            debug_config: None,
        };
        AssistRequest { r#type: Some(AssistRequestType::Config(assist_config)) }
    }
}

impl Google {
    pub async fn assistant<T: Display>(query: T) -> Result<GoogleAssistant> {
        let html_content = GoogleAssistant::get_html_content(query).await?;
        let launch_options = LaunchOptions::default_builder().window_size(Some((1920, 1080))).sandbox(false).build()?;
        let browser = Browser::new(launch_options)?;
        let tab = browser.new_tab()?;

        tab.navigate_to("about:blank")?;
        tab.evaluate(
            &format!(
                r#"
                    const html = document.querySelector("html");
                    html.style = "background-image: url(https://picsum.photos/1920/1080)";
                    html.innerHTML = `{}`;
                "#,
                html_content
                // Trim HTML tags
                .trim_start_matches("<html>")
                .trim_end_matches("</html>")
                // Force show card
                .replace(r#"style="display:none""#, "")
                // Fix padding issues for some cards
                .replace("data-hveid=", r#"style="padding: 60px 90px" data-hveid="#),
            ),
            false,
        )?;

        tab.wait_until_navigated()?;

        let card_image = tab.find_element("html")?.capture_screenshot(CaptureScreenshotFormatOption::Png)?;
        let suggestions = tab
            .find_elements(".suggestion")
            .map(|elements| elements.iter().map(|element| element.get_inner_text().unwrap_or_default()).collect::<Vec<String>>())
            .unwrap_or_default();

        Ok(GoogleAssistant { card_image, suggestions })
    }
}
