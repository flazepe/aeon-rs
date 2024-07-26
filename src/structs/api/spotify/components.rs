use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum SpotifyAlbumGroup {
    Album,
    Single,
    Compilation,
    AppearsOn,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum SpotifyAlbumType {
    Album,
    Single,
    Compilation,
}

#[derive(Deserialize, Debug)]
pub struct SpotifyAudioFeatures {
    pub acousticness: f64,
    pub analysis_url: String,
    pub danceability: f64,
    pub duration_ms: u64,
    pub energy: f64,
    pub id: String,
    pub instrumentalness: f64,
    pub key: i64,
    pub liveness: f64,
    pub loudness: f64,
    pub mode: u64,
    pub speechiness: f64,
    pub tempo: f64,
    pub time_signature: u64,
    pub track_href: String,

    #[serde(rename = "type")]
    pub object_type: SpotifyObjectType,

    pub uri: String,
    pub valence: f64,
}

#[derive(Deserialize, Debug)]
pub struct SpotifyCopyright {
    pub text: String,

    #[serde(rename = "type")]
    pub copyright_type: SpotifyCopyrightType,
}

#[derive(Deserialize, Debug)]
pub enum SpotifyCopyrightType {
    #[serde(rename = "C")]
    Copyright,

    #[serde(rename = "P")]
    Phonogram,
}

#[derive(Deserialize, Debug)]
pub struct SpotifyExternalIDs {
    pub isrc: Option<String>,
    pub ean: Option<String>,
    pub upc: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct SpotifyExternalURLs {
    pub spotify: String,
}

#[derive(Deserialize, Debug)]
pub struct SpotifyImage {
    pub width: u64,
    pub height: Option<u64>,
    pub url: String,
}

#[derive(Deserialize, Debug)]
pub struct SpotifyItems<T> {
    pub items: Vec<T>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum SpotifyObjectType {
    Artist,
    Playlist,
    Album,
    Show,
    Episode,
    AudioFeatures,
}

#[derive(Deserialize, Debug)]
pub struct SpotifyPaging<T> {
    pub href: String,
    pub items: Vec<T>,
    pub limit: u64,
    pub next: Option<String>,
    pub offset: u64,
    pub previous: Option<String>,
    pub total: u64,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum SpotifyReleaseDatePrecision {
    Year,
    Month,
    Day,
}

#[derive(Deserialize, Debug)]
pub struct SpotifyRestrictions {
    pub reason: String,
}

#[derive(Deserialize, Debug)]
pub struct SpotifySimpleAlbum {
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
}

#[derive(Deserialize, Debug)]
pub struct SpotifySimpleArtist {
    pub name: String,
    pub id: String,

    #[serde(rename = "type")]
    pub object_type: SpotifyObjectType,

    pub href: String,
    pub external_urls: SpotifyExternalURLs,
    pub uri: String,
}

#[derive(Deserialize, Debug)]
pub struct SpotifySimpleTrack {
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
}

#[derive(Deserialize, Debug)]
pub struct SpotifyTrackLink {
    pub external_urls: SpotifyExternalURLs,
    pub href: String,
    pub id: String,

    #[serde(rename = "type")]
    pub object_type: SpotifyObjectType,

    pub uri: String,
}
