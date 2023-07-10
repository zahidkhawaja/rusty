use hyper::{body::Buf, header, Body, Client, Request};
use hyper_tls::HttpsConnector;
use serde_derive::{Deserialize, Serialize};
use std::error::Error;
use std::io::{stdin, stdout, Write};
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
    let uri = "https://api.openai.com/v1/completions";

    let model = String::from("text-davinci-003");
    let stop = String::from("Text");

    let default_prompt =
        "Given text, return 1 bash command. Text:list contents of a directory. Command:ls";
    let mut user_input = String::new();

    let mut arguments: Vec<String> = args().collect();
    arguments.remove(0);

    if arguments.is_empty() {
        print!("Enter prompt: ");
        let _ = stdout().flush();
        stdin()
            .read_line(&mut user_input)
            .expect("Failed to read prompt.");
    } else {
        for x in arguments {
            user_input.push(' ');
            user_input.push_str(&x);
        }
    }

    let auth_header_val = format!("Bearer {}", api_key);

    let openai_request = OpenAIRequest {
        model,
        prompt: format!("{} Text:{}. Command:", default_prompt, user_input),
        max_tokens: 64,
        stop,
    };

    let body = Body::from(serde_json::to_vec(&openai_request)?);

    let req = Request::post(uri)
        .header(header::CONTENT_TYPE, "application/json")
        .header("Authorization", &auth_header_val)
        .body(body)
        .unwrap();

    let res = client.request(req).await?;

    let body = hyper::body::aggregate(res).await?;

    let json: OpenAIResponse = match serde_json::from_reader(body.reader()) {
        Ok(response) => response,
        Err(_) => {
            println!("Error calling OpenAI. Check environment variable OPENAI_API_KEY");
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
