use crate::constants::*;

pub struct SteamCountry<'a> {
    pub code: &'a str,
    pub name: &'a str,
    pub states: &'a [[&'a str; 2]],
}

impl<'a> SteamCountry<'a> {
    pub fn get(country_code: &str) -> Option<&Self> {
        STEAM_COUNTRIES
            .iter()
            .find(|country| country.code == country_code)
    }
}
