use crate::{
    functions::limit_strings,
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
use anyhow::{bail, Context, Result};
use serde::Deserialize;
use slashook::structs::embeds::Embed;
use std::fmt::Display;

#[derive(Deserialize, Debug)]
struct SpotifySearchTrackResponse {
    tracks: SpotifyItems<SpotifyFullTrack>,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
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
        let thumbnail = self.album.images.first().map_or("", |image| image.url.as_str());
        let title = format!("{}{}", if self.explicit { format!("{EXPLICIT_EMOJI} ") } else { "".into() }, self.name);
        let url = &self.external_urls.spotify;

        Embed::new().set_color(SPOTIFY_EMBED_COLOR).unwrap_or_default().set_thumbnail(thumbnail).set_title(title).set_url(url)
    }

    pub fn format(&self) -> Embed {
        let image = Spotify::generate_scannable(&self.uri);
        let artist =
            limit_strings(self.artists.iter().map(|artist| format!("[{}]({})", artist.name, artist.external_urls.spotify)), ", ", 1024);
        let album = format!(
            "[{}]({}) (disc {}, track {})",
            self.album.name, self.album.external_urls.spotify, self.disc_number, self.track_number,
        );
        let duration = format!(
            "{}{}",
            Spotify::format_duration(self.duration_ms),
            self.preview_url.as_ref().map(|preview_url| format!(" - [Preview]({preview_url})")).as_deref().unwrap_or(""),
        );
        let popularity = format!("{FIRE_EMOJI} {}%", self.popularity);

        self._format()
            .set_image(image)
            .add_field("Artist", artist, false)
            .add_field("Album", album, false)
            .add_field("Duration", duration, false)
            .add_field("Popularity", popularity, false)
    }

    pub fn format_audio_features(&self) -> Embed {
        let mut embed = self._format();
        let Some(audio_features) = self.audio_features.as_ref() else { return embed };
        let pitch_notation = if audio_features.key == -1 { None } else { Some(SPOTIFY_PITCH_NOTATIONS[audio_features.key as usize]) };
        let key = pitch_notation
            .map(|pitch_notation| format!("{pitch_notation} {}", ["Minor", "Major"][audio_features.mode as usize]))
            .unwrap_or_else(|| "N/A".into());
        let camelot = pitch_notation
            .map(|pitch_notation| {
                format!(
                    "{}{}",
                    SPOTIFY_CAMELOT.iter().enumerate().find(|(_, entry)| entry[audio_features.mode as usize] == pitch_notation).unwrap().0
                        + 1,
                    ["A", "b"][audio_features.mode as usize],
                )
            })
            .unwrap_or_else(|| "N/A".into());
        let tempo = format!("{:.0} BPM", audio_features.tempo);
        let time_signature = format!("{} / 4", audio_features.time_signature);
        let loudness = format!("{:.1} dB", audio_features.loudness);
        let valence = format!("{:.0}%", audio_features.valence * 100.0);
        let danceability = format!("{:.0}%", audio_features.danceability * 100.0);
        let energy = format!("{:.0}%", audio_features.energy * 100.0);
        let speechiness = format!("{:.0}%", audio_features.speechiness * 100.0);
        let acousticness = format!("{:.0}%", audio_features.acousticness * 100.0);
        let instrumentalness = format!("{:.0}%", audio_features.instrumentalness * 100.0);
        let liveness = format!("{:.0}%", audio_features.liveness * 100.0);

        embed = embed
            .add_field("Key", key, true)
            .add_field("Camelot", camelot, true)
            .add_field("Tempo", tempo, true)
            .add_field("Time Signature", time_signature, true)
            .add_field("Loudness", loudness, true)
            .add_field("Valence", valence, true)
            .add_field("Danceability", danceability, true)
            .add_field("Energy", energy, true)
            .add_field("Speechiness", speechiness, true)
            .add_field("Acousticness", acousticness, true)
            .add_field("Instrumentalness", instrumentalness, true)
            .add_field("Liveness", liveness, true);

        embed
    }

    pub fn format_available_countries(&self) -> Embed {
        let available_markets = self
            .available_markets
            .as_ref()
            .unwrap_or(&vec![])
            .iter()
            .map(|country| format!(":flag_{}:", country.to_lowercase()))
            .collect::<Vec<String>>()
            .join(" ");
        self._format().set_description(available_markets)
    }
}

impl Spotify {
    pub async fn get_track<T: Display>(id: T) -> Result<SpotifyFullTrack> {
        Spotify::query(format!("tracks/{id}")).await.context("Song not found.")
    }

    pub async fn search_track<T: Display>(query: T) -> Result<Vec<SpotifyFullTrack>> {
        let query = query.to_string();

        if query.contains("track") {
            return Ok(vec![Spotify::get_track(query.split('/').last().unwrap().split('?').next().unwrap()).await?]);
        }

        let results = Spotify::query::<_, SpotifySearchTrackResponse>(format!("search?type=track&q={query}")).await?.tracks.items;

        if results.is_empty() {
            bail!("Song not found.");
        }

        Ok(results)
    }
}
