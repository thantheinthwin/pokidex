use anyhow::{Context, Result};
use base64::{engine::general_purpose, Engine as _};
use gemini_rust::prelude::*;
use std::env;
use std::fs;

pub struct GeminiClient {
    client: Gemini,
}

impl GeminiClient {
    pub fn new() -> Result<Self> {
        let api_key =
            env::var("GEMINI_API_KEY").context("GEMINI_API_KEY environment variable not set")?;

        let client = Gemini::new(&api_key).context("Failed to create Gemini client")?;

        Ok(Self { client })
    }

    pub async fn generate_content(&self, prompt: &str) -> Result<String> {
        let response = self
            .client
            .generate_content()
            .with_user_message(prompt)
            .execute()
            .await
            .context("Failed to send request to Gemini API")?;

        Ok(response.text())
    }

    pub async fn identify_pokemon_from_image(&self, image_path: &str) -> Result<String> {
        let image_bytes =
            fs::read(image_path).context(format!("Failed to read image file: {}", image_path))?;

        let mime_type = Self::mime_type_for_path(image_path);
        let image_b64 = general_purpose::STANDARD.encode(image_bytes);

        let prompt = "You are validating whether an image contains a Pokémon. Return STRICT JSON only with one of these shapes: {\"type\":\"pokemon\",\"name\":\"<pokemon name>\"} or {\"type\":\"not_pokemon\",\"reason\":\"<short reason>\"}. If unsure, return not_pokemon.";

        let response = self
            .client
            .generate_content()
            .with_user_message(prompt)
            .with_inline_data(image_b64, &mime_type)
            .execute()
            .await
            .context("Failed to send image request to Gemini API")?;

        Ok(response.text())
    }

    fn mime_type_for_path(path: &str) -> String {
        let lower = path.to_lowercase();
        if lower.ends_with(".png") {
            "image/png".to_string()
        } else if lower.ends_with(".jpg") || lower.ends_with(".jpeg") {
            "image/jpeg".to_string()
        } else if lower.ends_with(".webp") {
            "image/webp".to_string()
        } else if lower.ends_with(".gif") {
            "image/gif".to_string()
        } else {
            "application/octet-stream".to_string()
        }
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
