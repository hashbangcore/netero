use crate::core::types::*;
use reqwest::Client;

impl Service {
    pub fn new(provider: Option<&str>) -> Self {
        let provider = provider.unwrap_or("codestral");

        let (envar, endpoint, model) = match provider {
            "codestral" => (
                "CODE_API_KEY",
                "https://codestral.mistral.ai/v1/chat/completions",
                "codestral-latest",
            ),

            "openrouter" => (
                "OPENROUTER_API_KEY",
                "https://openrouter.ai/api/v1/chat/completions",
                "openrouter/free",
            ),
            _ => todo!("{}", provider),
        };

        let apikey = std::env::var(envar.to_owned()).expect("variable not found.");

        Self {
            http: Client::new(),
            apikey: apikey,
            endpoint: endpoint.to_owned(),
            model: model.to_owned(),
        }
    }

    pub async fn complete(&self, content: &str) -> Result<String, Box<dyn std::error::Error>> {
        let body = ChatRequest {
            model: self.model.clone(),
            messages: vec![Message {
                role: "user".to_string(),
                content: content.to_string(),
            }],
        };

        let response = self
            .http
            .post(&self.endpoint)
            .header("Authorization", format!("Bearer {}", self.apikey))
            .json(&body)
            .send()
            .await?
            .json::<ChatResponse>()
            .await?;

        Ok(response.choices[0].message.content.clone())
    }
}
