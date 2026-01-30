use ron::de::from_str;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
struct Prompts {
    convention: HashMap<String, String>,
    context: String,
    hint: String,
    instruction: String,
    skeleton: String,
    staged: String,
}

trait CoverPrompt {
    fn cover(&self, title: &str) -> String;
}

impl CoverPrompt for String {
    fn cover(&self, title: &str) -> String {
        format!(
            "### START {} ###\n{}\n### END {} ###",
            title.to_uppercase(),
            self,
            title.to_uppercase()
        )
    }
}

fn read_prompt_file() -> Prompts {
    let ron_data = include_str!("prompt.ron");
    from_str(ron_data).expect("Error parseando RON")
}

fn get_convention() -> String {
    let prompts = read_prompt_file();

    prompts
        .convention
        .iter()
        .map(|(k, v)| format!("{}: {}", k, v))
        .collect::<Vec<_>>()
        .join("\n")
}

pub fn generate() -> String {
    let prompts = read_prompt_file();

    let sections = [
        ("CONVENTION", &get_convention()),
        ("PROJECT CONTEXT", &prompts.context),
        ("USER HINT", &prompts.hint),
        ("INSTRUCTION", &prompts.instruction),
        ("SKELETON", &prompts.skeleton),
        ("STAGED CHANGES", &prompts.staged),
    ];

    sections
        .iter()
        .map(|(title, content)| content.cover(title))
        .collect::<Vec<_>>()
        .join("\n\n")
}
