mod gemini;
mod pokeapi;
mod rag;

use anyhow::Result;
use clap::{Parser, Subcommand};
use rag::RAGEngine;
use std::io::{self, Write};

#[derive(Parser)]
#[command(name = "pokidex")]
#[command(about = "A Pokemon RAG agent powered by Gemini AI and PokéAPI")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Start interactive chat mode
    Chat,
    /// Ask a single question
    Ask {
        /// Your question about Pokemon
        question: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok(); // Load .env file if present

    let cli = Cli::parse();

    let rag_engine = RAGEngine::new()?;

    let _ctrlc = tokio::spawn(async {
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to listen for ctrl-c");
        println!("\nReceived Ctrl-C, exiting...");
        std::process::exit(0);
    });

    match cli.command {
        Some(Commands::Ask { question }) => {
            println!("Processing your question...\n");
            match rag_engine.process_query(&question).await {
                Ok(response) => {
                    println!("Assistant: {}", response);
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Some(Commands::Chat) | None => {
            run_chat_mode(rag_engine).await?;
        }
    }

    Ok(())
}

async fn run_chat_mode(rag_engine: RAGEngine) -> Result<()> {
    println!("Welcome to Pokidex RAG Agent!");
    println!("Ask me anything about Pokemon. Type 'quit' or 'exit' to leave.\n");

    loop {
        print!("You: ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let query = input.trim();

        if query.is_empty() {
            continue;
        }

        if query == "quit" || query == "exit" {
            println!("Goodbye!");
            break;
        }

        if query == "help" {
            println!("Ask me questions about Pokemon! Examples:");
            println!("  - What are Pikachu's stats?");
            println!("  - What type is Charizard?");
            println!("  - What moves can Pikachu learn?");
            println!("\nType 'quit' or 'exit' to leave.\n");
            continue;
        }

        print!("Assistant: ");
        io::stdout().flush()?;

        match rag_engine.process_query(query).await {
            Ok(response) => {
                println!("{}", response);
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                println!("Please try again or type 'help' for examples.");
            }
        }
        println!();
    }

    Ok(())
}
