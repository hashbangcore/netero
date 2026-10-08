use crate::core::Cli;
use crate::core::router::ServiceError;
use crate::util::env::env_var;

/// Configuration resolved from environment variables and CLI flags.
///
/// Netero is provider agnostic: endpoint, model and key always come from the
/// environment, there is no built-in provider.
pub struct Config {
    pub endpoint: String,
    pub model: String,
    pub apikey: Option<String>,
    pub verbose: bool,
}

impl Config {
    /// Loads configuration from environment variables.
    pub fn from_env(args: &Cli) -> Result<Self, ServiceError> {
        let endpoint = env_var("NETERO_URL");
        let model = env_var("NETERO_MODEL");
        let apikey = env_var("NETERO_API_KEY");

        let missing = missing_vars(endpoint.as_deref(), model.as_deref());
        if !missing.is_empty() {
            return Err(ServiceError::Config(format!(
                "falta definir {}",
                missing.join(" y ")
            )));
        }

        Ok(Self {
            endpoint: endpoint.unwrap_or_default(),
            model: model.unwrap_or_default(),
            apikey,
            verbose: args.verbose,
        })
    }
}

/// Lists the variables that still need to be set.
fn missing_vars(endpoint: Option<&str>, model: Option<&str>) -> Vec<&'static str> {
    [("NETERO_URL", endpoint), ("NETERO_MODEL", model)]
        .into_iter()
        .filter_map(|(name, value)| value.is_none().then_some(name))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lists_both_variables_when_nothing_is_set() {
        assert_eq!(missing_vars(None, None), vec!["NETERO_URL", "NETERO_MODEL"]);
    }

    #[test]
    fn lists_only_the_missing_one() {
        assert_eq!(
            missing_vars(Some("http://x/v1"), None),
            vec!["NETERO_MODEL"]
        );
        assert_eq!(missing_vars(None, Some("model")), vec!["NETERO_URL"]);
    }

    #[test]
    fn lists_nothing_when_both_are_set() {
        assert!(missing_vars(Some("http://x/v1"), Some("model")).is_empty());
    }

    #[test]
    fn names_every_missing_variable_in_one_message() {
        let missing = missing_vars(None, None).join(" y ");
        assert_eq!(
            ServiceError::Config(format!("falta definir {missing}")).to_string(),
            "falta definir NETERO_URL y NETERO_MODEL"
        );
    }
}
