mod file_picker;
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
    /// Identify a Pokemon from an image and return its specs
    IdentifyImage {
        /// Path to the image file
        image_path: String,
    },
    /// Open system file picker (Finder/File Explorer/dialog) and identify a Pokemon image
    SelectImage,
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
        Some(Commands::IdentifyImage { image_path }) => {
            println!("Analyzing image...\n");
            match rag_engine.process_image_query(&image_path).await {
                Ok(response) => println!("Assistant: {}", response),
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Some(Commands::SelectImage) => {
            println!("Opening file picker...\n");
            match file_picker::pick_image_file() {
                Ok(selected_path) => {
                    println!("Selected: {}", selected_path.display());
                    println!("Analyzing image...\n");
                    match rag_engine
                        .process_image_query(selected_path.to_string_lossy().as_ref())
                        .await
                    {
                        Ok(response) => println!("Assistant: {}", response),
                        Err(e) => {
                            eprintln!("Error: {}", e);
                            std::process::exit(1);
                        }
                    }
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
            println!("  - You can also run: pokidex identify-image ./pokemon.png");
            println!("  - Or open file picker: pokidex select-image");
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
