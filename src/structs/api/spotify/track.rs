use crate::{
    statics::emojis::{EXPLICIT_EMOJI, FIRE_EMOJI},
    structs::api::spotify::{
        components::{
            SpotifyAudioFeatures, SpotifyExternalIDs, SpotifyExternalURLs, SpotifyItems, SpotifyRestrictions, SpotifySimpleAlbum,
            SpotifySimpleArtist, SpotifyTrackLink,
        },
        statics::{SPOTIFY_CAMELOT, SPOTIFY_EMBED_COLOR, SPOTIFY_PITCH_NOTATIONS},
        Spotify,
    },
};
use anyhow::{bail, Result};
use serde::Deserialize;
use slashook::structs::embeds::Embed;
use std::fmt::Display;

#[derive(Deserialize)]
struct SpotifySearchTrackResponse {
    tracks: SpotifyItems<SpotifyFullTrack>,
}

#[derive(Deserialize)]
pub struct SpotifyFullTrack {
    // These are copy pasted from SpotifySimpleTrack
    pub artists: Vec<SpotifySimpleArtist>,
    pub available_markets: Option<Vec<String>>,
    pub disc_number: u64,
    pub duration_ms: u64,
    pub explicit: bool,
    pub external_urls: SpotifyExternalURLs,
    pub href: String,
    pub id: String,
    pub is_playable: Option<bool>,
    pub linked_from: Option<SpotifyTrackLink>,
    pub restrictions: Option<SpotifyRestrictions>,
    pub name: String,
    pub preview_url: Option<String>,
    pub track_number: u64,
    pub uri: String,

    // Extra fields
    pub album: SpotifySimpleAlbum,
    pub audio_features: Option<SpotifyAudioFeatures>, // Unofficial
    pub external_ids: SpotifyExternalIDs,
    pub popularity: u64,
    pub is_local: Option<bool>,
}

impl SpotifyFullTrack {
    pub async fn get_audio_features(&mut self) -> Result<&SpotifyAudioFeatures> {
        self.audio_features = Spotify::query(format!("audio-features/{}", self.id)).await?;
        Ok(self.audio_features.as_ref().unwrap())
    }

    fn _format(&self) -> Embed {
        Embed::new()
            .set_color(SPOTIFY_EMBED_COLOR)
            .unwrap_or_default()
            .set_thumbnail(self.album.images.first().map_or_else(|| "".into(), |image| image.url.clone()))
            .set_title(format!(
                "{}{}",
                match self.explicit {
                    true => format!("{EXPLICIT_EMOJI} "),
                    false => "".into(),
                },
                self.name,
            ))
            .set_url(&self.external_urls.spotify)
    }

    pub fn format(&self) -> Embed {
        self._format()
            .set_image(Spotify::generate_scannable(&self.uri))
            .add_field(
                "Artist",
                self.artists
                    .iter()
                    .take(5)
                    .map(|artist| format!("[{}]({})", artist.name, artist.external_urls.spotify))
                    .collect::<Vec<String>>()
                    .join(", "),
                false,
            )
            .add_field(
                "Album",
                format!(
                    "[{}]({}) (disc {}, track {})",
                    self.album.name, self.album.external_urls.spotify, self.disc_number, self.track_number,
                ),
                false,
            )
            .add_field(
                "Duration",
                format!(
                    "{}{}",
                    Spotify::format_duration(self.duration_ms),
                    self.preview_url.as_ref().map_or_else(|| "".into(), |preview_url| format!(" - [Preview]({preview_url})")),
                ),
                false,
            )
            .add_field("Popularity", format!("{FIRE_EMOJI} {}%", self.popularity), false)
    }

    pub fn format_audio_features(&self) -> Embed {
        let mut embed = self._format();

        if let Some(audio_features) = self.audio_features.as_ref() {
            let pitch_notation = match audio_features.key == -1 {
                true => None,
                false => Some(SPOTIFY_PITCH_NOTATIONS[audio_features.key as usize]),
            };

            embed = embed
                .add_field(
                    "Key",
                    pitch_notation.map_or_else(
                        || "N/A".into(),
                        |pitch_notation| format!("{pitch_notation} {}", ["Minor", "Major"][audio_features.mode as usize]),
                    ),
                    true,
                )
                .add_field(
                    "Camelot",
                    pitch_notation.map_or_else(
                        || "N/A".into(),
                        |pitch_notation| {
                            format!(
                                "{}{}",
                                SPOTIFY_CAMELOT
                                    .iter()
                                    .enumerate()
                                    .find(|(_, entry)| entry[audio_features.mode as usize] == pitch_notation)
                                    .unwrap()
                                    .0
                                    + 1,
                                match audio_features.mode == 0 {
                                    true => "A",
                                    false => "B",
                                },
                            )
                        },
                    ),
                    true,
                )
                .add_field("Tempo", format!("{:.0} BPM", audio_features.tempo), true)
                .add_field("Time Signature", format!("{} / 4", audio_features.time_signature), true)
                .add_field("Loudness", format!("{:.1} dB", audio_features.loudness), true)
                .add_field("Valence", format!("{:.0}%", audio_features.valence * 100.0), true)
                .add_field("Danceability", format!("{:.0}%", audio_features.danceability * 100.0), true)
                .add_field("Energy", format!("{:.0}%", audio_features.energy * 100.0), true)
                .add_field("Speechiness", format!("{:.0}%", audio_features.speechiness * 100.0), true)
                .add_field("Acousticness", format!("{:.0}%", audio_features.acousticness * 100.0), true)
                .add_field("Instrumentalness", format!("{:.0}%", audio_features.instrumentalness * 100.0), true)
                .add_field("Liveness", format!("{:.0}%", audio_features.liveness * 100.0), true);
        }

        embed
    }

    pub fn format_available_countries(&self) -> Embed {
        self._format().set_description(
            self.available_markets
                .as_ref()
                .unwrap_or(&vec![])
                .iter()
                .map(|country| format!(":flag_{}:", country.to_lowercase()))
                .collect::<Vec<String>>()
                .join(" "),
        )
    }
}

impl Spotify {
    pub async fn get_track<T: Display>(id: T) -> Result<SpotifyFullTrack> {
        match Spotify::query(format!("tracks/{id}")).await {
            Ok(track) => Ok(track),
            Err(_) => bail!("Song not found."),
        }
    }

    pub async fn search_track<T: Display>(query: T) -> Result<Vec<SpotifyFullTrack>> {
        let query = query.to_string();

        match query.contains("track") {
            true => Ok(vec![Spotify::get_track(query.split('/').last().unwrap().split('?').next().unwrap()).await?]),
            false => {
                let results = Spotify::query::<_, SpotifySearchTrackResponse>(format!("search?type=track&q={query}")).await?.tracks.items;

                if results.is_empty() {
                    bail!("Song not found.");
                }

                Ok(results)
            },
        }
    }
}
