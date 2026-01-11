# Pokemon RAG Agent - Architecture

## System Architecture

```
┌─────────────────┐
│   CLI Layer     │  (User Interface)
└────────┬─────────┘
         │
         ▼
┌─────────────────┐
│  RAG Orchestrator│  (Coordinates retrieval + generation)
└────────┬─────────┘
         │
    ┌────┴────┐
    │        │
    ▼        ▼
┌────────┐ ┌──────────┐
│PokéAPI │ │  Gemini  │
│Client  │ │   API    │
│(rustemon)│ │  Client  │
└────────┘ └──────────┘
```

## Component Details

### 1. CLI Layer (`src/cli.rs`)
- Handles user input/output
- Manages interactive chat loop
- Parses commands and arguments
- Provides user-friendly error messages

### 2. RAG Orchestrator (`src/rag.rs`)
- Receives user query
- Determines what Pokemon data to retrieve
- Calls PokéAPI client to fetch data
- Formats context for Gemini
- Calls Gemini API with context + query
- Returns generated response

### 3. PokéAPI Client (`src/pokeapi.rs`)
- Wraps rustemon functionality
- Provides high-level functions:
  - `get_pokemon(name_or_id)` -> Pokemon data
  - `get_pokemon_species(name_or_id)` -> Species data
  - `get_type(name_or_id)` -> Type information
  - `get_move(name_or_id)` -> Move information
  - `search_pokemon(query)` -> Search functionality

### 4. Gemini Client (`src/gemini.rs`)
- Handles API authentication
- Manages API calls
- Formats prompts with context
- Parses responses

## Data Flow

1. **User Query**: "What are Pikachu's stats?"
2. **Query Analysis**: RAG orchestrator identifies "Pikachu" and "stats"
3. **Data Retrieval**: PokéAPI client fetches Pikachu data
4. **Context Building**: Format Pokemon data as context
5. **LLM Call**: Send context + query to Gemini
6. **Response**: Return formatted answer to user

## Context Format

When sending to Gemini, we'll format Pokemon data as:
```
Context:
Pokemon: Pikachu
ID: 25
Types: Electric
Stats:
  - HP: 35
  - Attack: 55
  - Defense: 40
  ...
Abilities: Static, Lightning Rod
...

User Question: {user_query}
```

## Error Handling Strategy
- Network errors: Retry with exponential backoff
- API errors: Clear error messages to user
- Invalid Pokemon names: Suggest similar names
- Rate limiting: Queue requests
