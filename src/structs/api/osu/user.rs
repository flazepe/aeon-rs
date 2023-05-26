use crate::{
    functions::{format_timestamp, TimestampFormat},
    macros::yes_no,
    statics::emojis::{
        OFFLINE_EMOJI, ONLINE_EMOJI, OSU_A_EMOJI, OSU_SH_EMOJI, OSU_SUPPORTER_1_EMOJI, OSU_SUPPORTER_2_EMOJI, OSU_SUPPORTER_3_EMOJI,
        OSU_S_EMOJI, OSU_XH_EMOJI, OSU_X_EMOJI,
    },
    structs::{api::osu::Osu, duration::Duration},
    traits::Commas,
};
use anyhow::{bail, Result};
use serde::Deserialize;
use slashook::{chrono::DateTime, structs::embeds::Embed};
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Deserialize)]
pub struct OsuUser {
    pub avatar_url: String,
    pub country_code: String,
    pub default_group: String,
    pub id: u64,
    pub is_active: bool,
    pub is_bot: bool,
    pub is_deleted: bool,
    pub is_online: bool,
    pub is_supporter: bool,
    pub last_visit: Option<String>,
    pub pm_friends_only: bool,
    pub profile_colour: Option<String>,
    pub username: String,
    pub cover_url: String,
    pub discord: Option<String>,
    pub has_supported: bool,
    pub interests: Option<String>,
    pub join_date: String,
    pub kudosu: OsuUserKudosu,
    pub location: Option<String>,
    pub max_blocks: u64,
    pub max_friends: u64,
    pub occupation: Option<String>,
    pub playmode: OsuMode,
    pub playstyle: Option<Vec<OsuUserPlaystyle>>,
    pub post_count: u64,
    pub profile_order: Vec<OsuUserProfileSection>,
    pub title: Option<String>,
    pub title_url: Option<String>,
    pub twitter: Option<String>,
    pub website: Option<String>,
    pub country: OsuUserCountry,
    pub cover: OsuUserCover,
    // pub account_history: Vec<String>,
    // pub active_tournament_banner: Option<String>,
    pub badges: Vec<OsuUserBadge>,
    pub beatmap_playcounts_count: u64,
    pub comments_count: u64,
    pub favourite_beatmapset_count: u64,
    pub follower_count: u64,
    pub graveyard_beatmapset_count: u64,
    pub groups: Vec<OsuUserGroup>,
    pub guest_beatmapset_count: u64,
    pub loved_beatmapset_count: u64,
    pub mapping_follower_count: u64,
    pub monthly_playcounts: Vec<OsuUserDateCount>,
    pub nominated_beatmapset_count: u64,
    pub page: OsuUserPage,
    pub pending_beatmapset_count: u64,
    pub previous_usernames: Vec<String>,
    pub rank_highest: OsuUserRankHighest,
    pub ranked_beatmapset_count: u64,
    pub replays_watched_counts: Vec<OsuUserDateCount>,
    pub scores_best_count: u64,
    pub scores_first_count: u64,
    pub scores_pinned_count: u64,
    pub scores_recent_count: u64,
    pub statistics: OsuUserStatistics,
    pub support_level: u8,
    pub user_achievements: Vec<OsuUserAchievement>,
    pub rank_history: OsuUserRankHistory,
    // pub rankHistory: OsuUserRankHistory,
    pub ranked_and_approved_beatmapset_count: u64,
    pub unranked_beatmapset_count: u64,
}

#[derive(Deserialize)]
pub struct OsuUserBadge {
    pub awarded_at: String,
    pub description: String,
    pub image_url: String,
    pub url: String,
}

#[derive(Deserialize)]
pub struct OsuUserDateCount {
    pub start_date: String,
    pub count: u64,
}

#[derive(Deserialize)]
pub struct OsuUserAchievement {
    pub achieved_at: String,
    pub achievement_id: u64,
}

#[derive(Deserialize)]
pub struct OsuUserCountry {
    pub code: String,
    pub name: String,
}

#[derive(Deserialize)]
pub struct OsuUserCover {
    pub custom_url: Option<String>,
    pub url: String,
    pub id: Option<String>,
}

#[derive(Deserialize)]
pub struct OsuUserGradeCount {
    pub ss: i64,
    pub ssh: i64,
    pub s: i64,
    pub sh: i64,
    pub a: i64,
}

#[derive(Deserialize)]
pub struct OsuUserGroup {
    pub colour: String,
    pub has_listing: bool,
    pub has_playmodes: bool,
    pub id: u64,
    pub identifier: String,
    pub is_probationary: bool,
    pub name: String,
    pub short_name: String,
    pub playmodes: Option<Vec<OsuMode>>,
}

