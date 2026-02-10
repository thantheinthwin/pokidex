use crate::gemini::GeminiClient;
use crate::pokeapi::PokeApiClient;
use anyhow::Result;
use serde_json::Value;

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

    /// Process a user query by asking Gemini which tool to call (if any),
    /// executing that tool via the PokéAPI client, and returning Gemini's final answer.
    pub async fn process_query(&self, query: &str) -> Result<String> {
        // Describe available tools to the model and request a JSON action or a final answer.
        let tools_description = r#"
Available tools you can call:
1) get_pokemon(name) -> Returns detailed Pokemon data for a given name or id.
2) get_pokemon_species(name) -> Returns species information (flavor text, capture rate, etc.).
3) get_pokemon_stats(name) -> Returns only base stats for a given pokemon.
4) get_pokemon_moves(name) -> Returns a compact move list for a given pokemon.

If you need to call a tool, respond with a JSON object exactly in this shape:
  {"type":"action","tool":"get_pokemon","name":"pikachu"}
or
  {"type":"action","tool":"get_pokemon_species","name":"pikachu"}
or
  {"type":"action","tool":"get_pokemon_stats","name":"pikachu"}
or
  {"type":"action","tool":"get_pokemon_moves","name":"pikachu"}

If you can answer the user directly without calling a tool, respond with:
  {"type":"final","answer":"<your answer>"}
"#;

        let initial_prompt = format!(
            "You are a Pokemon assistant agent. Decide whether to call one of the available tools to fetch Pokemon data, or answer directly. {}\nUser Question: {}",
            tools_description, query
        );

        let decision = self.gemini.generate_content(&initial_prompt).await?;
        println!("[RAG] Model decision (raw): {}", decision);

        // Try to parse model decision as JSON
        if let Ok(json) = serde_json::from_str::<Value>(&decision) {
            if let Some(t) = json.get("type").and_then(|v| v.as_str()) {
                match t {
                    "action" => {
                        let tool = json.get("tool").and_then(|v| v.as_str()).unwrap_or("");
                        let name = json.get("name").and_then(|v| v.as_str()).unwrap_or("");
                        println!(
                            "[RAG] Model requested action: tool='{}' name='{}'",
                            tool, name
                        );

                        // Execute the requested tool
                        let tool_output = match tool {
                            "get_pokemon" => {
                                match self.pokeapi.get_pokemon(name).await {
                                    Ok(pokemon) => {
                                        // Try to also fetch species for richer context
                                        if let Ok(species) =
                                            self.pokeapi.get_pokemon_species(name).await
                                        {
                                            self.pokeapi
                                                .format_pokemon_with_species(&pokemon, &species)
                                        } else {
                                            self.pokeapi.format_pokemon_data(&pokemon)
                                        }
                                    }
                                    Err(e) => format!("Error fetching pokemon: {}", e),
                                }
                            }
                            "get_pokemon_species" => {
                                match self.pokeapi.get_pokemon_species(name).await {
                                    Ok(species) => {
                                        // Try to also fetch the Pokemon to reuse existing formatter
                                        if let Ok(pokemon) = self.pokeapi.get_pokemon(name).await {
                                            self.pokeapi
                                                .format_pokemon_with_species(&pokemon, &species)
                                        } else {
                                            // Build a minimal, human-readable species summary without serde
                                            let mut out = String::new();
                                            out.push_str("Species Information:\n");
                                            out.push_str(&format!(
                                                "  Capture Rate: {}\n",
                                                species.capture_rate
                                            ));
                                            if let Some(base_hap) = species.base_hapiness {
                                                out.push_str(&format!(
                                                    "  Base Happiness: {}\n",
                                                    base_hap
                                                ));
                                            }
                                            out.push_str(&format!(
                                                "  Is Legendary: {}\n",
                                                species.is_legendary
                                            ));
                                            out.push_str(&format!(
                                                "  Is Mythical: {}\n",
                                                species.is_mythical
                                            ));
                                            if let Some(flavor_text) = species
                                                .flavor_text_entries
                                                .iter()
                                                .find(|e| e.language.name == "en")
                                            {
                                                out.push_str(&format!(
                                                    "  Description: {}\n",
                                                    flavor_text.flavor_text.replace('\n', " ")
                                                ));
                                            }
                                            out
                                        }
                                    }
                                    Err(e) => format!("Error fetching species: {}", e),
                                }
                            }
                            "get_pokemon_stats" => match self.pokeapi.get_pokemon(name).await {
                                Ok(pokemon) => self.pokeapi.format_pokemon_stats(&pokemon),
                                Err(e) => format!("Error fetching pokemon stats: {}", e),
                            },
                            "get_pokemon_moves" => match self.pokeapi.get_pokemon(name).await {
                                Ok(pokemon) => self.pokeapi.format_pokemon_moves(&pokemon, 30),
                                Err(e) => format!("Error fetching pokemon moves: {}", e),
                            },
                            _ => format!("Unknown tool requested: {}", tool),
                        };

                        // Ask Gemini to produce a final answer using the tool output
                        println!("[RAG] Tool output:\n{}", tool_output);
                        let followup = format!(
                            "Tool output:\n{}\n\nBased on the tool output above, provide a concise, user-facing answer to the original question: {}",
                            tool_output, query
                        );
                        println!("[RAG] Sending followup to model...");

                        let final_resp = self.gemini.generate_content(&followup).await?;
                        println!("[RAG] Model final response: {}", final_resp);
                        return Ok(final_resp);
                    }
                    "final" => {
                        if let Some(ans) = json.get("answer").and_then(|v| v.as_str()) {
                            println!("[RAG] Model provided final answer without tools.");
                            return Ok(ans.to_string());
                        }
                    }
                    _ => {}
                }
            }
        }

        // Fallback: try to extract a Pokemon name and use the RAG pattern as before.
        let pokemon_name = self.pokeapi.extract_pokemon_name(query).await;

        if let Some(name) = pokemon_name {
            let pokemon = self.pokeapi.get_pokemon(&name).await?;
            let species = self.pokeapi.get_pokemon_species(&name).await.ok();

            let context = if let Some(species) = species {
                self.pokeapi.format_pokemon_with_species(&pokemon, &species)
            } else {
                self.pokeapi.format_pokemon_data(&pokemon)
            };

            self.gemini.generate_with_context(&context, query).await
        } else {
            let context = "You are a Pokemon assistant. Answer questions about Pokemon using general knowledge. If asked about a specific Pokemon, you may need the Pokemon name to provide detailed information.";
            self.gemini.generate_with_context(context, query).await
        }
    }
}
