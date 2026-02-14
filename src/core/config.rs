use crate::core::Cli;

/// Configuration resolved from environment variables and CLI flags.
pub struct Config {
    pub endpoint: String,
    pub model: String,
    pub apikey: Option<String>,
    pub verbose: bool,
}

impl Config {
    /// Loads configuration from environment variables with sensible defaults.
    pub fn from_env(args: &Cli) -> Self {
        let url = std::env::var("NETERO_URL")
            .ok()
            .filter(|v| !v.trim().is_empty());

        let model = std::env::var("NETERO_MODEL")
            .ok()
            .filter(|v| !v.trim().is_empty());

        let key = std::env::var("NETERO_API_KEY")
            .ok()
            .filter(|v| !v.trim().is_empty());

        let (endpoint, model, apikey) = match (url, model) {
            (Some(u), Some(m)) => (u, m, key),
            (None, None) => (
                "https://codestral.mistral.ai/v1/chat/completions".to_string(),
                "codestral-latest".to_string(),
                std::env::var("CODE_API_KEY")
                    .ok()
                    .filter(|v| !v.trim().is_empty()),
            ),
            _ => panic!("NETERO_URL and NETERO_MODEL must be set together"),
        };

        Self {
            endpoint,
            model,
            apikey,
            verbose: args.verbose,
        }
    }
}
