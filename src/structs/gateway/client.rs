use crate::{statics::CONFIG, structs::gateway::events::EventHandler};
use anyhow::Result;
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::{signal::ctrl_c, spawn};
use twilight_gateway::{CloseFrame, Config as TwilightConfig, Event, EventTypeFlags, Intents, Shard, StreamExt, create_recommended};
use twilight_http::Client as TwilightClient;
use twilight_model::gateway::payload::outgoing::identify::IdentifyProperties;

static SHUTDOWN: AtomicBool = AtomicBool::new(false);

pub struct GatewayClient(TwilightClient);

impl GatewayClient {
    pub fn new() -> Self {
        Self(TwilightClient::new(CONFIG.bot.token.clone()))
    }

    pub async fn create_shards(self) -> Result<()> {
        let shards = create_recommended(
            &self.0,
            TwilightConfig::new(
                CONFIG.bot.token.clone(),
                Intents::GUILDS
                    | Intents::GUILD_INVITES
                    | Intents::GUILD_MESSAGES
                    | Intents::GUILD_MESSAGE_REACTIONS
                    | Intents::GUILD_PRESENCES
                    | Intents::GUILD_VOICE_STATES
                    | Intents::MESSAGE_CONTENT,
            ),
            |_, builder| {
                builder.identify_properties(IdentifyProperties::new("Discord Android", "Google Pixel 9 Pro", "Android 15")).build()
            },
        )
        .await?
        .collect::<Vec<_>>();

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

    async fn runner(mut shard: Shard) {
        while let Some(item) = shard.next_event(EventTypeFlags::all()).await {
            let event = match item {
                Ok(Event::GatewayClose(_)) if SHUTDOWN.load(Ordering::Relaxed) => break,
                Ok(event) => event,
                Err(_) => continue,
            };

            let _ = EventHandler::handle(event, shard.sender()).await;
        }
    }
}
