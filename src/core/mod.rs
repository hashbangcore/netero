use std::env;

pub fn get_api_key() -> String {
    env::var("CODE_API_KEY").expect("Environment variable CODE_API_KEY not set")
}
