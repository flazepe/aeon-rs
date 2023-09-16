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
    presence::{Activity, ActivityType, Status},
};

pub struct GatewayClient {
    client: TwilightClient,
}

impl GatewayClient {
    pub fn new() -> Self {
        Self { client: TwilightClient::new(CONFIG.bot.token.clone()) }
    }

    pub async fn create_shards(self) -> Result<()> {
        let mut shards = create_recommended(
            &self.client,
            TwilightConfig::builder(
                CONFIG.bot.token.clone(),
                Intents::GUILDS
                    | Intents::GUILD_MESSAGE_REACTIONS
                    | Intents::GUILD_MESSAGES
                    | Intents::MESSAGE_CONTENT
                    | Intents::GUILD_PRESENCES,
            )
            .identify_properties(IdentifyProperties::new("Discord Android", "Google Pixel 8 Pro", "Android 14"))
            .presence(UpdatePresencePayload::new(
                vec![Activity {
                    application_id: None,
                    assets: None,
                    buttons: vec![],
                    created_at: None,
                    details: None,
                    emoji: None,
                    flags: None,
                    id: None,
                    instance: None,
                    kind: ActivityType::Custom,
                    name: "yes".into(),
                    party: None,
                    secrets: None,
                    state: Some("I got the new Pixel".into()),
                    timestamps: None,
                    url: None,
                }],
                false,
                None,
                Status::Online,
            )?)
            .build(),
            |_, builder| builder.build(),
        )
        .await?
        .collect::<Vec<_>>();

        let mut stream = ShardEventStream::new(shards.iter_mut());

        while let Some((shard, event)) = stream.next().await {
            match event {
                Ok(event) => EventHandler::handle(event, shard).await,
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
