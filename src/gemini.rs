use anyhow::{Context, Result};
use gemini_rust::prelude::*;
use std::env;

pub struct GeminiClient {
    client: Gemini,
}

impl GeminiClient {
    pub fn new() -> Result<Self> {
        let api_key = env::var("GEMINI_API_KEY")
            .context("GEMINI_API_KEY environment variable not set")?;
        
        let client = Gemini::new(&api_key)
            .context("Failed to create Gemini client")?;
        
        Ok(Self { client })
    }

    pub async fn generate_content(&self, prompt: &str) -> Result<String> {
        let response = self.client
            .generate_content()
            .with_user_message(prompt)
            .execute()
            .await
            .context("Failed to send request to Gemini API")?;

        Ok(response.text())
    }

    pub async fn generate_with_context(&self, context: &str, user_query: &str) -> Result<String> {
        let prompt = format!(
            "You are a helpful Pokemon assistant. Use the following Pokemon data to answer the user's question accurately and concisely.\n\n\
            Pokemon Data:\n{}\n\n\
            User Question: {}\n\n\
            Provide a clear, accurate answer based on the Pokemon data above. If the data doesn't contain the answer, say so.",
            context, user_query
        );

        self.generate_content(&prompt).await
    }
}
