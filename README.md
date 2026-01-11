# Pokidex RAG Agent

A CLI-based RAG (Retrieval-Augmented Generation) system that combines Google's Gemini API with PokéAPI (via rustemon) to provide an intelligent chatbot for Pokemon information.

## Features

- 🤖 **AI-Powered**: Uses Google Gemini API for natural language understanding
- 📊 **Pokemon Data**: Retrieves real-time Pokemon data from PokéAPI
- 💬 **Interactive CLI**: Chat interface for asking questions about Pokemon
- 🔍 **RAG System**: Combines retrieved Pokemon data with AI for accurate responses

## Quick Start

### Prerequisites

- Rust 1.70 or later
- Gemini API key ([Get one here](https://makersuite.google.com/app/apikey))

### Installation

1. **Clone or navigate to the project**

   ```bash
   cd pokidex
   ```

2. **Set up environment variables**

   ```bash
   # Create .env file
   echo "GEMINI_API_KEY=your_api_key_here" > .env

   # Or export directly
   export GEMINI_API_KEY=your_api_key_here
   ```

3. **Build the project**

   ```bash
   cargo build --release
   ```

4. **Run the CLI**

   ```bash
   # Interactive chat mode
   cargo run -- chat

   # Or ask a single question
   cargo run -- ask "What are Pikachu's stats?"
   ```

## Usage Examples

### Interactive Chat Mode

```bash
$ cargo run -- chat
Welcome to Pokidex RAG Agent!
Ask me anything about Pokemon. Type 'quit' or 'exit' to leave.

You: What are Pikachu's stats?
Assistant: Pikachu has the following base stats:
- HP: 35
- Attack: 55
- Defense: 40
- Special Attack: 50
- Special Defense: 50
- Speed: 90

You: What type is Charizard?
Assistant: Charizard is a Fire/Flying type Pokemon.

You: quit
Goodbye!
```

### Single Query Mode

```bash
$ cargo run -- ask "What moves can Pikachu learn?"
Assistant: [AI-generated response with Pokemon data]
```

## Project Structure

```
pokidex/
├── src/
│   ├── main.rs      # CLI entry point and chat loop
│   ├── gemini.rs    # Gemini API client
│   ├── pokeapi.rs   # PokéAPI client (using rustemon)
│   └── rag.rs       # RAG orchestrator
├── AGENT/
│   ├── PROJECT_PLAN.md
│   ├── ARCHITECTURE.md
│   ├── REQUIREMENTS.md
│   ├── IMPLEMENTATION_NOTES.md
│   └── SETUP.md
├── Cargo.toml
└── README.md
```

## How It Works

1. **User Query**: You ask a natural language question about Pokemon
2. **Pokemon Extraction**: System identifies Pokemon names in your query
3. **Data Retrieval**: Fetches Pokemon data from PokéAPI using rustemon
4. **Context Building**: Formats Pokemon data as context
5. **AI Generation**: Sends context + query to Gemini API
6. **Response**: Returns accurate, contextual answer

## Example Questions

- "What are Pikachu's stats?"
- "What type is Charizard?"
- "What moves can Pikachu learn?"
- "Is Mewtwo legendary?"
- "What are the abilities of Eevee?"
- "Compare Pikachu and Raichu"

## Dependencies

- `rustemon`: PokéAPI client for Rust
- `tokio`: Async runtime
- `clap`: CLI argument parsing
- `reqwest`: HTTP client for Gemini API
- `serde`, `serde_json`: JSON serialization
- `anyhow`: Error handling
- `dotenv`: Environment variable management

## Troubleshooting

### "GEMINI_API_KEY environment variable not set"

- Make sure you've set the environment variable
- Check that `.env` file exists and contains `GEMINI_API_KEY=...`
- Or export it: `export GEMINI_API_KEY=...`

### "Failed to find Pokemon"

- Check the Pokemon name spelling
- Pokemon names are case-insensitive
- Try using the Pokemon's ID number instead

### Network Errors

- Check your internet connection
- Verify API key is valid
- Check if PokéAPI is accessible

## Development

### Building

```bash
cargo build
```

### Running Tests

```bash
cargo test
```

### Running in Debug Mode

```bash
cargo run -- chat
```

## License

This project is open source and available for personal and educational use.

## Acknowledgments

- [PokéAPI](https://pokeapi.co/) for Pokemon data
- [rustemon](https://crates.io/crates/rustemon) for the PokéAPI Rust client
- Google Gemini API for AI capabilities
