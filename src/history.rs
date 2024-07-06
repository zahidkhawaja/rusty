use chrono::Utc;
use serde_derive::{Deserialize, Serialize};
use std::error::Error;

const HISTORY_FILE: &str = "history.json";

/// History of queries
#[derive(Debug, Serialize, Deserialize)]
pub struct History {
    pub query: String,
    pub response: String,
    pub timestamp: String,
}

/// Save the query and response to history
pub fn save(query: String, response: String) -> Result<(), Box<dyn Error>> {
    let history_path = get_history_path();

    let history = match std::fs::read_to_string(&history_path) {
        Ok(history) => history,
        Err(_) => String::from("[]"),
    };

    let mut history: Vec<History> = serde_json::from_str(&history)?;

    history.push(History {
        query: query.to_string(),
        response: response.to_string(),
        timestamp: Utc::now().to_rfc3339(),
    });

    let history = serde_json::to_string(&history)?;

    // write to file
    // expecting the parent directory to exist
    std::fs::write(&history_path, history)?;

    println!("History saved at: {}", history_path.display());

    Ok(())
}

/// List the history of queries
pub fn list() -> Result<Vec<History>, Box<dyn Error>> {
    let history_path = get_history_path();

    let history = match std::fs::read_to_string(&history_path) {
        Ok(history) => history,
        Err(_) => String::from("[]"),
    };

    let history: Vec<History> = serde_json::from_str(&history)?;

    Ok(history)
}

/// Get the history path
fn get_history_path() -> std::path::PathBuf {
    let history_path = dirs::config_dir().expect("Could not find the configuration directory");
    history_path
        .join(format!(".{}", env!("CARGO_PKG_NAME")))
        .join(HISTORY_FILE)
}