#[derive(Deserialize)]
pub struct OsuUserKudosu {
    pub total: u64,
    pub available: u64,
}

#[derive(Deserialize)]
pub struct OsuUserLevel {
    pub current: u64,
    pub progress: u64,
}

#[derive(Deserialize)]
pub struct OsuUserPage {
    pub html: String,
    pub raw: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum OsuUserPlaystyle {
    Mouse,
    Keyboard,
    Tablet,
    Touch,
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OsuUserProfileSection {
    Me,
    RecentActivity,
    TopRanks,
    Historical,
    Medals,
    Beatmaps,
    Kudosu,
}

#[derive(Deserialize)]
pub struct OsuUserRank {
    pub country: Option<u64>,
}

#[derive(Deserialize)]
pub struct OsuUserRankHighest {
    pub rank: u64,
    pub updated_at: String,
}

#[derive(Deserialize)]
pub struct OsuUserRankHistory {
    pub mode: OsuMode,
    pub data: Vec<u64>,
}

#[derive(Deserialize)]
pub struct OsuUserStatistics {
    pub count_100: u64,
    pub count_300: u64,
    pub count_50: u64,
    pub count_miss: u64,
    pub level: OsuUserLevel,
    pub global_rank: Option<u64>,
    pub global_rank_exp: Option<u64>,
    pub pp: f64,
    pub pp_exp: f64,
    pub ranked_score: u64,
    pub hit_accuracy: f64,
    pub play_count: u64,
    pub play_time: u64,
    pub total_score: u64,
    pub total_hits: u64,
    pub maximum_combo: u64,
    pub replays_watched_by_others: u64,
    pub is_ranked: bool,
    pub grade_counts: OsuUserGradeCount,
    pub country_rank: Option<u64>,
    pub rank: OsuUserRank,
}

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OsuMode {
    Osu,
    Taiko,
    Fruits,
    Mania,
}

impl Display for OsuMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "{}",
            match self {
                OsuMode::Osu => "osu!",
                OsuMode::Taiko => "osu!taiko",
                OsuMode::Fruits => "osu!catch",
                OsuMode::Mania => "osu!mania",
            },
        )
    }
}

impl OsuUser {
    fn _format(&self) -> Embed {
        Embed::new()
            .set_color("#ff69b4")
            .unwrap_or_default()
            .set_thumbnail(match self.avatar_url.starts_with('/') {
                true => format!("https://osu.ppy.sh{}", self.avatar_url),
                false => self.avatar_url.clone(),
            })
            .set_title(
                format!(
                    "{} {}{}",
                    match self.is_online {
                        true => ONLINE_EMOJI,
                        false => OFFLINE_EMOJI,
                    },
                    match self.support_level {
                        1 => format!("{OSU_SUPPORTER_1_EMOJI} "),
                        2 => format!("{OSU_SUPPORTER_2_EMOJI} "),
                        3 => format!("{OSU_SUPPORTER_3_EMOJI} "),
                        _ => "".into(),
                    },
                    self.username,
                )
                .trim(),
            )
            .set_url(format!("https://osu.ppy.sh/users/{}", self.id))
    }

    pub fn format(&self) -> Embed {
        self._format()
            .add_field("Mode", self.rank_history.mode.to_string(), true)
            .add_field(
                "Rank",
                format!(
                    "#{} (#{} peak)",
                    self.statistics.global_rank.map_or("-".into(), |global_rank| global_rank.commas()),
                    self.rank_highest.rank.commas(),
                ),
                true,
            )
            .add_field("ID", self.id, true)
            .add_field("Followers", self.follower_count, true)
            .add_field(
                "Country",
                format!(
                    ":flag_{}: [{}](https://osu.ppy.sh/rankings/osu/performance?country={})",
                    self.country.code.to_lowercase(),
                    self.country.name,
                    self.country_code,
                ),
                true,
            )
            .add_field(
                "Playstyle",
                self.playstyle.as_ref().map_or("N/A".into(), |playstyle| {
                    playstyle.iter().map(|entry| format!("{entry:?}")).collect::<Vec<String>>().join(", ")
                }),
                true,
            )
            .add_field(
                "Created",
                format_timestamp(DateTime::parse_from_rfc3339(self.join_date.as_str()).unwrap().timestamp(), TimestampFormat::Full),
                false,
            )
    }

