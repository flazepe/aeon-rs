use regex::{Regex, RegexBuilder};
use std::sync::LazyLock;

pub static EMOJI_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^(?:<a?:\w{2,32}:)?(\d{17,21})>?$").unwrap());
pub static MARKDOWN_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\\?[*_~`]").unwrap());
pub static COPYRIGHT_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\((C|P)\)|©|℗").unwrap());
pub static HTTPS_URL_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"https://(.+?)/").unwrap());
pub static URL_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\bhttps?://(www\.)?[-a-zA-Z0-9@:%._\+~#=]{1,256}\.[a-zA-Z0-9()]{1,6}\b([-a-zA-Z0-9()@:%_\+.~#?&//=]*)\b").unwrap()
});
pub static OG_IMAGE_TAG_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"<meta\s*(content="(.+)")?\s*property="og:image"\s*(content="(.+)")?\s*/?>"#).unwrap());
pub static BBCODE_REGEX: LazyLock<Regex> =
    LazyLock::new(|| RegexBuilder::new(r"\[/?[bi]\]|\[url=(.+?)\]|\[/url\]").case_insensitive(true).build().unwrap());
