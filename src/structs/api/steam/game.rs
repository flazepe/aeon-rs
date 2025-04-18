use crate::{
    functions::{format_timestamp, limit_strings},
    statics::REQWEST,
    structs::api::steam::{
        Steam,
        statics::{STEAM_EMBED_AUTHOR_ICON_URL, STEAM_EMBED_AUTHOR_URL, STEAM_EMBED_COLOR},
    },
};
use anyhow::{Result, bail};
use nipper::Document;
use serde::Deserialize;
use serde_json::{Value, from_value};
use slashook::{chrono::NaiveDateTime, structs::embeds::Embed};
use std::fmt::Display;

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct SteamGameRequirements {
    pub minimum: String,
    pub recommended: Option<String>,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct SteamGamePriceOverview {
    pub currency: String,
    pub initial: u64,

    #[serde(rename = "final")]
    pub final_price: u64,

    pub discount_percent: u64,
    pub initial_formatted: String,
    pub final_formatted: String,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct SteamPackageGroup {
    pub name: String,
    pub title: String,
    pub description: String,
    pub selection_text: String,
    pub save_text: String,
    // pub display_type: u64, Sometimes a number and sometimes a string
    pub is_recurring_subscription: String, // A bool in a string
    pub subs: Vec<SteamPackageGroupSub>,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct SteamPackageGroupSub {
    #[serde(rename = "packageid")]
    pub id: u64,

    pub percent_savings_text: String,
    pub percent_savings: u64,
    pub option_text: String,
    pub option_description: String,
    pub can_get_free_license: String, // A number in a string, probably a 0 and 1 as a string
    pub is_free_license: bool,
    pub price_in_cents_with_discount: u64,
}

#[derive(Deserialize, Debug)]
pub struct SteamPlatforms {
    pub windows: bool,
    pub mac: bool,
    pub linux: bool,
}

#[derive(Deserialize, Debug)]
pub struct SteamMetacritic {
    pub score: u64,
    pub url: String,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct SteamCategory {
    pub id: u64,
    pub description: String,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct SteamGenre {
    pub id: String, // Why is this one inside a string
    pub description: String,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct SteamScreenshot {
    pub id: u64,
    pub path_thumbnail: String,
    pub path_full: String,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct SteamMovie {
    pub id: u64,
    pub name: String,
    pub thumbnail: String,
    pub webm: SteamMovieSizes,
    pub mp4: SteamMovieSizes,
    pub highlight: bool,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct SteamMovieSizes {
    #[serde(rename = "480")]
    pub min: String,

    pub max: String,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct SteamRecommendations {
    pub total: u64,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct SteamAchievements {
    pub total: u64,
    pub highlighted: Vec<SteamAchievement>,
}

#[derive(Deserialize, Debug)]
pub struct SteamAchievement {
    pub name: String,
    pub path: String,
}

#[derive(Deserialize, Debug)]
pub struct SteamReleaseDate {
    pub coming_soon: bool,
    pub date: String,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct SteamSupportInfo {
    pub url: String,
    pub email: String,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct SteamContentDescriptor {
    pub ids: Vec<u64>,
    pub notes: Option<String>,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct SteamGame {
    #[serde(rename = "type")]
    pub app_type: String,

    pub name: String,

    #[serde(rename = "steam_appid")]
    pub id: u64,

    // pub required_age: String, Sometimes a 0 (number)
    pub is_free: bool,
    pub controller_support: Option<String>,
    pub dlc: Option<Vec<u64>>,
    pub detailed_description: String,
    pub about_the_game: String,
    pub short_description: String,
    pub supported_languages: String,
    pub reviews: Option<String>,
    pub header_image: String,
    pub website: Option<String>,
    // pub pc_requirements: SteamGameRequirements, Sometimes an empty array for some fucking reason
    // pub mac_requirements: SteamGameRequirements, Sometimes an empty array for some fucking reason
    // pub linux_requirements: SteamGameRequirements, Sometimes an empty array for some fucking reason
    pub legal_notice: Option<String>,
    pub ext_user_account_notice: Option<String>,
    pub developers: Option<Vec<String>>,
    pub publishers: Option<Vec<String>>,
    pub price_overview: Option<SteamGamePriceOverview>,
    pub packages: Vec<u64>,
    pub package_groups: Vec<SteamPackageGroup>,
    pub platforms: SteamPlatforms,
    pub metacritic: Option<SteamMetacritic>,
    pub categories: Option<Vec<SteamCategory>>,
    pub genres: Option<Vec<SteamGenre>>,
    pub screenshots: Vec<SteamScreenshot>,
    pub movies: Vec<SteamMovie>,
    pub recommendations: Option<SteamRecommendations>,
    pub achievements: Option<SteamAchievements>,
    pub release_date: Option<SteamReleaseDate>,
    pub support_info: SteamSupportInfo,
    pub background: String,
    pub background_raw: String,
    pub content_descriptors: SteamContentDescriptor,
}

impl SteamGame {
    pub fn _format(&self) -> Embed {
        let title = &self.name;
        let url = format!("https://store.steampowered.com/app/{}", self.id);

        Embed::new()
            .set_color(STEAM_EMBED_COLOR)
            .unwrap_or_default()
            .set_author("Steam  •  Game", Some(STEAM_EMBED_AUTHOR_URL), Some(STEAM_EMBED_AUTHOR_ICON_URL))
            .set_title(title)
            .set_url(url)
    }

    pub fn format(&self) -> Embed {
        let image = &self.header_image;
        let description = limit_strings(Document::from(&self.short_description).select("body").text().split('\n'), "\n", 4096);
        let release_date = self
            .release_date
            .as_ref()
            .map(|release_date| {
                format!(
                    "{}{}",
                    format_timestamp(
                        NaiveDateTime::parse_from_str(&format!("{} 00:00", release_date.date), "%b %-d, %Y %R")
                            .unwrap()
                            .and_utc()
                            .timestamp(),
                        true,
                    ),
                    if release_date.coming_soon { " (coming soon)" } else { "" },
                )
            })
            .unwrap_or_else(|| "TBA".into());
        let price = if self.is_free {
            "Free".into()
        } else {
            self.price_overview.as_ref().map_or_else(
                || "N/A".into(),
                |price_overview| {
                    if price_overview.discount_percent > 0 {
                        format!(
                            "~~{}~~ {} ({}% off)",
                            price_overview.initial_formatted, price_overview.final_formatted, price_overview.discount_percent,
                        )
                    } else {
                        price_overview.final_formatted.clone()
                    }
                },
            )
        };

        self._format()
            .set_image(image)
            .set_description(description)
            .add_field("Release Date", release_date, false)
            .add_field("Price", price, false)
    }

    pub fn format_developers(&self) -> Embed {
        let developers = self.developers.as_ref().map(|developers| developers.join(", "));
        let publishers = self.publishers.as_ref().map(|publishers| publishers.join(", "));

        self._format()
            .set_image(&self.background)
            .add_field("Developer", developers.as_deref().unwrap_or("N/A"), false)
            .add_field("Publisher", publishers.as_deref().unwrap_or("N/A"), false)
            .add_field("Website", self.website.as_deref().unwrap_or("N/A"), false)
    }

    pub fn format_details(&self) -> Embed {
        let categories = self.categories.as_ref().map(|categories| {
            limit_strings(
                    categories.iter().map(|category| {
                        format!(
                            "[{}](https://store.steampowered.com/tags/en/{})",
                            category.description,
                            category.description.replace(' ', "+"),
                        )
                    }),
                    ", ",
                    1024,
                )
        });
        let genres = self.genres.as_ref().map(|genres| {
            limit_strings(
                genres.iter().map(|genre| {
                    format!("[{}](https://store.steampowered.com/genre/{}/)", genre.description, genre.description.replace(' ', "+"),)
                }),
                ", ",
                1024,
            )
        });
        let platforms = {
            let mut platforms = vec![];

            if self.platforms.windows {
                platforms.push("Windows");
            }

            if self.platforms.mac {
                platforms.push("macOS");
            }

            if self.platforms.linux {
                platforms.push("Linux");
            }

            platforms.join(", ")
        };
        let metacritic = self.metacritic.as_ref().map(|metacritic| format!("[{}]({})", metacritic.score, metacritic.url));

        self._format()
            .add_field("Category", categories.as_deref().unwrap_or("N/A"), false)
            .add_field("Genre", genres.as_deref().unwrap_or("N/A"), false)
            .add_field("Platforms", platforms, false)
            .add_field("Metacritic", metacritic.as_deref().unwrap_or("N/A"), false)
    }

    pub fn format_featured_achievements(&self) -> Embed {
        let achievements = self.achievements.as_ref().map(|achievements| {
            limit_strings(
                achievements.highlighted.iter().map(|achievement| format!("[{}]({})", achievement.name, achievement.path)),
                "\n",
                4096,
            )
        });

        self._format().set_description(achievements.as_deref().unwrap_or("N/A"))
    }
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct SteamGameResponse {
    pub success: bool,
    pub data: SteamGame,
}

#[derive(Debug)]
pub struct SteamSearchResult {
    pub name: String,
    pub id: String,
}

impl Steam {
    pub async fn get_game<T: Display>(id: T) -> Result<SteamGame> {
        match REQWEST
            .get("https://store.steampowered.com/api/appdetails")
            .query(&[("cc", "us"), ("appids", id.to_string().as_str())])
            .send()
            .await?
            .json::<Value>()
            .await?
            .as_object_mut()
            .unwrap()
            .remove(&id.to_string())
        {
            Some(value) => Ok(from_value::<SteamGameResponse>(value)?.data),
            None => bail!("Game not found."),
        }
    }

    pub async fn search_game<T: Display>(query: T) -> Result<Vec<SteamSearchResult>> {
        let document = Document::from(
            &REQWEST
                .get("https://store.steampowered.com/search/results")
                .query(&[("category1", "998"), ("term", query.to_string().as_str())]) // category1=998 is games only
                .send()
                .await?
                .text()
                .await?,
        );

        let names = document.select(".title").nodes().iter().map(|node| node.text().to_string()).collect::<Vec<String>>();

        if names.is_empty() {
            bail!("Game not found.");
        }

        let ids = document
            .select("#search_resultsRows a")
            .nodes()
            .iter()
            .map(|node| node.attr("data-ds-appid").unwrap().to_string())
            .collect::<Vec<String>>();

        Ok(names
            .into_iter()
            .enumerate()
            .map(|(index, name)| SteamSearchResult { name, id: ids[index].clone() })
            .filter(
                |result| result.id.chars().all(|char| char.is_numeric()), // Need to filter out subs
            )
            .collect::<Vec<SteamSearchResult>>())
    }
}
