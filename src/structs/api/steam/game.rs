use crate::{
    functions::{format_timestamp, limit_strings, TimestampFormat},
    statics::REQWEST,
    structs::api::steam::{statics::STEAM_EMBED_COLOR, Steam},
};
use anyhow::{bail, Result};
use nipper::Document;
use serde::Deserialize;
use serde_json::{from_value, Value};
use slashook::{chrono::NaiveDateTime, structs::embeds::Embed};

#[derive(Deserialize)]
pub struct SteamGameRequirements {
    pub minimum: String,
    pub recommended: Option<String>,
}

#[derive(Deserialize)]
pub struct SteamGamePriceOverview {
    pub currency: String,
    pub initial: u64,

    #[serde(rename = "final")]
    pub final_price: u64,

    pub discount_percent: u64,
    pub initial_formatted: String,
    pub final_formatted: String,
}

#[derive(Deserialize)]
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

#[derive(Deserialize)]
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

#[derive(Deserialize)]
pub struct SteamPlatforms {
    pub windows: bool,
    pub mac: bool,
    pub linux: bool,
}

#[derive(Deserialize)]
pub struct SteamMetacritic {
    pub score: u64,
    pub url: String,
}

#[derive(Deserialize)]
pub struct SteamCategory {
    pub id: u64,
    pub description: String,
}

#[derive(Deserialize)]
pub struct SteamGenre {
    pub id: String, // Why is this one inside a string
    pub description: String,
}

#[derive(Deserialize)]
pub struct SteamScreenshot {
    pub id: u64,
    pub path_thumbnail: String,
    pub path_full: String,
}

#[derive(Deserialize)]
pub struct SteamMovie {
    pub id: u64,
    pub name: String,
    pub thumbnail: String,
    pub webm: SteamMovieSizes,
    pub mp4: SteamMovieSizes,
    pub highlight: bool,
}

#[derive(Deserialize)]
pub struct SteamMovieSizes {
    #[serde(rename = "480")]
    pub min: String,

    pub max: String,
}

#[derive(Deserialize)]
pub struct SteamRecommendations {
    pub total: u64,
}

#[derive(Deserialize)]
pub struct SteamAchievements {
    pub total: u64,
    pub highlighted: Vec<SteamAchievement>,
}

#[derive(Deserialize)]
pub struct SteamAchievement {
    pub name: String,
    pub path: String,
}

#[derive(Deserialize)]
pub struct SteamReleaseDate {
    pub coming_soon: bool,
    pub date: String,
}

#[derive(Deserialize)]
pub struct SteamSupportInfo {
    pub url: String,
    pub email: String,
}

#[derive(Deserialize)]
pub struct SteamContentDescriptor {
    pub ids: Vec<u64>,
    pub notes: Option<String>,
}

#[derive(Deserialize)]
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
        Embed::new()
            .set_color(STEAM_EMBED_COLOR)
            .unwrap_or_default()
            .set_title(&self.name)
            .set_url(format!("https://store.steampowered.com/app/{}", self.id))
    }

    pub fn format(&self) -> Embed {
        self._format()
            .set_image(&self.header_image)
            .set_description(limit_strings(Document::from(&self.short_description).select("body").text().split('\n'), "\n", 4096))
            .add_field(
                "Release Date",
                self.release_date.as_ref().map_or("TBA".into(), |release_date| {
                    format!(
                        "{}{}",
                        format_timestamp(
                            NaiveDateTime::parse_from_str(&format!("{} 00:00", release_date.date), "%b %-d, %Y %R").unwrap().timestamp(),
                            TimestampFormat::Full,
                        ),
                        match release_date.coming_soon {
                            true => " (coming soon)",
                            false => "",
                        },
                    )
                }),
                false,
            )
            .add_field(
                "Price",
                match self.is_free {
                    true => "Free".into(),
                    false => {
                        self.price_overview.as_ref().map_or("N/A".into(), |price_overview| match price_overview.discount_percent > 0 {
                            true => format!(
                                "~~{}~~ {} ({}% off)",
                                price_overview.initial_formatted, price_overview.final_formatted, price_overview.discount_percent,
                            ),
                            false => price_overview.final_formatted.clone(),
                        })
                    },
                },
                false,
            )
    }

    pub fn format_developers(&self) -> Embed {
        self._format()
            .set_image(&self.background)
            .add_field("Developers", self.developers.as_ref().map_or("N/A".into(), |developers| developers.join(", ")), false)
            .add_field("Publishers", self.publishers.as_ref().map_or("N/A".into(), |publishers| publishers.join(", ")), false)
            .add_field("Website", self.website.as_ref().unwrap_or(&"N/A".into()), false)
    }

    pub fn format_details(&self) -> Embed {
        self._format()
            .add_field(
                "Category",
                self.categories.as_ref().map_or("N/A".into(), |categories| {
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
                }),
                false,
            )
            .add_field(
                "Genre",
                self.genres.as_ref().map_or("N/A".into(), |genres| {
                    limit_strings(
                        genres.iter().map(|genre| {
                            format!(
                                "[{}](https://store.steampowered.com/genre/{}/)",
                                genre.description,
                                genre.description.replace(' ', "+"),
                            )
                        }),
                        ", ",
                        1024,
                    )
                }),
                false,
            )
            .add_field(
                "Platforms",
                {
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
                },
                false,
            )
            .add_field(
                "Metacritic",
                self.metacritic.as_ref().map_or("N/A".into(), |metacritic| format!("[{}]({})", metacritic.score, metacritic.url)),
                false,
            )
    }

    pub fn format_featured_achievements(&self) -> Embed {
        self._format().set_description(self.achievements.as_ref().map_or("N/A".into(), |achievements| {
            limit_strings(
                achievements.highlighted.iter().map(|achievement| format!("[{}]({})", achievement.name, achievement.path)),
                "\n",
                4096,
            )
        }))
    }
}

#[derive(Deserialize)]
pub struct SteamGameResponse {
    pub success: bool,
    pub data: SteamGame,
}

pub struct SteamSearchResult {
    pub name: String,
    pub id: String,
}

impl Steam {
    pub async fn get_game<T: ToString>(id: T) -> Result<SteamGame> {
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

    pub async fn search_game<T: ToString>(query: T) -> Result<Vec<SteamSearchResult>> {
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
