use crate::{
    functions::{format_timestamp, label_num, limit_strings, TimestampFormat},
    statics::{
        emojis::{COPYRIGHT_EMOJI, FIRE_EMOJI, PHONOGRAM_EMOJI},
        regex::COPYRIGHT_REGEX,
    },
    structs::api::spotify::{
        components::{
            SpotifyAlbumGroup, SpotifyAlbumType, SpotifyCopyright, SpotifyCopyrightType, SpotifyExternalIDs, SpotifyExternalURLs,
            SpotifyImage, SpotifyItems, SpotifyObjectType, SpotifyPaging, SpotifyReleaseDatePrecision, SpotifyRestrictions,
            SpotifySimpleAlbum, SpotifySimpleArtist, SpotifySimpleTrack,
        },
        Spotify, SPOTIFY_EMBED_COLOR,
    },
};
use anyhow::{bail, Context, Result};
use serde::Deserialize;
use slashook::{chrono::NaiveDateTime, structs::embeds::Embed};
use std::fmt::Display;

#[derive(Deserialize, Debug)]
struct SpotifySearchAlbumResponse {
    albums: SpotifyItems<SpotifySimpleAlbum>,
}

#[derive(Deserialize, Debug)]
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
        let thumbnail = self.images.first().as_ref().map_or("", |image| image.url.as_str());
        let title = if self.name.is_empty() { "N/A".into() } else { self.name.clone() };
        let url = &self.external_urls.spotify;

        Embed::new().set_color(SPOTIFY_EMBED_COLOR).unwrap_or_default().set_thumbnail(thumbnail).set_title(title).set_url(url)
    }

    pub fn format(&self) -> Embed {
        let image = Spotify::generate_scannable(&self.uri);
        let artist =
            limit_strings(self.artists.iter().map(|artist| format!("[{}]({})", artist.name, artist.external_urls.spotify)), ", ", 1024);
        let label = &self.label;
        let release_date = match self.release_date_precision {
            SpotifyReleaseDatePrecision::Day => format_timestamp(
                NaiveDateTime::parse_from_str(&format!("{} 00:00", self.release_date), "%F %R").unwrap().and_utc().timestamp(),
                TimestampFormat::Full,
            ),
            _ => self.release_date.clone(),
        };
        let genres = if self.genres.is_empty() { "N/A".into() } else { self.genres.join(", ") };
        let duration = format!(
            "{} ({})",
            Spotify::format_duration(self.tracks.items.iter().map(|track| track.duration_ms).reduce(|acc, cur| acc + cur).unwrap_or(0)),
            label_num(self.total_tracks, "song", "songs"),
        );
        let copyrights = self
            .copyrights
            .iter()
            .map(|copyright| {
                format!(
                    "{} {}",
                    match copyright.copyright_type {
                        SpotifyCopyrightType::Copyright => COPYRIGHT_EMOJI,
                        SpotifyCopyrightType::Phonogram => PHONOGRAM_EMOJI,
                    },
                    COPYRIGHT_REGEX.replace_all(&copyright.text, ""),
                )
            })
            .collect::<Vec<String>>()
            .join("\n");

        self._format()
            .set_image(image)
            .add_field("Artist", artist, false)
            .add_field("Label", label, false)
            .add_field("Release Date", release_date, false)
            .add_field("Genre", genres, false)
            .add_field("Duration", duration, false)
            .add_field("Popularity", format!("{FIRE_EMOJI} {}%", self.popularity), false)
            .add_field("Copyright", copyrights, false)
    }

    pub fn format_tracks(&self) -> Embed {
        let tracks = limit_strings(
            self.tracks.items.iter().map(|track| {
                format!(
                    "`{}{:0pad_length$}.` [{}]({}) [{}]",
                    if self.tracks.items.iter().any(|track| track.disc_number == 2) {
                        format!("{}-", track.disc_number)
                    } else {
                        "".into()
                    },
                    track.track_number,
                    track.name,
                    track.external_urls.spotify,
                    Spotify::format_duration(track.duration_ms),
                    pad_length = self.tracks.items.len().to_string().len(),
                )
            }),
            "\n",
            4096,
        );
        self._format().set_description(tracks)
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
    pub async fn get_album<T: Display>(id: T) -> Result<SpotifyFullAlbum> {
        Self::query(format!("albums/{id}")).await.context("Album not found.")
    }

    async fn get_simple_album<T: Display>(id: T) -> Result<SpotifySimpleAlbum> {
        Self::query(format!("albums/{id}")).await.context("Album not found.")
    }

    pub async fn search_simple_album<T: Display>(query: T) -> Result<Vec<SpotifySimpleAlbum>> {
        let query = query.to_string();

        if query.contains("album") {
            return Ok(vec![Self::get_simple_album(query.split('/').last().unwrap().split('?').next().unwrap()).await?]);
        }

        let results = Self::query::<_, SpotifySearchAlbumResponse>(format!("search?type=album&q={query}")).await?.albums.items;

        if results.is_empty() {
            bail!("Album not found.");
        }

        Ok(results)
    }
}
