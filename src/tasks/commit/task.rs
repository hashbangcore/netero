use crate::core;

use std::process::Command;

fn prompt_instruction() -> &'static str {
    include_str!("prompts/instruction.txt")
}

fn prompt_convention() -> &'static str {
    include_str!("prompts/convention.txt")
}

fn prompt_skeleton() -> &'static str {
    include_str!("prompts/skeleton.txt")
}

fn cover(title: &str, content: &str) -> String {
    let t = title.to_uppercase();
    format!(":: START {t} ::\n{content}\n:: END {t} ::")
}

fn comment(text: &str) -> String {
    text.lines()
        .map(|line| format!("# {}", line))
        .collect::<Vec<_>>()
        .join("\n")
}

fn staged_changes() -> String {
    run_commands(&[
        "cat $(find . -type f -name 'README.md' -not -path '*/target/*' -not -path '*/docs/*' | xargs -I {} realpath {})",
        "git branch -v",
        "git remote -v",
        "git log --stat -1",
        "git status -s",
        "git status",
        "git diff --cached --quiet && echo 'No staged changes' || echo 'Staged changes present' && git diff --staged",
    ])
}

fn run_commands(commands: &[&str]) -> String {
    let mut sections = Vec::with_capacity(commands.len());

    for cmd_str in commands {
        let output = Command::new("sh").arg("-c").arg(cmd_str).output();
        match output {
            Ok(out) => {
                let stdout = String::from_utf8_lossy(&out.stdout);
                let stderr = String::from_utf8_lossy(&out.stderr);
                let combined_output = if !stderr.is_empty() {
                    format!("{}{}", stdout, stderr)
                } else {
                    stdout.to_string()
                };

                sections.push(format!(
                    "[section]\n[command]\n{}\n[output]\n{}\n[end section]",
                    cmd_str,
                    combined_output.trim_end()
                ));
            }
            Err(err) => {
                sections.push(format!(
                    "[section]\n[command]\n{}\n[error]\n{}\n[end section]",
                    cmd_str, err
                ));
            }
        }
    }

    sections.join("\n\n")
}

fn generate(hint: Option<&str>) -> String {
    let user_hint = hint.unwrap_or("");

    let context = "repository context";
    let staged_changes = staged_changes();

    let sections = [
        ("INSTRUCTION", prompt_instruction().to_string()),
        ("CONVENTION", prompt_convention().to_string()),
        ("SKELETON", prompt_skeleton().to_string()),
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

fn normalize_commit_message(message: &str) -> String {
    let trimmed = message.trim_end();
    let lines: Vec<&str> = trimmed.split('\n').collect();
    if lines.len() <= 1 {
        return trimmed.to_string();
    }
    if !lines[1].is_empty() {
        let mut out = String::new();
        out.push_str(lines[0]);
        out.push('\n');
        out.push('\n');
        out.push_str(&lines[1..].join("\n"));
        return out;
    }
    trimmed.to_string()
}

pub async fn generate_commit(
    service: &core::Service,
    args: &core::Cli,
    hint: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    let prompt = generate(hint);

    if args.verbose {
        println!("{}\n\n", prompt);
    }

    let result = service.complete(&prompt).await?;
    let result = normalize_commit_message(&result);

    // TODO: manejar de forma más segura
    match result.contains("Error: no changes staged for commit") {
        true => println!("{}", result),
        false => println!("{}\n\n\n{}", result.trim_end(), comment(prompt_convention())),
    }

    Ok(())
}
