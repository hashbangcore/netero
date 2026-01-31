use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<Message>,
}

#[derive(Deserialize)]
struct ChatResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    message: ResponseMessage,
}

#[derive(Deserialize)]
struct ResponseMessage {
    content: String,
}

pub struct Codestral {
    http: Client,
    api_key: String,
}

impl Codestral {
    pub fn new() -> Self {
        let key =
            std::env::var("CODE_API_KEY").expect("Variable de entorno CODE_API_KEY no configurada");
        Self {
            http: Client::new(),
            api_key: key,
        }
    }

    pub async fn complete(&self, input: &str) -> Result<String, Box<dyn std::error::Error>> {
        let body = ChatRequest {
            model: "codestral-latest".to_string(),
            messages: vec![Message {
                role: "user".to_string(),
                content: input.to_string(),
            }],
        };

        let response = self
            .http
            .post("https://codestral.mistral.ai/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&body)
            .send()
            .await?
            .json::<ChatResponse>()
            .await?;

        Ok(response.choices[0].message.content.clone())
    }
}
