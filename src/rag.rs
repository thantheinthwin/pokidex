use anyhow::Result;
use crate::gemini::GeminiClient;
use crate::pokeapi::PokeApiClient;

pub struct RAGEngine {
    gemini: GeminiClient,
    pokeapi: PokeApiClient,
}

impl RAGEngine {
    pub fn new() -> Result<Self> {
        Ok(Self {
            gemini: GeminiClient::new()?,
            pokeapi: PokeApiClient::new(),
        })
    }

    pub async fn process_query(&self, query: &str) -> Result<String> {
        // Try to extract Pokemon name from query
        let pokemon_name = self.pokeapi.extract_pokemon_name(query).await;
        
        if let Some(name) = pokemon_name {
            // Fetch Pokemon data
            let pokemon = self.pokeapi.get_pokemon(&name).await?;
            let species = self.pokeapi.get_pokemon_species(&name).await.ok();
            
            // Format context
            let context = if let Some(species) = species {
                self.pokeapi.format_pokemon_with_species(&pokemon, &species)
            } else {
                self.pokeapi.format_pokemon_data(&pokemon)
            };
            
            // Generate response with context
            self.gemini.generate_with_context(&context, query).await
        } else {
            // No specific Pokemon found, use general context
            let context = "You are a Pokemon assistant. Answer questions about Pokemon using general knowledge. If asked about a specific Pokemon, you may need the Pokemon name to provide detailed information.";
            self.gemini.generate_with_context(context, query).await
        }
    }
}
