use crate::{
    statics::CONFIG,
    structs::{
        api::google::{
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
            statics::{GOOGLE_EMBED_AUTHOR_ICON_URL, GOOGLE_EMBED_AUTHOR_URL, GOOGLE_EMBED_COLOR},
        },
        select_menu::SelectMenu,
    },
};
use anyhow::{Result, bail};
use futures::stream::iter;
use gouth::Builder;
use headless_chrome::{Browser, LaunchOptions, protocol::cdp::Page::CaptureScreenshotFormatOption};
use serde_json::json;
use slashook::{
    commands::MessageResponse,
    structs::{embeds::Embed, utils::File},
};
use std::fmt::Display;
use tokio::fs::read;
use tonic::{
    Request,
    metadata::MetadataValue,
    transport::{Certificate, Channel, ClientTlsConfig},
};

pub struct GoogleAssistant {
    card_image: Option<Vec<u8>>,
    suggestions: Vec<String>,
    text: Option<String>,
    audio_file: Option<Vec<u8>>,
}

impl GoogleAssistant {
    async fn request<T: Display>(query: T) -> Result<Self> {
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

        let mut client = EmbeddedAssistantClient::with_interceptor(
            Channel::from_static("https://embeddedassistant.googleapis.com").tls_config(tls_config)?.connect().await?,
            move |mut req: Request<()>| {
                let Ok(header_value) = token.header_value() else { return Ok(req) };
                let Ok(metadata_value) = MetadataValue::try_from(&*header_value) else { return Ok(req) };
                req.metadata_mut().insert("authorization", metadata_value);
                Ok(req)
            },
        );

        let query = Self::generate_request(query);
        let mut response = client.assist(Request::new(iter(vec![query]))).await?.into_inner();

        let mut html = None::<String>;
        let mut text = None::<String>;
        let mut audio_file = None::<Vec<u8>>;

        while let Some(message) = response.message().await? {
            if let Some(screen_out) = message.screen_out {
                html = Some(String::from_utf8(screen_out.data)?);
            }

            if let Some(dialog_state_out) = message.dialog_state_out {
                if !dialog_state_out.supplemental_display_text.is_empty() {
                    text = Some(dialog_state_out.supplemental_display_text);
                }
            }

            if let Some(mut audio_out) = message.audio_out {
                if let Some(audio_file) = &mut audio_file {
                    audio_file.append(&mut audio_out.audio_data);
                } else {
                    audio_file = Some(audio_out.audio_data);
                }
            }
        }

        if html.is_none() && text.is_none() && audio_file.is_none() {
            bail!("Could not get response.");
        }

        let mut card_image = None;
        let mut suggestions = vec![];

        if let Some(html) = html {
            let launch_options: LaunchOptions<'_> =
                LaunchOptions::default_builder().window_size(Some((1920, 1080))).sandbox(false).build()?;
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
                    html
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

            card_image = Some(tab.find_element("html")?.capture_screenshot(CaptureScreenshotFormatOption::Png)?);

            if let Ok(elements) = tab.find_elements(".suggestion") {
                suggestions = elements.iter().map(|element| element.get_inner_text().unwrap_or_default()).collect::<Vec<String>>();
            }
        }

        Ok(Self { card_image, suggestions, text, audio_file })
    }

    fn generate_request<T: Display>(query: T) -> AssistRequest {
        let config_type: AssistConfigType = AssistConfigType::TextQuery(query.to_string());

        let device_config = DeviceConfig {
            device_id: CONFIG.api.google_assistant.device_id.clone(),
            device_model_id: CONFIG.api.google_assistant.device_model_id.clone(),
        };

        // Batman, Türkiye: 37.887, 41.132
        // San Francisco: 37.783333, -122.416667
        let device_coordinates = LatLng { latitude: 37.783333, longitude: -122.416667 };
        let device_location = DeviceLocation { r#type: Some(DeviceLocationType::Coordinates(device_coordinates)) };

        let dialog_state_in = DialogStateIn {
            conversation_state: vec![],
            language_code: "en-US".to_string(),
            device_location: Some(device_location),
            is_new_conversation: false,
        };

        let screen_out_config = ScreenOutConfig { screen_mode: ScreenMode::Playing.into() };
        let audio_out_config = AudioOutConfig { encoding: Encoding::OpusInOgg.into(), sample_rate_hertz: 24000, volume_percentage: 100 };

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

    pub fn format(&self) -> MessageResponse {
        let mut response = MessageResponse {
            tts: Some(false),
            content: None,
            flags: None,
            embeds: None,
            components: None,
            attachments: None,
            allowed_mentions: None,
            files: None,
            poll: None,
        };

        let mut embed = Embed::new().set_color(GOOGLE_EMBED_COLOR).unwrap_or_default().set_author(
            "Google  •  Assistant",
            Some(GOOGLE_EMBED_AUTHOR_URL),
            Some(GOOGLE_EMBED_AUTHOR_ICON_URL),
        );

        if let Some(card_image) = &self.card_image {
            response = response.add_file(File::new("image.png", card_image.as_slice()));
            embed = embed.set_image("attachment://image.png");
        }

        if !self.suggestions.is_empty() {
            let select_menu = SelectMenu::new("google", "assistant", "Try saying…", None::<String>)
                .add_options(self.suggestions.iter().map(|suggestion| (suggestion.clone(), suggestion.clone(), None::<String>)));
            response = response.set_components(select_menu.into());
        }

        if let Some(text) = &self.text {
            embed = embed.set_description(text);
        }

        if let Some(audio_file) = &self.audio_file {
            response = response.add_file(File::new("audio.ogg", audio_file.as_slice()));
        }

        response.add_embed(embed)
    }
}

impl Google {
    pub async fn assistant<T: Display>(query: T) -> Result<GoogleAssistant> {
        GoogleAssistant::request(query).await
    }
}
