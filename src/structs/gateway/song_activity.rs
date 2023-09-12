use serde::Serialize;

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SongActivity {
    pub service: SongActivityService,
    pub style: SongActivityStyle,
    pub title: String,
    pub artist: String,
    pub album: String,
    pub album_cover: String,
    pub timestamps: Option<(u64, u64)>,
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum SongActivityService {
    Aeon,
    Deezer,
    Itunes,
    SoundCloud,
    Spotify,
    YouTube,
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum SongActivityStyle {
    Classic,
    Nori,
    Rovi,
    Vxc,
}

impl From<&str> for SongActivityStyle {
    fn from(value: &str) -> Self {
        match value {
            "classic" => SongActivityStyle::Classic,
            "nori" => SongActivityStyle::Nori,
            "rovi" => SongActivityStyle::Rovi,
            "vxc" => SongActivityStyle::Vxc,
            // Default card style is nori's
            _ => SongActivityStyle::Nori,
        }
    }
}
