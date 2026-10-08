use std::process::Command;

use super::parse::extract_inline_commands;

/// Executes inline shell commands and returns a formatted output section, if any.
pub fn run_inline_commands(user_input: &str) -> Option<String> {
    let commands = extract_inline_commands(user_input);
    if commands.is_empty() {
        return None;
    }

    let mut entries = Vec::new();

    for cmd in commands {
        let output = Command::new("bash").args(["-lc", &cmd]).output();

        match output {
            Ok(out) => {
                let stdout = String::from_utf8_lossy(&out.stdout).trim_end().to_string();
                let stderr = String::from_utf8_lossy(&out.stderr).trim_end().to_string();

                if out.status.success() {
                    let stdout_display = if stdout.is_empty() {
                        "<empty>"
                    } else {
                        &stdout
                    };
                    entries.push(format!(
                        "[section]\n[command]\n{}\n\n[stdout]\n{}\n[end section]",
                        cmd, stdout_display
                    ));
                    if !stderr.is_empty() {
                        entries.push(format!("[stderr]\n{}", stderr));
                    }
                } else {
                    let stderr_display = if stderr.is_empty() {
                        "<empty>"
                    } else {
                        &stderr
                    };
                    let stdout_display = if stdout.is_empty() {
                        "<empty>"
                    } else {
                        &stdout
                    };
                    entries.push(format!(
                        "$({})\n[exit status]\n{}\n[stderr]\n{}\n[stdout]\n{}",
                        cmd, out.status, stderr_display, stdout_display
                    ));
                }
            }
            Err(err) => {
                entries.push(format!("$({})\n[error]\n{}", cmd, err));
            }
        }
    }

    Some(entries.join("\n\n"))
}
