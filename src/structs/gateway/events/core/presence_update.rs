use crate::{
    statics::CACHE,
    structs::gateway::song_activity::{SongActivity, SongActivityService, SongActivityStyle},
};
use anyhow::Result;
use twilight_model::gateway::payload::incoming::PresenceUpdate;

pub async fn handle(event: &PresenceUpdate) -> Result<()> {
    let mut song_activities = CACHE.song_activities.write().unwrap();
    let user_id: String = event.user.id().to_string();
    let Some(activity) = event.activities.iter().find(|activity| activity.name == "Spotify") else {
        song_activities.remove(&user_id);
        return Ok(());
    };
    let song_activity = SongActivity {
        service: SongActivityService::Spotify,
        style: SongActivityStyle::Nori,
        title: activity.details.as_ref().map_or_else(|| "Unknown".into(), |details| details.clone()),
        artist: activity.state.as_ref().map_or_else(|| "Unknown".into(), |state| state.replace(';', ",")),
        album: activity
            .assets
            .as_ref()
            .and_then(|assets| assets.large_text.as_ref())
            .map_or_else(|| "Local Files".into(), |large_text| large_text.clone()),
        album_cover: activity
            .assets
            .as_ref()
            .and_then(|assets| assets.large_image.as_ref())
            .map(|large_image| format!("https://i.scdn.co/image/{}", large_image.chars().skip(8).collect::<String>()))
            .unwrap_or_default(),
        timestamps: activity.timestamps.as_ref().map(|timestamps| (timestamps.start.unwrap_or(0), timestamps.end.unwrap_or(0))),
    };

    song_activities.insert(user_id.clone(), song_activity);

    Ok(())
}
