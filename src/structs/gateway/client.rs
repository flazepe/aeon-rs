use crate::{statics::CONFIG, structs::gateway::events::handler::EventHandler};
use anyhow::Result;
use futures::StreamExt;
use twilight_gateway::{
    stream::{create_recommended, ShardEventStream},
    Config as TwilightConfig, Intents,
};
use twilight_http::Client as TwilightClient;
use twilight_model::gateway::{
    payload::outgoing::{identify::IdentifyProperties, update_presence::UpdatePresencePayload},
    presence::Status,
};

pub struct GatewayClient {
    client: TwilightClient,
}

impl GatewayClient {
    pub fn new() -> Self {
        Self {
            client: TwilightClient::new(CONFIG.bot.token.clone()),
        }
    }

    pub async fn create_shards(self) -> Result<()> {
        let mut shards = create_recommended(
            &self.client,
            TwilightConfig::builder(
                CONFIG.bot.token.clone(),
                Intents::GUILDS | Intents::GUILD_MESSAGE_REACTIONS | Intents::GUILD_MESSAGES | Intents::MESSAGE_CONTENT,
            )
            .identify_properties(IdentifyProperties {
                browser: "Discord Android".into(),
                device: "Google Pixel 7 Pro".into(),
                os: "Android 14".into(),
            })
            .build(),
            |_, builder| builder.build(),
        )
        .await?
        .collect::<Vec<_>>();

        let mut stream = ShardEventStream::new(shards.iter_mut());

        while let Some((shard, event)) = stream.next().await {
            match event {
                Ok(event) => EventHandler::handle(event, shard),
                Err(error) => {
                    if error.is_fatal() {
                        break;
                    }
                },
            };
        }

        Ok(())
    }
}
