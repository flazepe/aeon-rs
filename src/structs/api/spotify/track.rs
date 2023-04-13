use crate::{
    macros::if_else,
    statics::{
        emojis::{EXPLICIT_EMOJI, FIRE_EMOJI},
        spotify::SPOTIFY_EMBED_COLOR,
    },
    structs::api::spotify::{
        components::{
            SpotifyExternalIDs, SpotifyExternalURLs, SpotifyItems, SpotifyRestrictions, SpotifySimpleAlbum,
            SpotifySimpleArtist, SpotifyTrackLink,
        },
        Spotify,
    },
};
use anyhow::{bail, Result};
use serde::Deserialize;
use slashook::structs::embeds::Embed;
use std::fmt::Display;

#[derive(Deserialize)]
pub struct SpotifySearchTrackResponse {
    pub tracks: SpotifyItems<SpotifyFullTrack>,
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
    pub external_ids: SpotifyExternalIDs,
    pub popularity: u64,
    pub is_local: Option<bool>,
}

impl SpotifyFullTrack {
    fn _format(&self) -> Embed {
        Embed::new()
            .set_color(SPOTIFY_EMBED_COLOR)
            .unwrap_or_default()
            .set_thumbnail(self.album.images.get(0).map_or(&"".into(), |image| &image.url))
            .set_title(format!(
                "{}{}",
                if_else!(self.explicit, format!("{EXPLICIT_EMOJI} "), "".into()),
                self.name
            ))
            .set_url(&self.external_urls.spotify)
    }

    pub fn format(self) -> Embed {
        self._format()
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
                    self.album.name, self.album.external_urls.spotify, self.disc_number, self.track_number
                ),
                false,
            )
            .add_field(
                "Duration",
                format!(
                    "{}{}",
                    Spotify::format_duration(self.duration_ms),
                    self.preview_url
                        .map_or("".into(), |preview_url| format!(" - [Preview]({preview_url})"))
                ),
                false,
            )
            .add_field("Popularity", format!("{FIRE_EMOJI} {}%", self.popularity), false)
            .set_image(Spotify::generate_scannable(&self.uri))
    }

    pub fn format_supported_countries(self) -> Embed {
        self._format().set_description(
            self.available_markets
                .unwrap_or(vec![])
                .iter()
                .map(|country| format!(":flag_{}:", country.to_lowercase()))
                .collect::<Vec<String>>()
                .join(" "),
        )
    }
}

impl Spotify {
    pub async fn get_track<T: Display>(id: T) -> Result<SpotifyFullTrack> {
        Ok(Spotify::query(format!("tracks/{id}")).await?)
    }

    pub async fn search_track<T: Display>(query: T) -> Result<Vec<SpotifyFullTrack>> {
        let results = Spotify::query::<_, SpotifySearchTrackResponse>(format!("search?type=track&q={query}"))
            .await?
            .tracks
            .items;

        if_else!(results.is_empty(), bail!("Song not found."), Ok(results))
    }
}
