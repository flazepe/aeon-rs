use crate::{
    statics::CACHE,
    structs::gateway::{
        cache::{SongActivity, SongActivityService, SongActivityStyle},
        events::handler::EventHandler,
    },
};
use twilight_model::gateway::payload::incoming::PresenceUpdate;

impl EventHandler {
    pub async fn on_presence_update(presence: Box<PresenceUpdate>) {
        let mut spotify = CACHE.spotify.write().unwrap();
        let user_id = presence.user.id().to_string();

        match presence.activities.iter().find(|activity| activity.name == "Spotify") {
            Some(activity) => spotify.insert(
                user_id.clone(),
                SongActivity {
                    service: SongActivityService::Spotify,
                    style: SongActivityStyle::Nori,
                    title: activity.details.as_ref().map_or("No title".into(), |details| details.clone()),
                    artist: activity.state.as_ref().map_or("No artist".into(), |state| state.replace(';', ",")),
                    album: activity
                        .assets
                        .as_ref()
                        .and_then(|assets| assets.large_text.as_ref())
                        .map_or("Local Files".into(), |large_text| large_text.clone()),
                    album_cover: activity.assets.as_ref().and_then(|assets| assets.large_image.as_ref()).map_or("".into(), |large_image| {
                        format!("https://i.scdn.co/image/{}", large_image.chars().skip(8).collect::<String>())
                    }),
                    timestamps: Some((
                        activity.timestamps.as_ref().and_then(|timestamps| timestamps.start).unwrap_or(0),
                        activity.timestamps.as_ref().and_then(|timestamps| timestamps.end).unwrap_or(0),
                    )),
                },
            ),
            None => spotify.remove(&user_id),
        };
    }
}