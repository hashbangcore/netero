use ron::de::from_str;
use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
struct Prompts {
    convention: String,
    context: String,
    hint: String,
    instruction: String,
    skeleton: String,
    staged: String,
}

fn expand_prompt(path: &str) -> String {
    fs::read_to_string(path)
        .unwrap_or_else(|_| format!("Error: No se pudo leer el archivo {}", path))
}

fn cover(title: &str, content: &str) -> String {
    let t = title.to_uppercase();
    format!("### START {t} ###\n{content}\n### END {t} ###")
}

pub fn generate() -> String {
    let ron_data = include_str!("prompt.ron");
    let p: Prompts = from_str(ron_data).expect("Error parseando RON");

    let sections = [
        ("CONVENTION", expand_prompt(&p.convention)),
        ("PROJECT CONTEXT", p.context),
        ("USER HINT", p.hint),
        ("INSTRUCTION", expand_prompt(&p.instruction)),
        ("SKELETON", expand_prompt(&p.skeleton)),
        ("STAGED CHANGES", expand_prompt(&p.staged)),
    ];

    sections
        .iter()
        .map(|(title, content)| cover(title, content))
        .collect::<Vec<_>>()
        .join("\n\n")
}
