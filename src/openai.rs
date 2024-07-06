use hyper::{body::Buf, header, Body, Client, Request};
use hyper_tls::HttpsConnector;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct OpenAIChoiceMessage {
    role: String,
    content: String,
}

#[derive(Deserialize, Debug)]
struct OpenAIChoices {
    message: OpenAIChoiceMessage,
}

/// Response from OpenAI
#[derive(Deserialize, Debug)]
struct OpenAIResponse {
    choices: Vec<OpenAIChoices>,
}

/// Data structure to send to OpenAI
#[derive(Serialize, Debug)]
struct OpenAIRequest {
    model: String,
    messages: Vec<OpenAIChoiceMessage>,
}

const URI: &str = "https://api.openai.com/v1/chat/completions";

/// Configuration file
#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub api_key: String,
    pub model: String,
}
/// Query to OpenAI
pub struct OpenAIQuery {
    /// The query/text to send to OpenAI
    pub query: String,
    /// OpenAI API key
    pub api_key: String,
    /// OpenAI model name. Default is "gpt-3.5-turbo"
    pub model: String,
}

/// Generate a bash command using OpenAI
pub async fn openai_query(query: OpenAIQuery) -> Result<String, String> {
    let OpenAIQuery {
        query,
        api_key,
        model,
    } = query;

    let https = HttpsConnector::new();
    let client = Client::builder().build(https);

    let auth_header_val = format!("Bearer {}", api_key);

    let openai_request = OpenAIRequest {
        model,
        messages: vec![
            OpenAIChoiceMessage {
                role: "system".to_string(),
                content: String::from("You are bash command generator. Only return the command."),
            },
            OpenAIChoiceMessage {
                role: "user".to_string(),
                content: String::from("How to list contents of a directory in bash?"),
            },
            OpenAIChoiceMessage {
                role: "assistant".to_string(),
                content: "ls".to_string(),
            },
            OpenAIChoiceMessage {
                role: "user".to_string(),
                content: query.trim().to_string(),
            },
        ],
    };

    // TODO: handle error
    let body = Body::from(serde_json::to_vec(&openai_request).unwrap());

    let req = Request::post(URI)
        .header(header::CONTENT_TYPE, "application/json")
        .header("Authorization", &auth_header_val)
        .body(body)
        .unwrap();

    let res = client.request(req).await.unwrap();

    let body = hyper::body::aggregate(res).await.unwrap();

    let json: OpenAIResponse = match serde_json::from_reader(body.reader()) {
        Ok(response) => response,
        Err(e) => {
            println!("Error: check environment variable OPENAI_API_KEY or try again later");
            println!("Error: {}", e);
            std::process::exit(1);
        }
    };

    let bash = json.choices[0]
        .message
        .content
        .lines()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("\n");

    Ok(bash)
}
