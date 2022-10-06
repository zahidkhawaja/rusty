use hyper::{body::Buf, header, Body, Client, Request};
use hyper_tls::HttpsConnector;
use serde_derive::{Deserialize, Serialize};
use std::{env, env::args};

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
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Check for environment variable OPENAI_KEY
    let api_key = match env::var("OPENAI_KEY") {
        Ok(key) => key,
        Err(_) => {
            println!("Error: please create an environment variable OPENAI_KEY");
            std::process::exit(1);
        }
    };

    let https = HttpsConnector::new();
    let client = Client::builder().build(https);
    let uri = "https://api.openai.com/v1/completions";

    let model = String::from("text-davinci-002");

    let default_prompt =
        "Given text, return 1 bash command. Text:list contents of a directory. Command:ls";
    let mut user_input = String::new();

    let mut arguments: Vec<String> = args().collect();
    arguments.remove(0);

    if arguments.is_empty() {
        println!("Welcome to Rusty! Enter an argument to get started.");
        std::process::exit(1);
    }

    for x in arguments {
        user_input.push(' ');
        user_input.push_str(&x);
    }

    let auth_header_val = format!("Bearer {}", api_key);

    let openai_request = OpenAIRequest {
        model,
        prompt: format!("{}. Text:{}. Command:", default_prompt, user_input),
        max_tokens: 64,
    };

    let body = Body::from(serde_json::to_vec(&openai_request)?);

    let req = Request::post(uri)
        .header(header::CONTENT_TYPE, "application/json")
        .header("Authorization", &auth_header_val)
        .body(body)
        .unwrap();

    let res = client.request(req).await?;

    let body = hyper::body::aggregate(res).await?;

    let json: OpenAIResponse = serde_json::from_reader(body.reader())?;

    println!("{}", json.choices[0].text);

    Ok(())
}
