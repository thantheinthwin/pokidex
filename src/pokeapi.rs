use anyhow::{Context, Result};
use rustemon::client::RustemonClient;
use rustemon::model::pokemon::Pokemon;
use rustemon::model::pokemon::PokemonSpecies;
use std::fmt::Write;

pub struct PokeApiClient {
    client: RustemonClient,
}

impl PokeApiClient {
    pub fn new() -> Self {
        Self {
            client: RustemonClient::default(),
        }
    }

    pub async fn get_pokemon(&self, name_or_id: &str) -> Result<Pokemon> {
        // Try by name first
        match rustemon::pokemon::pokemon::get_by_name(name_or_id, &self.client).await {
            Ok(pokemon) => Ok(pokemon),
            Err(_) => {
                // Try parsing as ID
                if let Ok(id) = name_or_id.parse::<i64>() {
                    rustemon::pokemon::pokemon::get_by_id(id, &self.client)
                        .await
                        .context(format!("Failed to find Pokemon with ID: {}", id))
                } else {
                    Err(anyhow::anyhow!("Failed to find Pokemon: {}", name_or_id))
                }
            }
        }
    }

    pub async fn get_pokemon_species(&self, name_or_id: &str) -> Result<PokemonSpecies> {
        // Try by name first
        match rustemon::pokemon::pokemon_species::get_by_name(name_or_id, &self.client).await {
            Ok(species) => Ok(species),
            Err(_) => {
                // Try parsing as ID
                if let Ok(id) = name_or_id.parse::<i64>() {
                    rustemon::pokemon::pokemon_species::get_by_id(id, &self.client)
                        .await
                        .context(format!("Failed to find Pokemon species with ID: {}", id))
                } else {
                    Err(anyhow::anyhow!("Failed to find Pokemon species: {}", name_or_id))
                }
            }
        }
    }

    pub fn format_pokemon_data(&self, pokemon: &Pokemon) -> String {
        let mut output = String::new();
        
        writeln!(output, "Name: {}", pokemon.name).ok();
        writeln!(output, "ID: {}", pokemon.id).ok();
        
        // Types
        write!(output, "Types: ").ok();
        let types: Vec<String> = pokemon
            .types
            .iter()
            .map(|t| t.type_.name.clone())
            .collect();
        writeln!(output, "{}", types.join(", ")).ok();
        
        // Stats
        writeln!(output, "Stats:").ok();
        for stat in &pokemon.stats {
            let stat_name = &stat.stat.name;
            let base_stat = stat.base_stat;
            writeln!(output, "  - {}: {}", stat_name, base_stat).ok();
        }
        
        // Abilities
        write!(output, "Abilities: ").ok();
        let abilities: Vec<String> = pokemon
            .abilities
            .iter()
            .map(|a| {
                let mut name = a.ability.name.clone();
                if a.is_hidden {
                    name.push_str(" (hidden)");
                }
                name
            })
            .collect();
        writeln!(output, "{}", abilities.join(", ")).ok();
        
        // Height and Weight
        writeln!(output, "Height: {} dm", pokemon.height).ok();
        writeln!(output, "Weight: {} hg", pokemon.weight).ok();
        
        // Base Experience
        if let Some(base_exp) = pokemon.base_experience {
            writeln!(output, "Base Experience: {}", base_exp).ok();
        }
        
        output
    }

    pub fn format_pokemon_with_species(&self, pokemon: &Pokemon, species: &PokemonSpecies) -> String {
        let mut output = self.format_pokemon_data(pokemon);
        
        // Add species information
        writeln!(output, "\nSpecies Information:").ok();
        writeln!(output, "  Capture Rate: {}", species.capture_rate).ok();
        if let Some(base_happiness) = species.base_hapiness {
            writeln!(output, "  Base Happiness: {}", base_happiness).ok();
        }
        writeln!(output, "  Is Legendary: {}", species.is_legendary).ok();
        writeln!(output, "  Is Mythical: {}", species.is_mythical).ok();
        
        // Flavor text (first English one)
        if let Some(flavor_text) = species
            .flavor_text_entries
            .iter()
            .find(|e| e.language.name == "en")
        {
            writeln!(output, "  Description: {}", 
                flavor_text.flavor_text.replace('\n', " ")).ok();
        }
        
        output
    }

    pub async fn extract_pokemon_name(&self, query: &str) -> Option<String> {
        // Simple extraction - look for capitalized words that might be Pokemon names
        // This is a basic implementation; could be improved with NLP
        let words: Vec<&str> = query.split_whitespace().collect();
        
        for word in words {
            // Check if word starts with capital letter and is reasonable length
            if word.chars().next().map(|c| c.is_uppercase()).unwrap_or(false)
                && word.len() > 2
                && word.len() < 20
            {
                // Try to get the Pokemon to verify
                if let Ok(_) = self.get_pokemon(&word.to_lowercase()).await {
                    return Some(word.to_lowercase());
                }
            }
        }
        
        None
    }
}
