use crate::{
    functions::{format_timestamp, if_else_option, limit_string, TimestampFormat},
    statics::{anilist::ANILIST_USER_FIELDS, colors::PRIMARY_COLOR},
    structs::api::anilist::{
        components::{AniListCharacterNode, AniListFormat, AniListImage, AniListNodes, AniListResponse, AniListTitle},
        AniList,
    },
};
use anyhow::{bail, Result};
use serde::Deserialize;
use serde_json::json;
use slashook::{
    chrono::{TimeZone, Utc},
    structs::embeds::Embed,
};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AniListUserAnimeStatistics {
    pub episodes_watched: u64,
    pub minutes_watched: u64,
    pub mean_score: f64,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AniListUserMangaStatistics {
    pub chapters_read: u64,
    pub volumes_read: u64,
    pub mean_score: f64,
}

#[derive(Deserialize)]
pub struct AniListUserStatistics {
    pub anime: AniListUserAnimeStatistics,
    pub manga: AniListUserMangaStatistics,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AniListUserMediaFavorites {
    pub title: AniListTitle,
    pub format: Option<AniListFormat>,
    pub site_url: String,
}

#[derive(Deserialize)]
pub struct AniListUserFavorites {
    pub anime: AniListNodes<AniListUserMediaFavorites>,
    pub manga: AniListNodes<AniListUserMediaFavorites>,
    pub characters: AniListNodes<AniListCharacterNode>,
    pub staff: AniListNodes<AniListCharacterNode>,
}

#[derive(Deserialize)]
pub struct AniListUserResponse {
    #[serde(rename = "User")]
    pub user: Option<AniListUser>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AniListUser {
    pub id: u64,
    pub site_url: String,
    pub avatar: Option<AniListImage>,
    pub name: String,
    pub created_at: u64,
    pub updated_at: u64,
    pub about: Option<String>,
    pub statistics: AniListUserStatistics,

    #[serde(rename = "favourites")]
    pub favorites: AniListUserFavorites,
}

impl AniListUser {
    fn _format(&self) -> Embed {
        Embed::new()
            .set_color(PRIMARY_COLOR)
            .unwrap_or_default()
            .set_thumbnail(if_else_option(self.avatar.as_ref(), |avatar| &avatar.large, &"".into()))
            .set_title(&self.name)
            .set_url(&self.site_url)
    }

    pub fn format(self) -> Embed {
        self._format()
            .set_image(format!("https://img.anili.st/user/{}", self.id))
            .add_field(
                "Created",
                format_timestamp(self.created_at, TimestampFormat::Full),
                false,
            )
            .add_field(
                "Anime Statistics",
                format!(
                    "Watched {} episodes\n{} minutes spent\n{:.0}% mean score",
                    self.statistics.anime.episodes_watched,
                    self.statistics.anime.minutes_watched,
                    self.statistics.anime.mean_score,
                ),
                true,
            )
            .add_field(
                "Manga Statistics",
                format!(
                    "Read {} chapters\nRead {} volumes\n{:.0}% mean score",
                    self.statistics.manga.chapters_read,
                    self.statistics.manga.volumes_read,
                    self.statistics.manga.mean_score,
                ),
                true,
            )
            .set_footer("Last updated", None::<String>)
            .set_timestamp(Utc.timestamp_opt(self.updated_at as i64, 0).unwrap())
    }

    pub fn format_about(self) -> Embed {
        self._format()
            .set_description(limit_string(self.about.unwrap_or("".into()), "\n", 4096))
    }

    pub fn format_favorite_anime(self) -> Embed {
        self._format().set_description(limit_string(
            self.favorites
                .anime
                .nodes
                .iter()
                .map(|anime| {
                    format!(
                        "[{}]({}){}",
                        anime.title.romaji,
                        anime.site_url,
                        if_else_option(
                            anime.format.as_ref(),
                            |format| format!(" ({})", AniList::format_enum_value(format)),
                            "".into()
                        )
                    )
                })
                .collect::<Vec<String>>()
                .join("\n"),
            "\n",
            4096,
        ))
    }

    pub fn format_favorite_manga(self) -> Embed {
        self._format().set_description(limit_string(
            self.favorites
                .manga
                .nodes
                .iter()
                .map(|manga| {
                    format!(
                        "[{}]({}){}",
                        manga.title.romaji,
                        manga.site_url,
                        if_else_option(
                            manga.format.as_ref(),
                            |format| format!(" ({})", AniList::format_enum_value(format)),
                            "".into()
                        )
                    )
                })
                .collect::<Vec<String>>()
                .join("\n"),
            "\n",
            4096,
        ))
    }

    pub fn format_favorite_characters(self) -> Embed {
        self._format().set_description(limit_string(
            self.favorites
                .characters
                .nodes
                .iter()
                .map(|character| format!("[{}]({})", character.name.full, character.site_url))
                .collect::<Vec<String>>()
                .join("\n"),
            "\n",
            4096,
        ))
    }

    pub fn format_favorite_staff(self) -> Embed {
        self._format().set_description(limit_string(
            self.favorites
                .staff
                .nodes
                .iter()
                .map(|staff| format!("[{}]({})", staff.name.full, staff.site_url))
                .collect::<Vec<String>>()
                .join("\n"),
            "\n",
            4096,
        ))
    }
}

impl AniList {
    pub async fn get_user<T: ToString>(user: T) -> Result<AniListUser> {
        let result: AniListResponse<AniListUserResponse> = AniList::query(
            format!(
                "query($search: String) {{
					User(name: $search) {{
						{ANILIST_USER_FIELDS}
					}}
				}}"
            ),
            json!({ "search": user.to_string() }),
        )
        .await?;

        match result.data.user {
            Some(user) => Ok(user),
            None => bail!("User not found."),
        }
    }
}
