use crate::core;

use super::git::staged_changes;
use super::utils::{prompt as prompt_for, comment, cover, normalize_commit_message};

fn template(hint: Option<&str>, convention_text: &str) -> String {
    // Build a single prompt with all required sections.
    let user_hint = hint.unwrap_or("");

    let context = "repository context";
    let staged_changes = staged_changes();

    let sections = [
        ("INSTRUCTION", prompt_for("instruction")),
        ("CONVENTION", convention_text.to_string()),
        ("SKELETON", prompt_for("skeleton")),
        ("PROJECT CONTEXT", context.to_string()),
        ("USER HINT", user_hint.to_string()),
        ("REPOSITORY STATUS", staged_changes),
    ];

    sections
        .iter()
        .map(|(title, content)| cover(title, content))
        .collect::<Vec<_>>()
        .join("\n\n")
}

/// Builds a commit prompt, calls the model, and prints the final message.
pub async fn dispatch(
    service: &core::Service,
    args: &core::Cli,
    hint: Option<&str>,
    convention_path: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    let convention_text = if let Some(path) = convention_path {
        std::fs::read_to_string(path)?
    } else {
        prompt_for("convention")
    };

    let prompt = template(hint, &convention_text);

    if args.verbose {
        println!("{}\n\n", prompt);
    }

    let result = service.complete(&prompt).await?;
    let result = normalize_commit_message(&result);

    // TODO: manejar de forma más segura
    match result.contains("Error: no changes staged for commit") {
        true => println!("{}", result),
        false => println!("{}\n\n\n{}", result.trim_end(), comment(&convention_text)),
    }

    Ok(())
}
