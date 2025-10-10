use regex::{Regex, RegexBuilder};
use std::sync::LazyLock;

pub static EMOJI_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^(?:<a?:\w{2,32}:)?(\d{17,21})>?$").unwrap());
pub static MARKDOWN_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\\?[*_~`]").unwrap());
pub static COPYRIGHT_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\((C|P)\)|©|℗").unwrap());
pub static HTTPS_URL_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"https://(.+?)/").unwrap());
pub static URL_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"https?://(www\.)?[-a-zA-Z0-9@:%._\+~#=]{1,256}\.[a-zA-Z0-9()]{1,6}\b([-a-zA-Z0-9()@:%_\+.~#?&//=]*)").unwrap()
});
pub static DISCORD_URL_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    let url_regex = URL_REGEX.as_str();

    let suppressed_url = format!("<(?<suppressed_url>{url_regex})>");
    let normal_url = format!(r"(?<normal_url>{url_regex})");

    Regex::new(&format!(r"{suppressed_url}|{normal_url}")).unwrap()
});
pub static SPOILER_REGEX: LazyLock<Regex> =
    LazyLock::new(|| RegexBuilder::new(r"\|\|(.+?)\|\|").dot_matches_new_line(true).build().unwrap());
pub static BBCODE_REGEX: LazyLock<Regex> =
    LazyLock::new(|| RegexBuilder::new(r"\[/?[bi]\]|\[url=(.+?)\]|\[/url\]").case_insensitive(true).build().unwrap());
