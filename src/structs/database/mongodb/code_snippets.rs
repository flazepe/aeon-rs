use crate::structs::api::piston::PistonState;
use anyhow::{Result, bail};
use futures::TryStreamExt;
use mongodb::{Collection, bson::doc};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Deserialize, Serialize, Clone, Default, Debug)]
pub struct CodeSnippet {
    pub name: String,
    pub user_id: String,
    pub piston_state: PistonState,
}

#[derive(Debug)]
pub struct CodeSnippets {
    collection: Collection<CodeSnippet>,
}

impl CodeSnippets {
    pub fn new(collection: Collection<CodeSnippet>) -> Self {
        Self { collection }
    }

    pub async fn get<T: Display, U: Display>(&self, name: T, user_id: U) -> Option<CodeSnippet> {
        let name = name.to_string().to_lowercase();
        self.collection.find_one(doc! { "name": name, "user_id": user_id.to_string() }).await.unwrap_or_default()
    }

    pub async fn get_all<T: Display>(&self, user_id: T) -> Result<Vec<CodeSnippet>> {
        let user_id = user_id.to_string();
        let code_snippets = self.collection.find(doc! { "user_id": user_id.to_string() }).await?.try_collect::<Vec<CodeSnippet>>().await?;
        Ok(code_snippets)
    }

    pub async fn upsert(&self, code_snippet: CodeSnippet) -> Result<()> {
        if self.get(&code_snippet.name, &code_snippet.user_id).await.is_some() {
            self.collection.replace_one(doc! { "name": &code_snippet.name, "user_id": &code_snippet.user_id }, code_snippet).await?;
            return Ok(());
        }

        if self.collection.count_documents(doc! {}).await? >= 50 {
            bail!("You cannot have over 50 code snippets.");
        }

        self.collection.insert_one(code_snippet).await?;
        Ok(())
    }

    pub async fn delete<T: Display, U: Display>(&self, name: T, user_id: U) -> Result<()> {
        let name = name.to_string().to_lowercase();
        let user_id = user_id.to_string();

        if self.get(&name, &user_id).await.is_none() {
            bail!("Code snippet not found.");
        }

        self.collection.delete_one(doc! { "name": name, "user_id": user_id }).await?;
        Ok(())
    }
}
