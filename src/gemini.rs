use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize)]
struct GeminiRequest {
    contents: Vec<Content>,
}

#[derive(Debug, Serialize)]
struct Content {
    parts: Vec<Part>,
}

#[derive(Debug, Serialize)]
struct Part {
    text: String,
}

#[derive(Debug, Deserialize)]
struct GeminiResponse {
    candidates: Vec<Candidate>,
}

#[derive(Debug, Deserialize)]
struct Candidate {
    content: ResponseContent,
}

#[derive(Debug, Deserialize)]
struct ResponseContent {
    parts: Vec<ResponsePart>,
}

#[derive(Debug, Deserialize)]
struct ResponsePart {
    text: String,
}

pub struct GeminiClient {
    api_key: String,
    base_url: String,
}

impl GeminiClient {
    pub fn new() -> Result<Self> {
        let api_key = env::var("GEMINI_API_KEY")
            .context("GEMINI_API_KEY environment variable not set")?;
        
        Ok(Self {
            api_key,
            base_url: "https://generativelanguage.googleapis.com/v1beta/models/gemini-pro:generateContent".to_string(),
        })
    }

    pub async fn generate_content(&self, prompt: &str) -> Result<String> {
        let request = GeminiRequest {
            contents: vec![Content {
                parts: vec![Part {
                    text: prompt.to_string(),
                }],
            }],
        };

        let client = reqwest::Client::new();
        let url = format!("{}?key={}", self.base_url, self.api_key);
        
        let response = client
            .post(&url)
            .json(&request)
            .send()
            .await
            .context("Failed to send request to Gemini API")?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            anyhow::bail!("Gemini API error ({}): {}", status, error_text);
        }

        let gemini_response: GeminiResponse = response
            .json()
            .await
            .context("Failed to parse Gemini API response")?;

        let text = gemini_response
            .candidates
            .first()
            .and_then(|c| c.content.parts.first())
            .map(|p| p.text.clone())
            .context("No response text in Gemini API response")?;

        Ok(text)
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
