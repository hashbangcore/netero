use reqwest::Client;
use serde::{Deserialize, Serialize};

pub struct Service {
    pub http: Client,
    pub apikey: Option<String>,
    pub endpoint: String,
    pub model: String,
}
pub struct CliContext {
    pub ai: Service,
    pub verbose: bool,
    pub provider: String,
    pub stdin: String,
}

#[derive(Serialize)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Serialize)]
pub struct ChatRequest {
    pub model: String,
    pub messages: Vec<Message>,
}

#[derive(Deserialize)]
pub struct ChatResponse {
    pub choices: Vec<Choice>,
}

#[derive(Deserialize)]
pub struct Choice {
    pub message: ResponseMessage,
}

#[derive(Deserialize)]
pub struct ResponseMessage {
    pub content: String,
}
