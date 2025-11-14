use crate::{
    statics::CONFIG,
    structs::{
        database::{Database, redis::keys::RedisKey},
        gateway::events::EventHandler,
    },
};
use anyhow::Result;
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::{signal::ctrl_c, spawn};
use twilight_gateway::{
    CloseFrame, Config as TwilightConfig, ConfigBuilder as TwilightConfigBuilder, Event, EventTypeFlags, Intents, Shard, StreamExt,
    create_recommended,
};
use twilight_http::Client as TwilightClient;
use twilight_model::gateway::{
    payload::outgoing::{identify::IdentifyProperties, update_presence::UpdatePresencePayload},
    presence::{Activity, ActivityType, Status},
};

static SHUTDOWN: AtomicBool = AtomicBool::new(false);

pub struct GatewayClient(TwilightClient);

impl GatewayClient {
    pub fn new() -> Self {
        Self(TwilightClient::new(CONFIG.bot.token.clone()))
    }

    pub async fn create_shards(self) -> Result<()> {
        let shards = create_recommended(&self.0, Self::generate_config().await?, |_, builder| builder.build()).await?.collect::<Vec<_>>();
        let mut senders = Vec::with_capacity(shards.len());
        let mut tasks = Vec::with_capacity(shards.len());

        for shard in shards {
            senders.push(shard.sender());
            tasks.push(spawn(Self::runner(shard)));
        }

        ctrl_c().await?;
        SHUTDOWN.store(true, Ordering::Relaxed);

        for sender in senders {
            _ = sender.close(CloseFrame::NORMAL);
        }

        for handler in tasks {
            _ = handler.await;
        }

        Ok(())
    }

    async fn generate_config() -> Result<TwilightConfig> {
        let custom_status = Database::get_redis()?.get::<String>(&RedisKey::CustomStatus).await.unwrap_or_default();

        let presence = UpdatePresencePayload {
            activities: vec![Activity {
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
                state: Some(custom_status),
                timestamps: None,
                url: None,
            }],
            afk: false,
            since: None,
            status: Status::Online,
        };

        let config = TwilightConfigBuilder::new(
            CONFIG.bot.token.clone(),
            Intents::DIRECT_MESSAGES
                | Intents::GUILDS
                | Intents::GUILD_EMOJIS_AND_STICKERS
                | Intents::GUILD_INVITES
                | Intents::GUILD_MEMBERS
                | Intents::GUILD_MESSAGES
                | Intents::GUILD_MESSAGE_REACTIONS
                | Intents::GUILD_MODERATION
                | Intents::GUILD_PRESENCES
                | Intents::GUILD_VOICE_STATES
                | Intents::MESSAGE_CONTENT,
        )
        .identify_properties(IdentifyProperties::new("Discord Android", "Google Pixel 10 Pro", "Android 16"))
        .presence(presence)
        .build();

        Ok(config)
    }

    async fn runner(mut shard: Shard) {
        while let Some(item) = shard.next_event(EventTypeFlags::all()).await {
            let event = match item {
                Ok(Event::GatewayClose(_)) if SHUTDOWN.load(Ordering::Relaxed) => break,
                Ok(event) => event,
                Err(_) => continue,
            };

            EventHandler::handle(event, shard.sender()).await;
        }
    }
}
