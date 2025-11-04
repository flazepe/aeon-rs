use crate::statics::{CONFIG, REQWEST};
use anyhow::{Context, Result};
use mongodb::{
    Collection,
    bson::{doc, to_document},
};
use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};
use slashook::chrono::Utc;
use std::fmt::Display;

#[derive(Deserialize, Debug)]
struct RawOAuthToken {
    access_token: String,
    token_type: String,
    expires_in: i64,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct OAuthToken {
    pub _id: String,
    pub token: String,
    pub expires_at: i64,
}

#[derive(Debug)]
pub struct RefreshableOAuthToken {
    collection: Collection<OAuthToken>,
    name: String,
    request: RequestBuilder,
    timestamp: i64,
}

impl RefreshableOAuthToken {
    pub fn new<T: Display>(collection: Collection<OAuthToken>, name: T, request: RequestBuilder) -> Self {
        Self { collection, name: name.to_string(), request, timestamp: Utc::now().timestamp() }
    }

    async fn generate_token(&self) -> Result<OAuthToken> {
        let token = self.request.try_clone().context("Could not clone RequestBuilder.")?.send().await?.json::<RawOAuthToken>().await?;

        let token = OAuthToken {
            _id: self.name.clone(),
            token: format!("{} {}", token.token_type, token.access_token),
            expires_at: self.timestamp + token.expires_in,
        };

        self.collection.update_one(doc! { "_id": &self.name }, doc! { "$set": to_document(&token)? }).upsert(true).await?;

        Ok(token)
    }

    pub async fn get_token(&self) -> Result<String> {
        let Some(token) = self.collection.find_one(doc! { "_id": &self.name }).await? else {
            return Ok(self.generate_token().await?.token);
        };

        if token.expires_at > self.timestamp { Ok(token.token) } else { Ok(self.generate_token().await?.token) }
    }
}

#[derive(Debug)]
pub struct OAuth {
    pub osu: RefreshableOAuthToken,
    pub spotify: RefreshableOAuthToken,
}

impl OAuth {
    pub fn new(collection: Collection<OAuthToken>) -> Self {
        Self {
            osu: RefreshableOAuthToken::new(
                collection.clone(),
                "osu",
                REQWEST.post("https://osu.ppy.sh/oauth/token").form(&[
                    ("client_id", CONFIG.api.osu.client_id.as_str()),
                    ("client_secret", CONFIG.api.osu.client_secret.as_str()),
                    ("grant_type", "client_credentials"),
                    ("scope", "public"),
                ]),
            ),
            spotify: RefreshableOAuthToken::new(
                collection.clone(),
                "spotify",
                REQWEST
                    .post("https://accounts.spotify.com/api/token")
                    .header("authorization", format!("Basic {}", CONFIG.api.spotify_token))
                    .form(&[("grant_type", "client_credentials")]),
            ),
        }
    }
}
