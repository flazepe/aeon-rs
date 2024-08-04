use std::collections::HashMap;

#[derive(Debug)]
pub struct SteamCountry<'a> {
    pub name: &'a str,
    pub states: HashMap<&'a str, &'a str>,
}
