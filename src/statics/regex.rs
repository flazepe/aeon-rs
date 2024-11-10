use regex::{Regex, RegexBuilder};
use std::sync::LazyLock;

pub static MARKDOWN_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\\?[*_~`]").unwrap());
pub static COPYRIGHT_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\((C|P)\)|©|℗").unwrap());
pub static HTTPS_URL_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"https://(.+?)/").unwrap());
pub static BBCODE_REGEX: LazyLock<Regex> =
    LazyLock::new(|| RegexBuilder::new(r"\[/?[bi]\]|\[url=(.+?)\]|\[/url\]").case_insensitive(true).build().unwrap());