    pub fn format_about(&self) -> Embed {
        self._format()
            .add_field("Location", self.location.as_ref().unwrap_or(&"N/A".into()), true)
            .add_field("Interests", self.interests.as_ref().unwrap_or(&"N/A".into()), true)
            .add_field("Occupation", self.occupation.as_ref().unwrap_or(&"N/A".into()), true)
            .add_field("Website", self.website.as_ref().unwrap_or(&"N/A".into()), true)
            .add_field(
                "Twitter",
                self.twitter.as_ref().map_or("N/A".into(), |twitter| format!("[@{twitter}](https://twitter.com/{twitter})")),
                true,
            )
            .add_field("Discord", self.discord.as_ref().unwrap_or(&"N/A".into()), true)
    }

    pub fn format_statistics(&self) -> Embed {
        self._format()
            .add_field("Performance Points", format!("{}pp", format!("{:.2}", self.statistics.pp).commas()), true)
            .add_field("Accuracy", format!("{:.2}%", self.statistics.hit_accuracy), true)
            .add_field("Level", format!("{} ({}%)", self.statistics.level.current, self.statistics.level.progress), true)
            .add_field("Total Hits", self.statistics.total_hits.commas(), true)
            .add_field("Maximum Combo", self.statistics.maximum_combo.commas(), true)
            .add_field("First Place Ranks", self.scores_first_count.commas(), true)
            .add_field(
                "Score",
                format!("{} ({} ranked)", self.statistics.total_score.commas(), self.statistics.ranked_score.commas(),),
                false,
            )
            .add_field(
                "Play Count",
                format!(
                    "{} ({})",
                    self.statistics.play_count.commas(),
                    Duration::new().parse(format!("{}s", self.statistics.play_time)).unwrap(),
                ),
                false,
            )
            .add_field("Replays Watched by Others", self.statistics.replays_watched_by_others.commas(), false)
            .add_field(
                "Grades",
                [
                    format!("{OSU_X_EMOJI} {}", self.statistics.grade_counts.ss.commas()),
                    format!("{OSU_XH_EMOJI} {}", self.statistics.grade_counts.ssh.commas()),
                    format!("{OSU_S_EMOJI} {}", self.statistics.grade_counts.s.commas()),
                    format!("{OSU_SH_EMOJI} {}", self.statistics.grade_counts.sh.commas()),
                    format!("{OSU_A_EMOJI} {}", self.statistics.grade_counts.a.commas()),
                ]
                .join("\n"),
                false,
            )
    }

    pub fn format_website_statistics(&self) -> Embed {
        self._format()
            .add_field(
                "Previous Username",
                match self.previous_usernames.is_empty() {
                    true => "-".into(),
                    false => self.previous_usernames.join(", "),
                },
                false,
            )
            .add_field(
                "Supporter",
                match self.is_supporter {
                    true => format!("Yes (level {})", self.support_level),
                    false => format!(
                        "No ({})",
                        match self.has_supported {
                            true => "has supported before",
                            false => "had never supported before",
                        },
                    ),
                },
                false,
            )
            .add_field("Bot", yes_no!(self.is_bot), true)
            .add_field("Forum Posts", format!("[{}](https://osu.ppy.sh/users/{}/posts)", self.post_count.commas(), self.id), true)
            .add_field("Comments", self.comments_count.commas(), true)
            .add_field(
                "Kudosu!",
                format!(
                    "[{} ({} available)](https://osu.ppy.sh/users/{}#kudosu)",
                    self.kudosu.total.commas(),
                    self.kudosu.available.commas(),
                    self.id,
                ),
                false,
            )
            .add_field(
                "Beatmaps",
                [
                    format!("{} favorite", self.favourite_beatmapset_count.commas()),
                    format!("{} ranked and approved", self.ranked_and_approved_beatmapset_count.commas()),
                    format!("{} as guest", self.guest_beatmapset_count.commas()),
                    format!("{} loved", self.loved_beatmapset_count.commas()),
                    format!("{} pending", self.unranked_beatmapset_count.commas()),
                    format!("{} graveyarded", self.graveyard_beatmapset_count.commas()),
                ]
                .join("\n"),
                false,
            )
    }
}

impl Osu {
    pub async fn get_user<T: Display, U: ToString>(user: T, mode: U) -> Result<OsuUser> {
        let mut mode = mode.to_string();

        if !["osu", "taiko", "fruits", "mania"].contains(&mode.as_str()) {
            mode = "".into();
        }

        match Osu::query(format!("users/{user}/{mode}")).await {
            Ok(user) => Ok(user),
            Err(_) => bail!("User not found."),
        }
    }
}
