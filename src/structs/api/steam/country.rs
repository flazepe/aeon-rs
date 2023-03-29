use std::collections::HashMap;

pub struct SteamCountry<'a> {
    pub name: &'a str,
    pub states: HashMap<&'a str, &'a str>,
}
