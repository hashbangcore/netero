use crate::core::Cli;
use crate::util::env::env_var;

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
        // Read env vars only once to keep behavior consistent.
        let url = env_var("NETERO_URL");
        let model = env_var("NETERO_MODEL");
        let key = env_var("NETERO_API_KEY");

        let (endpoint, model, apikey) = match (url, model) {
            (Some(u), Some(m)) => (u, m, key),
            (None, None) => (
                "https://codestral.mistral.ai/v1/chat/completions".to_string(),
                "codestral-latest".to_string(),
                env_var("CODE_API_KEY"),
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
