use once_cell::sync::Lazy;
use std::collections::HashMap;

pub static OCR_LANGUAGES: Lazy<HashMap<&str, &str>> = Lazy::new(|| {
    HashMap::from([
        ("ara", "Arabic"),
        ("chi_sim", "Chinese Simplified"),
        ("chi_tra", "Chinese Traditional"),
        ("eng", "English"),
        ("fin", "Finnish"),
        ("fra", "French"),
        ("deu", "German"),
        ("heb", "Hebrew"),
        ("hin", "Hindi"),
        ("hun", "Hungarian"),
        ("ind", "Indonesian"),
        ("ita", "Italian"),
        ("jpn", "Japanese"),
        ("kor", "Korean"),
        ("lat", "Latin"),
        ("msa", "Malay"),
        ("nor", "Norwegian"),
        ("pol", "Polish"),
        ("por", "Portuguese"),
        ("rus", "Russian"),
        ("spa", "Spanish"),
        ("swe", "Swedish"),
        ("tam", "Tamil"),
        ("tel", "Telugu"),
        ("tha", "Thai"),
        ("tur", "Turkish"),
        ("ukr", "Ukrainian"),
        ("urd", "Urdu"),
        ("vie", "Vietnamese"),
        ("yid", "Yiddish"),
    ])
});
