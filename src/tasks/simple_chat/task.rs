use crate::core;
use crate::utils;
use std::io::{self, BufRead};

fn crete_prompt(history: &str, user_input: &str) -> String {
    format!(
        "
LLM: Netero
USER: {}
Datetime: {}

== INSTRUCTION ==
- Do not use emojis
- Respond with between 1 and 15 lines at most
- Keep answers short, avoid long explanations
- Respond in the same language as the user
== END INSTRUCTION ==

== HISTORIAL CHAT ==
{}
== END HISTORIAL CHAT ==

== USER MESSAGE ==
{}
== END USER MESSAGE ==
",
        utils::get_user(),
        utils::current_datetime(),
        history,
        user_input
    )
}

pub async fn connect(ctx: &core::CliContext) {
    let provider = ctx.provider.clone();
    let mut history: Vec<String> = Vec::new();

    println!("::: Simple Chat Started ::: {} :::\n", provider);

    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        let user_input = match line {
            Ok(l) => l.trim().to_string(),
            Err(_) => break,
        };

        if user_input.is_empty() {
            continue;
        }

        let dialog = history.join("\n");
        let prompt = crete_prompt(&dialog, &user_input);

        if ctx.verbose {
            println!("{}", prompt);
        }

        match ctx.ai.complete(&prompt).await {
            Ok(text) => {
                let output = utils::render_markdown(&text);
                println!("\x1b[90m{}\x1b[0m", output);

                history.push(format!("USER: {}", user_input));
                history.push(format!("ASSISTANT: {}\n", text));
            }
            Err(err) => {
                eprintln!("AI error: {}", err);
                break;
            }
        }
    }
}
