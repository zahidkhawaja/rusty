use clap::Parser;
use cli::{Cli, Commands};
use dirs;
use openai::{openai_query, Config, OpenAIQuery};
use std::error::Error;

mod cli;
mod history;
mod openai;

const CONFIG_FILE: &str = "config.json";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let args = Cli::parse();

    match args.command {
        Some(commands) => match commands {
            Commands::Query { query } => {
                let config_path = get_config_path();

                // check if config file exists
                if !config_path.exists() {
                    println!(
                        "Config file not found. Please run `{} setup` to create one.",
                        env!("CARGO_PKG_NAME")
                    );
                    return Ok(());
                }

                // read config file
                let config = std::fs::read_to_string(config_path)?;

                match serde_json::from_str::<Config>(&config) {
                    Ok(config) => {
                        println!("Config: {:#?}", config);

                        let response = openai_query(OpenAIQuery {
                            query: query.to_string(),
                            api_key: config.api_key,
                            model: config.model,
                        })
                        .await?;

                        println!("{}", response);

                        // save to history
                        history::save(query, response).unwrap();
                    }
                    Err(_) => {
                        println!("Error: could not parse config file. Run `{} setup` to create another config file.", env!("CARGO_PKG_NAME"));
                    }
                }
            }
            Commands::Setup => {
                // user input for api key and model name
                println!("Please enter your OpenAI API key:");
                let mut api_key = String::new();
                std::io::stdin().read_line(&mut api_key).unwrap();

                println!("Please enter the model name:");
                let mut model = String::new();
                std::io::stdin().read_line(&mut model).unwrap();

                // remove leading/trailing whitespaces
                let api_key = api_key.trim().to_string();
                let model = model.trim().to_string();

                // check if the value is empty
                if api_key.is_empty() || model.is_empty() {
                    println!("Error: API key or model name cannot be empty");
                    return Ok(());
                }

                // create config file
                let config = Config { api_key, model };

                let config_path = get_config_path();

                println!("Creating config file at: {}", config_path.display());

                // create the directory if it doesn't exist
                if !config_path.parent().unwrap().exists() {
                    std::fs::create_dir_all(config_path.parent().unwrap())?;
                }

                // create config file
                std::fs::write(&config_path, serde_json::to_string_pretty(&config)?)?;

                println!("Config file created at: {}", config_path.display());
            }
            Commands::History => {
                let history = history::list().unwrap();

                if history.is_empty() {
                    println!("No history found");
                    return Ok(());
                }

                for (index, h) in history.iter().enumerate() {
                    println!("Index: {}", index);
                    println!("Query: {}", h.query);
                    println!("Response: {}", h.response);
                    println!(
                        "Timestamp: {}",
                        chrono::DateTime::parse_from_rfc3339(&h.timestamp)
                            .unwrap()
                            .format("%b %d, %Y %I:%M %p")
                    );
                    println!();
                }
            }
        },
        None => {
            println!("No command provided");
        }
    }

    Ok(())
}

fn get_config_path() -> std::path::PathBuf {
    let config_dir = dirs::config_dir().expect("Could not find the configuration directory");
    config_dir
        .join(format!(".{}", env!("CARGO_PKG_NAME")))
        .join(CONFIG_FILE)
}
