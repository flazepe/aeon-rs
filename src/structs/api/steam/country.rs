use crate::statics::steam::*;

pub struct SteamCountry<'a> {
    pub code: &'a str,
    pub name: &'a str,
    pub states: &'a [[&'a str; 2]],
}

impl<'a> SteamCountry<'a> {
    pub fn get<T: ToString>(country_code: T) -> Option<&'a Self> {
        STEAM_COUNTRIES
            .iter()
            .find(|country| country.code == country_code.to_string())
    }
}
