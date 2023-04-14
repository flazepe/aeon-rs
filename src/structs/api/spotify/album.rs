use crate::{
    functions::{format_timestamp, limit_string, TimestampFormat},
    macros::{if_else, plural},
    statics::{
        emojis::{COPYRIGHT_EMOJI, FIRE_EMOJI, PHONOGRAM_EMOJI},
        spotify::SPOTIFY_EMBED_COLOR,
    },
    structs::api::spotify::{
        components::{
            SpotifyAlbumGroup, SpotifyAlbumType, SpotifyCopyright, SpotifyCopyrightType, SpotifyExternalIDs,
            SpotifyExternalURLs, SpotifyImage, SpotifyItems, SpotifyObjectType, SpotifyPaging,
            SpotifyReleaseDatePrecision, SpotifyRestrictions, SpotifySimpleAlbum, SpotifySimpleArtist,
            SpotifySimpleTrack,
        },
        Spotify,
    },
};
use anyhow::{bail, Result};
use regex::Regex;
use serde::Deserialize;
use slashook::{chrono::NaiveDateTime, structs::embeds::Embed};
use std::fmt::Display;

#[derive(Deserialize)]
pub struct SpotifySearchAlbumResponse {
    pub albums: SpotifyItems<SpotifySimpleAlbum>,
}

#[derive(Deserialize)]
pub struct SpotifyFullAlbum {
    // These are copy pasted from SpotifySimpleAlbum
    #[serde(rename = "type")]
    pub object_type: SpotifyObjectType,

    pub href: String,
    pub external_urls: SpotifyExternalURLs,
    pub uri: String,
    pub album_group: Option<SpotifyAlbumGroup>,
    pub album_type: SpotifyAlbumType,
    pub artists: Vec<SpotifySimpleArtist>,
    pub available_markets: Option<Vec<String>>,
    pub id: String,
    pub images: Vec<SpotifyImage>,
    pub name: String,
    pub release_date: String,
    pub release_date_precision: SpotifyReleaseDatePrecision,
    pub restrictions: Option<SpotifyRestrictions>,
    pub total_tracks: u64,

    // Extra fields
    pub copyrights: Vec<SpotifyCopyright>,
    pub external_ids: SpotifyExternalIDs,
    pub genres: Vec<String>,
    pub label: String,
    pub popularity: u64,
    pub tracks: SpotifyPaging<SpotifySimpleTrack>,
}

impl SpotifyFullAlbum {
    fn _format(&self) -> Embed {
        Embed::new()
            .set_color(SPOTIFY_EMBED_COLOR)
            .unwrap_or_default()
            .set_thumbnail(self.images.get(0).map_or(&"".into(), |image| &image.url))
            .set_title(if_else!(self.name.is_empty(), "N/A".into(), self.name.clone()))
            .set_url(&self.external_urls.spotify)
    }

    pub fn format(self) -> Embed {
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
            .add_field("Label", self.label, false)
            .add_field(
                "Release Date",
                if_else!(
                    matches!(self.release_date_precision, SpotifyReleaseDatePrecision::Day),
                    format_timestamp(
                        NaiveDateTime::parse_from_str(&format!("{} 00:00", self.release_date), "%F %R")
                            .unwrap()
                            .timestamp(),
                        TimestampFormat::Full
                    ),
                    self.release_date
                ),
                false,
            )
            .add_field(
                "Genre",
                if_else!(self.genres.is_empty(), "N/A".into(), self.genres.join(", ")),
                false,
            )
            .add_field(
                "Duration",
                format!(
                    "{} ({})",
                    Spotify::format_duration(
                        self.tracks
                            .items
                            .iter()
                            .map(|track| track.duration_ms)
                            .reduce(|acc, cur| acc + cur)
                            .unwrap_or(0),
                    ),
                    plural!(self.total_tracks, "song")
                ),
                false,
            )
            .add_field("Popularity", format!("{FIRE_EMOJI} {}%", self.popularity), false)
            .add_field(
                if_else!(self.copyrights.len() == 1, "Copyright", "Copyrights"),
                self.copyrights
                    .iter()
                    .map(|copyright| {
                        format!(
                            "{} {}",
                            match copyright.copyright_type {
                                SpotifyCopyrightType::Copyright => COPYRIGHT_EMOJI,
                                SpotifyCopyrightType::Phonogram => PHONOGRAM_EMOJI,
                            },
                            Regex::new(r"\((C|P)\)|©|℗").unwrap().replace_all(&copyright.text, "")
                        )
                    })
                    .collect::<Vec<String>>()
                    .join("\n"),
                false,
            )
    }

    pub fn format_tracks(self) -> Embed {
        self._format().set_description(limit_string(
            self.tracks
                .items
                .iter()
                .map(|track| {
                    format!(
                        "`{}.` [{}]({}) [{}]",
                        format!(
                            "{}{:0pad_length$}",
                            if_else!(
                                self.tracks.items.iter().any(|track| track.disc_number == 2),
                                format!("{}-", track.disc_number),
                                "".into()
                            ),
                            track.track_number,
                            pad_length = self.tracks.items.len().to_string().len()
                        ),
                        track.name,
                        track.external_urls.spotify,
                        Spotify::format_duration(track.duration_ms)
                    )
                })
                .collect::<Vec<String>>()
                .join("\n"),
            "\n",
            4096,
        ))
    }

    pub fn format_available_countries(self) -> Embed {
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
    pub async fn get_album<T: Display>(id: T) -> Result<SpotifyFullAlbum> {
        match Spotify::query(format!("albums/{id}")).await {
            Ok(album) => Ok(album),
            Err(_) => bail!("Album not found."),
        }
    }

    async fn get_simple_album<T: Display>(id: T) -> Result<SpotifySimpleAlbum> {
        match Spotify::query(format!("albums/{id}")).await {
            Ok(album) => Ok(album),
            Err(_) => bail!("Album not found."),
        }
    }

    pub async fn search_album<T: Display>(query: T) -> Result<Vec<SpotifySimpleAlbum>> {
        let query = query.to_string();

        if query.contains("album") {
            Ok(vec![
                Spotify::get_simple_album(query.split("/").last().unwrap().split("?").next().unwrap()).await?,
            ])
        } else {
            let results = Spotify::query::<_, SpotifySearchAlbumResponse>(format!("search?type=album&q={query}"))
                .await?
                .albums
                .items;

            if_else!(results.is_empty(), bail!("Album not found."), Ok(results))
        }
    }
}
