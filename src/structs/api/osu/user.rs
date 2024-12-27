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
use anyhow::{Context, Result};
use serde::Deserialize;
use slashook::{chrono::DateTime, structs::embeds::Embed};
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
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

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct OsuUserBadge {
    pub awarded_at: String,
    pub description: String,
    pub image_url: String,
    pub url: String,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct OsuUserDateCount {
    pub start_date: String,
    pub count: u64,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct OsuUserAchievement {
    pub achieved_at: String,
    pub achievement_id: u64,
}

#[derive(Deserialize, Debug)]
pub struct OsuUserCountry {
    pub code: String,
    pub name: String,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct OsuUserCover {
    pub custom_url: Option<String>,
    pub url: String,
    pub id: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct OsuUserGradeCount {
    pub ss: i64,
    pub ssh: i64,
    pub s: i64,
    pub sh: i64,
    pub a: i64,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
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

#[derive(Deserialize, Debug)]
pub struct OsuUserKudosu {
    pub total: u64,
    pub available: u64,
}

#[derive(Deserialize, Debug)]
pub struct OsuUserLevel {
    pub current: u64,
    pub progress: u64,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
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

#[derive(Deserialize, Debug)]
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

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct OsuUserRank {
    pub country: Option<u64>,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct OsuUserRankHighest {
    pub rank: u64,
    pub updated_at: String,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct OsuUserRankHistory {
    pub mode: OsuMode,
    pub data: Vec<u64>,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
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

#[derive(Deserialize, Debug)]
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
                Self::Osu => "osu!",
                Self::Taiko => "osu!taiko",
                Self::Fruits => "osu!catch",
                Self::Mania => "osu!mania",
            },
        )
    }
}

impl OsuUser {
    fn _format(&self) -> Embed {
        let thumbnail =
            if self.avatar_url.starts_with('/') { format!("https://osu.ppy.sh{}", self.avatar_url) } else { self.avatar_url.clone() };
        let title = format!(
            "{} {}{}",
            if self.is_online { ONLINE_EMOJI } else { OFFLINE_EMOJI },
            match self.support_level {
                1 => format!("{OSU_SUPPORTER_1_EMOJI} "),
                2 => format!("{OSU_SUPPORTER_2_EMOJI} "),
                3 => format!("{OSU_SUPPORTER_3_EMOJI} "),
                _ => "".into(),
            },
            self.username,
        );
        let url = format!("https://osu.ppy.sh/users/{}", self.id);

        Embed::new().set_color("#ff69b4").unwrap_or_default().set_thumbnail(thumbnail).set_title(title).set_url(url)
    }

    pub fn format(&self) -> Embed {
        let mode = self.rank_history.mode.to_string();
        let rank = format!(
            "#{} (#{} peak)",
            self.statistics.global_rank.map(|global_rank| global_rank.commas()).as_deref().unwrap_or("-"),
            self.rank_highest.rank.commas(),
        );
        let id = self.id;
        let followers = self.follower_count;
        let country = format!(
            ":flag_{}: [{}](https://osu.ppy.sh/rankings/osu/performance?country={})",
            self.country.code.to_lowercase(),
            self.country.name,
            self.country_code,
        );
        let playstyle = self
            .playstyle
            .as_ref()
            .map(|playstyle| playstyle.iter().map(|entry| format!("{entry:?}")).collect::<Vec<String>>().join(", "))
            .unwrap_or_else(|| "N/A".into());
        let created = format_timestamp(DateTime::parse_from_rfc3339(self.join_date.as_str()).unwrap().timestamp(), TimestampFormat::Full);

        self._format()
            .add_field("Mode", mode, true)
            .add_field("Rank", rank, true)
            .add_field("ID", id, true)
            .add_field("Followers", followers, true)
            .add_field("Country", country, true)
            .add_field("Playstyle", playstyle, true)
            .add_field("Created", created, false)
    }

    pub fn format_about(&self) -> Embed {
        let location = self.location.as_deref().unwrap_or("N/A");
        let interests = self.interests.as_deref().unwrap_or("N/A");
        let occupation = self.occupation.as_deref().unwrap_or("N/A");
        let website = self.website.as_deref().unwrap_or("N/A");
        let twitter = self.twitter.as_ref().map(|twitter| format!("[@{twitter}](https://x.com/{twitter})")).unwrap_or_else(|| "N/A".into());
        let discord = self.discord.as_deref().unwrap_or("N/A");

        self._format()
            .add_field("Location", location, true)
            .add_field("Interests", interests, true)
            .add_field("Occupation", occupation, true)
            .add_field("Website", website, true)
            .add_field("Twitter", twitter, true)
            .add_field("Discord", discord, true)
    }

    pub fn format_statistics(&self) -> Embed {
        let pp = format!("{}pp", format!("{:.2}", self.statistics.pp).commas());
        let accuracy = format!("{:.2}%", self.statistics.hit_accuracy);
        let level = format!("{} ({}%)", self.statistics.level.current, self.statistics.level.progress);
        let total_hits = self.statistics.total_hits.commas();
        let maximum_combo = self.statistics.maximum_combo.commas();
        let first_place_ranks = self.scores_first_count.commas();
        let scores = format!("{} ({} ranked)", self.statistics.total_score.commas(), self.statistics.ranked_score.commas());
        let play_count = format!(
            "{} ({})",
            self.statistics.play_count.commas(),
            Duration::new().parse(format!("{}s", self.statistics.play_time)).unwrap(),
        );
        let replays_watched_by_others = self.statistics.replays_watched_by_others.commas();
        let grades = [
            format!("{OSU_X_EMOJI} {}", self.statistics.grade_counts.ss.commas()),
            format!("{OSU_XH_EMOJI} {}", self.statistics.grade_counts.ssh.commas()),
            format!("{OSU_S_EMOJI} {}", self.statistics.grade_counts.s.commas()),
            format!("{OSU_SH_EMOJI} {}", self.statistics.grade_counts.sh.commas()),
            format!("{OSU_A_EMOJI} {}", self.statistics.grade_counts.a.commas()),
        ]
        .join("\n");

        self._format()
            .add_field("PP", pp, true)
            .add_field("Accuracy", accuracy, true)
            .add_field("Level", level, true)
            .add_field("Total Hits", total_hits, true)
            .add_field("Maximum Combo", maximum_combo, true)
            .add_field("First Place Ranks", first_place_ranks, true)
            .add_field("Score", scores, false)
            .add_field("Play Count", play_count, false)
            .add_field("Replays Watched by Others", replays_watched_by_others, false)
            .add_field("Grades", grades, false)
    }

    pub fn format_website_statistics(&self) -> Embed {
        let previous_usernames = if self.previous_usernames.is_empty() { "-".into() } else { self.previous_usernames.join(", ") };
        let supporter = if self.is_supporter {
            format!("Yes (level {})", self.support_level)
        } else {
            format!("No ({})", if self.has_supported { "has supported before" } else { "had never supported before" })
        };
        let bot = yes_no!(self.is_bot);
        let forum_posts = format!("[{}](https://osu.ppy.sh/users/{}/posts)", self.post_count.commas(), self.id);
        let comments = self.comments_count.commas();
        let kudosu = format!(
            "[{} ({} available)](https://osu.ppy.sh/users/{}#kudosu)",
            self.kudosu.total.commas(),
            self.kudosu.available.commas(),
            self.id,
        );
        let beatmaps = [
            format!("{} favorite", self.favourite_beatmapset_count.commas()),
            format!("{} ranked and approved", self.ranked_and_approved_beatmapset_count.commas()),
            format!("{} as guest", self.guest_beatmapset_count.commas()),
            format!("{} loved", self.loved_beatmapset_count.commas()),
            format!("{} pending", self.unranked_beatmapset_count.commas()),
            format!("{} graveyarded", self.graveyard_beatmapset_count.commas()),
        ]
        .join("\n");

        self._format()
            .add_field("Previous Username", previous_usernames, false)
            .add_field("Supporter", supporter, false)
            .add_field("Bot", bot, true)
            .add_field("Forum Posts", forum_posts, true)
            .add_field("Comments", comments, true)
            .add_field("Kudosu!", kudosu, false)
            .add_field("Beatmaps", beatmaps, false)
    }
}

impl Osu {
    pub async fn get_user<T: Display, U: Display>(user: T, mode: U) -> Result<OsuUser> {
        let mut mode = mode.to_string();

        if !["osu", "taiko", "fruits", "mania"].contains(&mode.as_str()) {
            mode = "".into();
        }

        Self::query(format!("users/{user}/{mode}")).await.context("User not found.")
    }
}
