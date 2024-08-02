use once_cell::sync::Lazy;
use regex::{Regex, RegexBuilder};

pub static MARKDOWN_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\\?[*_~`]").unwrap());
pub static COPYRIGHT_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\((C|P)\)|©|℗").unwrap());
pub static HTTPS_URL_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"https://(.+?)/").unwrap());
pub static BBCODE_REGEX: Lazy<Regex> =
    Lazy::new(|| RegexBuilder::new(r"\[/?[bi]\]|\[url=(.+?)\]|\[/url\]").case_insensitive(true).build().unwrap());
