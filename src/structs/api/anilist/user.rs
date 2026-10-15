use crate::{
    functions::{format_timestamp, paginate_strings},
    structs::{
        api::anilist::{
            AniList,
            components::{AniListCharacterNode, AniListFormat, AniListImage, AniListNodes, AniListResponse, AniListTitle},
            statics::{ANILIST_EMBED_AUTHOR_ICON_URL, ANILIST_EMBED_AUTHOR_URL, ANILIST_EMBED_COLOR, ANILIST_USER_FIELDS},
        },
        pagination::Pagination,
    },
};
use anyhow::{Context, Result};
use serde::Deserialize;
use serde_json::json;
use slashook::{
    chrono::{TimeZone, Utc},
    structs::embeds::Embed,
};
use std::fmt::Display;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AniListUserAnimeStatistics {
    pub episodes_watched: u64,
    pub minutes_watched: u64,
    pub mean_score: f64,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AniListUserMangaStatistics {
    pub chapters_read: u64,
    pub volumes_read: u64,
    pub mean_score: f64,
}

#[derive(Deserialize, Debug)]
pub struct AniListUserStatistics {
    pub anime: AniListUserAnimeStatistics,
    pub manga: AniListUserMangaStatistics,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AniListUserMediaFavorites {
    pub title: AniListTitle,
    pub format: Option<AniListFormat>,
    pub site_url: String,
}

#[derive(Deserialize, Debug)]
pub struct AniListUserFavorites {
    pub anime: AniListNodes<AniListUserMediaFavorites>,
    pub manga: AniListNodes<AniListUserMediaFavorites>,
    pub characters: AniListNodes<AniListCharacterNode>,
    pub staff: AniListNodes<AniListCharacterNode>,
}

#[derive(Deserialize, Debug)]
pub struct AniListUserResponse {
    #[serde(rename = "User")]
    pub user: Option<AniListUser>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AniListUser {
    pub id: u64,
    pub site_url: String,
    pub avatar: AniListImage,
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
        let thumbnail = &self.avatar.large;
        let name: &String = &self.name;
        let url = &self.site_url;

        Embed::new()
            .set_color(ANILIST_EMBED_COLOR)
            .unwrap_or_default()
            .set_thumbnail(thumbnail)
            .set_author("AniList  •  User", Some(ANILIST_EMBED_AUTHOR_URL), Some(ANILIST_EMBED_AUTHOR_ICON_URL))
            .set_title(name)
            .set_url(url)
    }

    pub fn format_pagination<T: Display>(&self, command: T) -> Pagination {
        let image = format!("https://img.anili.st/user/{}", self.id);
        let created = format_timestamp(self.created_at, true);
        let anime_statistics = format!(
            "Watched {} episodes\n{} minutes spent\n{:.0}% mean score",
            self.statistics.anime.episodes_watched, self.statistics.anime.minutes_watched, self.statistics.anime.mean_score,
        );
        let manga_statistics = format!(
            "Read {} chapters\nRead {} volumes\n{:.0}% mean score",
            self.statistics.manga.chapters_read, self.statistics.manga.volumes_read, self.statistics.manga.mean_score,
        );
        let timestamp = Utc.timestamp_opt(self.updated_at as i64, 0).unwrap();

        let overview_embed = self
            ._format()
            .set_image(image)
            .add_field("Created", created, false)
            .add_field("Anime Statistics", anime_statistics, true)
            .add_field("Manga Statistics", manga_statistics, true)
            .set_footer("Last updated", None::<String>)
            .set_timestamp(timestamp);

        let about = paginate_strings(self.about.as_deref().unwrap_or_default().split('\n'), "\n", 4096);
        let about_embeds = about.iter().map(|entry| self._format().set_description(entry)).collect::<Vec<Embed>>();

        let favorite_anime = paginate_strings(
            self.favorites.anime.nodes.iter().map(|anime| {
                format!(
                    "[{}]({}){}",
                    anime.title.romaji,
                    anime.site_url,
                    anime.format.as_ref().map(|format| format!(" ({format})")).as_deref().unwrap_or_default(),
                )
            }),
            "\n",
            4096,
        );
        let favorite_anime_embeds = favorite_anime.iter().map(|entry| self._format().set_description(entry)).collect::<Vec<Embed>>();

        let favorite_manga = paginate_strings(
            self.favorites.manga.nodes.iter().map(|manga| {
                format!(
                    "[{}]({}){}",
                    manga.title.romaji,
                    manga.site_url,
                    manga.format.as_ref().map(|format| format!(" ({format})")).as_deref().unwrap_or_default(),
                )
            }),
            "\n",
            4096,
        );
        let favorite_manga_embeds = favorite_manga.iter().map(|entry| self._format().set_description(entry)).collect::<Vec<Embed>>();

        let favorite_characters = paginate_strings(
            self.favorites.characters.nodes.iter().map(|character| format!("[{}]({})", character.name.full, character.site_url)),
            "\n",
            4096,
        );
        let favorite_characters_embeds =
            favorite_characters.iter().map(|entry| self._format().set_description(entry)).collect::<Vec<Embed>>();

        let favorite_staff = paginate_strings(
            self.favorites.staff.nodes.iter().map(|staff| format!("[{}]({})", staff.name.full, staff.site_url)),
            "\n",
            4096,
        );
        let favorite_staff_embeds = favorite_staff.iter().map(|entry| self._format().set_description(entry)).collect::<Vec<Embed>>();

        Pagination::new(command)
            .add_section("Overview", overview_embed)
            .add_section("About", about_embeds)
            .add_section("Favorite Anime", favorite_anime_embeds)
            .add_section("Favorite Manga", favorite_manga_embeds)
            .add_section("Favorite Characters", favorite_characters_embeds)
            .add_section("Favorite Staff", favorite_staff_embeds)
    }
}

impl AniList {
    pub async fn get_user<T: Display>(name: T) -> Result<AniListUser> {
        Self::query::<_, AniListResponse<AniListUserResponse>>(
            format!(
                "query($search: String) {{
                    User(name: $search) {{
                        {ANILIST_USER_FIELDS}
                    }}
                }}",
            ),
            json!({ "search": name.to_string() }),
        )
        .await?
        .data
        .user
        .context("User not found.")
    }
}
