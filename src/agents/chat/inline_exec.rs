use crate::util::shell::{Shell, run_shell};

use super::parse::extract_inline_commands;

/// Executes inline shell commands and returns a formatted output section, if any.
pub fn run_inline_commands(user_input: &str) -> Option<String> {
    let commands = extract_inline_commands(user_input);
    if commands.is_empty() {
        return None;
    }

    let mut entries = Vec::new();

    for cmd in commands {
        let out = run_shell(&cmd, Shell::BashLogin);

        match &out.error {
            Some(err) => entries.push(format!("$({})\n[error]\n{}", cmd, err)),
            None if out.success => {
                entries.push(format!(
                    "[section]\n[command]\n{}\n\n[stdout]\n{}\n[end section]",
                    cmd,
                    out.stdout_display()
                ));
                if !out.stderr.trim_end().is_empty() {
                    entries.push(format!("[stderr]\n{}", out.stderr_display()));
                }
            }
            None => entries.push(format!(
                "$({})\n[exit status]\n{}\n[stderr]\n{}\n[stdout]\n{}",
                cmd,
                out.status,
                out.stderr_display(),
                out.stdout_display()
            )),
        }
    }

    Some(entries.join("\n\n"))
}
