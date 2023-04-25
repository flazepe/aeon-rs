use crate::{macros::if_else, statics::MONGODB};
use anyhow::Result;
use mongodb::{
    bson::{doc, to_document},
    options::UpdateOptions,
    Collection,
};
use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Deserialize)]
pub struct RawOauthToken {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u64,
}

#[derive(Deserialize, Serialize)]
pub struct OauthToken {
    pub _id: String,
    pub token: String,
    pub expires_at: u64,
}

pub struct OAuth {
    oauth: Collection<OauthToken>,
    name: String,
    request: RequestBuilder,
    timestamp: u64,
}

impl OAuth {
    pub fn new<T: ToString>(name: T, request: RequestBuilder) -> Self {
        Self {
            oauth: MONGODB.get().unwrap().collection::<OauthToken>("oauth"),
            name: name.to_string(),
            request,
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
        }
    }

    async fn generate_token(self) -> Result<OauthToken> {
        let token = self.request.send().await?.json::<RawOauthToken>().await?;

        let token = OauthToken {
            _id: self.name.clone(),
            token: format!("{} {}", token.token_type, token.access_token),
            expires_at: self.timestamp + token.expires_in,
        };

        self.oauth
            .update_one(doc! { "_id": &self.name }, doc! { "$set": to_document(&token)? }, UpdateOptions::builder().upsert(true).build())
            .await?;

        Ok(token)
    }

    pub async fn get_token(self) -> Result<String> {
        Ok(if let Some(token) = self.oauth.find_one(doc! { "_id": &self.name }, None).await? {
            if_else!(token.expires_at > self.timestamp, token, self.generate_token().await?).token
        } else {
            self.generate_token().await?.token
        })
    }
}
