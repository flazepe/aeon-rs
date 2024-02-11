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
    Modern,
    Nori,
    Rovi,
    Vxc,
}

impl From<&str> for SongActivityStyle {
    fn from(value: &str) -> Self {
        match value {
            "classic" => Self::Classic,
            "modern" => Self::Modern,
            "nori" => Self::Nori,
            "rovi" => Self::Rovi,
            "vxc" => Self::Vxc,
            // Default card style is nori's
            _ => Self::Nori,
        }
    }
}
