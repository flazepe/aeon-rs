use crate::structs::{config::CONFIG, gateway::events::handler::EventHandler};
use anyhow::Result;
use futures::StreamExt;
use twilight_gateway::{
    stream::{create_recommended, ShardEventStream},
    Config as TwilightConfig, Intents,
};
use twilight_http::Client as TwilightClient;

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
            TwilightConfig::new(
                CONFIG.bot.token.clone(),
                Intents::GUILDS
                    | Intents::GUILD_MESSAGE_REACTIONS
                    | Intents::GUILD_MESSAGES
                    | Intents::MESSAGE_CONTENT,
            ),
            |_, builder| builder.build(),
        )
        .await?
        .collect::<Vec<_>>();

        let mut stream = ShardEventStream::new(shards.iter_mut());

        while let Some((shard, event)) = stream.next().await {
            match event {
                Ok(event) => EventHandler::handle(shard, event),
                Err(source) => {
                    if source.is_fatal() {
                        break;
                    }
                }
            };
        }

        Ok(())
    }
}
