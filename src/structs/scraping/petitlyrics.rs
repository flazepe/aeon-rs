use crate::{
    functions::limit_strings,
    statics::{colors::PRIMARY_COLOR, REQWEST},
    structs::api::google::{statics::GOOGLE_TRANSLATE_LANGUAGES, Google},
};
use anyhow::{bail, Result};
use base64::{prelude::BASE64_STANDARD, Engine};
use http_req::{
    request::{Method, Request},
    uri::Uri,
};
use nipper::Document;
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use serde_repr::Serialize_repr;
use slashook::structs::embeds::Embed;
use std::{convert::TryFrom, fmt::Display};

#[derive(Debug)]
pub struct PetitLyrics {
    pub artist: String,
    pub title: String,
    pub lyrics_id: String,
}

#[derive(Serialize)]
struct PetitLyricsSearchParams {
    artist: Option<String>,
    artist_opt: Option<PetitLyricsSearchOpt>,
    title: Option<String>,
    title_opt: Option<PetitLyricsSearchOpt>,
    lyrics: Option<String>,
}

#[derive(Serialize_repr)]
#[repr(u8)]
#[allow(dead_code)]
enum PetitLyricsSearchOpt {
    StartsWith,
    PartialMatch,
    PerfectMatch,
}

#[derive(Deserialize)]
struct PetitLyricsRawLyricsLine {
    lyrics: String,
}

impl PetitLyrics {
    async fn search(params: PetitLyricsSearchParams) -> Result<Vec<Self>> {
        let mut results = vec![];

        let body = REQWEST.post("https://petitlyrics.com/search_lyrics").form(&params).send().await?.text().await?;

        for selection in Document::from(&body).select("#lyrics_list tr").iter() {
            let artist = selection.select(".lyrics-list-artist").text().trim().into();
            let title = selection.select(".lyrics-list-title").text().trim().into();

            let Some(url) = selection.select("td:nth-child(2) a").attr("href") else { continue };
            let lyrics_id = url.replace("/lyrics/", "");

            results.push(Self { artist, title, lyrics_id });
        }

        if results.is_empty() {
            bail!("No results found.");
        }

        Ok(results)
    }

    pub async fn search_partial<T: Display, U: Display, V: Display>(
        artist: Option<T>,
        title: Option<U>,
        lyrics: Option<V>,
    ) -> Result<Vec<Self>> {
        Self::search(PetitLyricsSearchParams {
            artist: artist.as_ref().map(|artist| artist.to_string()),
            artist_opt: artist.map(|_| PetitLyricsSearchOpt::PartialMatch),
            title: title.as_ref().map(|title| title.to_string()),
            title_opt: title.map(|_| PetitLyricsSearchOpt::PartialMatch),
            lyrics: lyrics.map(|lyrics| lyrics.to_string()),
        })
        .await
    }

    pub async fn search_perfect<T: Display, U: Display, V: Display>(
        artist: Option<T>,
        title: Option<U>,
        lyrics: Option<V>,
    ) -> Result<Vec<Self>> {
        Self::search(PetitLyricsSearchParams {
            artist: artist.as_ref().map(|artist| artist.to_string()),
            artist_opt: artist.map(|_| PetitLyricsSearchOpt::PerfectMatch),
            title: title.as_ref().map(|title| title.to_string()),
            title_opt: title.map(|_| PetitLyricsSearchOpt::PerfectMatch),
            lyrics: lyrics.map(|lyrics| lyrics.to_string()),
        })
        .await
    }

    pub async fn get_formatted_lyrics<T: Display>(&self, translate_language: Option<T>) -> Result<Embed> {
        let res = REQWEST.get("https://petitlyrics.com/lib/pl-lib.js").send().await?;
        let cookie = res.headers().get("set-cookie").map(|cookie| cookie.to_str().unwrap().to_string()).unwrap_or_default();
        let csrf_token = res.text().await?.split('\'').nth(3).map(|str| str.to_string()).unwrap_or_default();

        // You have to fetch the lyrics page with the cookie first before fetching the ajax URL
        REQWEST.get(format!("https://petitlyrics.com/lyrics/{}", self.lyrics_id)).header("cookie", &cookie).send().await?.text().await?;

        // Fetch the ajax URL with a request client that preserves header cases (for X-CSRF-Token). I know, it's stupid.
        let mut body = Vec::new();
        let _ = Request::new(&Uri::try_from("https://petitlyrics.com/com/get_lyrics.ajax")?)
            .method(Method::POST)
            .header("content-type", "application/x-www-form-urlencoded")
            .header("cookie", &cookie)
            .header("X-CSRF-Token", &csrf_token)
            .header("x-requested-with", "XMLHttpRequest")
            .body(format!("lyrics_id={}", self.lyrics_id).as_bytes())
            .send(&mut body)?;

        // Parse lyrics from the body
        let mut lyrics = vec![];

        for entry in from_str::<Vec<PetitLyricsRawLyricsLine>>(&String::from_utf8_lossy(&body))? {
            let line = String::from_utf8(BASE64_STANDARD.decode(&entry.lyrics)?)?.replace("<♪>", "");

            if lyrics.is_empty() {
                lyrics.push(line);
            } else if line.chars().next().map_or(false, |char| char.is_lowercase()) {
                lyrics.last_mut().unwrap().push_str(&format!(" {line}"));
            } else {
                lyrics.push(line);
            }
        }

        // Translate lines
        let translate_language = translate_language.map(|translate_language| translate_language.to_string()).unwrap_or_default();

        if GOOGLE_TRANSLATE_LANGUAGES.contains_key(translate_language.as_str()) {
            let translated = Google::translate(lyrics.join("\n"), "auto", &translate_language).await?.translation;
            let mut translated_lines = translated.split('\n');

            for line in lyrics.iter_mut() {
                let Some(translated_line) = translated_lines.next() else { break };

                if !translated_line.is_empty() {
                    line.push_str(&format!("\n-# {translated_line}"));
                }
            }
        }

        let title = format!("{} - {}", self.artist, self.title).chars().take(256).collect::<String>();
        let description = limit_strings(lyrics, "\n", 4096);

        Ok(Embed::new().set_color(PRIMARY_COLOR).unwrap_or_default().set_title(title).set_description(description))
    }
}
