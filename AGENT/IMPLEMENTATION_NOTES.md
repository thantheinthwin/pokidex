# Pokemon RAG Agent - Implementation Notes

## Key Implementation Details

### Gemini API Integration

The Gemini API can be accessed via:
- REST API: `https://generativelanguage.googleapis.com/v1beta/models/gemini-pro:generateContent`
- Or use a Rust crate like `google-generativeai-rs` if available

**API Key**: Must be set in `GEMINI_API_KEY` environment variable

**Request Format**:
```json
{
  "contents": [{
    "parts": [{
      "text": "Context: {pokemon_data}\n\nUser Question: {user_query}"
    }]
  }]
}
```

### Rustemon Usage

Rustemon provides async functions for PokéAPI access:

```rust
use rustemon::client::RustemonClient;
use rustemon::pokemon::pokemon::Pokemon;

// Initialize client
let client = RustemonClient::default();

// Get Pokemon
let pikachu = Pokemon::get_by_name("pikachu", &client).await?;
```

**Key rustemon types:**
- `Pokemon` - Pokemon data
- `PokemonSpecies` - Species information
- `Type` - Type data
- `Move` - Move information
- `Ability` - Ability data

### RAG Prompt Template

```
You are a helpful Pokemon assistant. Use the following Pokemon data to answer the user's question.

Pokemon Data:
{formatted_pokemon_data}

User Question: {user_query}

Provide a clear, accurate answer based on the Pokemon data above.
```

### Error Handling Patterns

```rust
// Network errors
match result {
    Ok(data) => data,
    Err(rustemon::error::RustemonError::NetworkError(_)) => {
        // Retry logic
    }
    Err(e) => return Err(format!("Error: {}", e)),
}
```

### CLI Loop Structure

```rust
loop {
    print!("You: ");
    let input = read_line();
    
    if input.trim() == "quit" || input.trim() == "exit" {
        break;
    }
    
    match process_query(&input).await {
        Ok(response) => println!("Assistant: {}", response),
        Err(e) => eprintln!("Error: {}", e),
    }
}
```

## Testing Strategy

1. **Unit Tests**: Test individual components
   - Pokemon data formatting
   - Query parsing
   - Error handling

2. **Integration Tests**: Test API interactions
   - Mock API responses
   - Test RAG flow

3. **Manual Testing**: Test with real API
   - Various Pokemon queries
   - Edge cases (invalid names, etc.)

## Performance Considerations

- **Caching**: Cache Pokemon data in memory for session
- **Rate Limiting**: Respect API rate limits
- **Async**: Use async/await for non-blocking operations

## Security Considerations

- Never commit API keys
- Use `.env` file (gitignored) for local development
- Validate user input to prevent injection

## Deployment Notes

- Binary should be standalone
- Environment variable must be set at runtime
- Consider adding `--api-key` CLI flag as alternative
