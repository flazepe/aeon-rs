use crate::statics::COLLECTIONS;
use anyhow::Result;
use mongodb::{
    bson::{doc, to_document},
    options::UpdateOptions,
};
use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Deserialize)]
struct RawOAuthToken {
    access_token: String,
    token_type: String,
    expires_in: u64,
}

#[derive(Deserialize, Serialize)]
pub struct OAuthToken {
    pub _id: String,
    pub token: String,
    pub expires_at: u64,
}

pub struct OAuth {
    name: String,
    request: RequestBuilder,
    timestamp: u64,
}

impl OAuth {
    pub fn new<T: ToString>(name: T, request: RequestBuilder) -> Self {
        Self { name: name.to_string(), request, timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() }
    }

    async fn generate_token(self) -> Result<OAuthToken> {
        let token = self.request.send().await?.json::<RawOAuthToken>().await?;

        let token = OAuthToken {
            _id: self.name.clone(),
            token: format!("{} {}", token.token_type, token.access_token),
            expires_at: self.timestamp + token.expires_in,
        };

        COLLECTIONS
            .oauth
            .update_one(
                doc! {
                    "_id": self.name,
                },
                doc! {
                    "$set": to_document(&token)?,
                },
                UpdateOptions::builder().upsert(true).build(),
            )
            .await?;

        Ok(token)
    }

    pub async fn get_token(self) -> Result<String> {
        Ok(match COLLECTIONS.oauth.find_one(doc! { "_id": &self.name }, None).await? {
            Some(token) => {
                match token.expires_at > self.timestamp {
                    true => token,
                    false => self.generate_token().await?,
                }
                .token
            },
            None => self.generate_token().await?.token,
        })
    }
}
