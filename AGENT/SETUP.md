# Setup Instructions

## Prerequisites

- Rust installed (1.70+)
- Gemini API key from Google

## Setup Steps

1. **Get Gemini API Key**

   - Visit: https://makersuite.google.com/app/apikey
   - Create a new API key
   - Copy the key

2. **Configure Environment**

   ```bash
   # Copy the example file
   cp .env.example .env

   # Edit .env and add your API key
   # GEMINI_API_KEY=your_actual_api_key_here
   ```

   Or set it directly:

   ```bash
   export GEMINI_API_KEY=your_actual_api_key_here
   ```

3. **Build the Project**

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

### Interactive Mode

```bash
$ cargo run -- chat
Welcome to Pokidex RAG Agent!
Ask me anything about Pokemon. Type 'quit' or 'exit' to leave.

You: What are Pikachu's stats?
Assistant: [Response from AI]

You: quit
Goodbye!
```

### Single Query Mode

```bash
$ cargo run -- ask "What type is Charizard?"
Assistant: [Response from AI]
```

## Troubleshooting

### "GEMINI_API_KEY environment variable not set"

- Make sure you've set the environment variable
- Check that `.env` file exists and contains `GEMINI_API_KEY=...`
- Or export it in your shell: `export GEMINI_API_KEY=...`

### "Failed to find Pokemon"

- Check the Pokemon name spelling
- Pokemon names are case-insensitive but must match PokéAPI format
- Try using the Pokemon's ID number instead

### Network Errors

- Check your internet connection
- Verify API key is valid
- Check if PokéAPI is accessible

## Project Structure

```
pokidex/
├── src/
│   ├── main.rs      # CLI entry point
│   ├── gemini.rs    # Gemini API client
│   ├── pokeapi.rs   # PokéAPI client (using rustemon)
│   └── rag.rs       # RAG orchestrator
├── AGENT/
│   ├── PROJECT_PLAN.md
│   ├── ARCHITECTURE.md
│   ├── REQUIREMENTS.md
│   ├── IMPLEMENTATION_NOTES.md
│   └── SETUP.md (this file)
├── Cargo.toml
└── .env (create this)
```
