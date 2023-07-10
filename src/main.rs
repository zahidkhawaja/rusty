use hyper::{body::Buf, header, Body, Client, Request};
use hyper_tls::HttpsConnector;
use serde_derive::{Deserialize, Serialize};
use std::env;
use std::error::Error;
use std::io::{stdin, stdout, Write};

#[derive(Deserialize, Debug)]
struct OpenAIChoices {
    text: String,
}

#[derive(Deserialize, Debug)]
struct OpenAIResponse {
    choices: Vec<OpenAIChoices>,
}

#[derive(Serialize, Debug)]
struct OpenAIRequest {
    model: String,
    prompt: String,
    max_tokens: u32,
    stop: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    // Check for environment variable OPENAI_API_KEY
    let api_key = match env::var("OPENAI_API_KEY") {
        Ok(key) => key,
        Err(_) => {
            println!("Error: missing environment variable OPENAI_API_KEY");
            std::process::exit(1);
        }
    };

    let https = HttpsConnector::new();
    let client = Client::builder().build(https);

    const URI: &str = "https://api.openai.com/v1/completions";

    let model = String::from("text-davinci-003");
    let stop = String::from("Text");

    let default_prompt =
        "Given text, return 1 bash command. Text:list contents of a directory. Command:ls";

    let user_input = env::args().skip(1).collect::<Vec<String>>().join(" ");

    // If no arguments were provided, ask for user input
    let user_input = if user_input.is_empty() {
        print!("Enter prompt: ");
        let _ = stdout().flush();
        let mut input = String::new();
        stdin()
            .read_line(&mut input)
            .expect("Failed to read prompt.");
        input
    } else {
        user_input
    };

    let auth_header_val = format!("Bearer {}", api_key);

    let openai_request = OpenAIRequest {
        model,
        prompt: format!("{} Text:{}. Command:", default_prompt, user_input),
        max_tokens: 64,
        stop,
    };

    let body = Body::from(serde_json::to_vec(&openai_request)?);

    let req = Request::post(URI)
        .header(header::CONTENT_TYPE, "application/json")
        .header("Authorization", &auth_header_val)
        .body(body)
        .unwrap();

    let res = client.request(req).await?;

    let body = hyper::body::aggregate(res).await?;

    let json: OpenAIResponse = match serde_json::from_reader(body.reader()) {
        Ok(response) => response,
        Err(_) => {
            println!("Error: check environment variable OPENAI_API_KEY or try again later");
            std::process::exit(1);
        }
    };

    println!(
        "{}",
        json.choices[0]
            .text
            .split('\n')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>()
            .join("\n")
    );

    Ok(())
}
