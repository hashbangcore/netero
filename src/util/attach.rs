use std::env;
use std::fs;

use super::text::{indent_block, looks_like_path, split_args};

/// File attachment extracted from user input.
pub struct Attachment {
    /// Path as written by the user (not expanded).
    pub path: String,
    pub content: String,
}

/// Expands a leading `~/` into the current home directory.
fn expand_path(path: &str) -> String {
    match path.strip_prefix("~/") {
        Some(rest) => match env::var("HOME") {
            Ok(home) => format!("{}/{}", home, rest),
            Err(_) => path.to_string(),
        },
        None => path.to_string(),
    }
}

fn read_file(path: &str) -> Option<String> {
    fs::read_to_string(path).ok()
}

/// Extracts file attachments from tokenized input.
fn extract_attachments_from_tokens(tokens: &[String]) -> (Vec<String>, Vec<Attachment>) {
    let mut remaining = Vec::new();
    let mut attachments = Vec::new();

    for token in tokens {
        if !looks_like_path(token) {
            remaining.push(token.clone());
            continue;
        }

        let expanded = expand_path(token);
        match fs::metadata(&expanded) {
            Ok(meta) if meta.is_file() => match read_file(&expanded) {
                Some(content) => attachments.push(Attachment {
                    path: token.clone(),
                    content,
                }),
                None => remaining.push(token.clone()),
            },
            _ => remaining.push(token.clone()),
        }
    }

    (remaining, attachments)
}

/// Extracts file attachments from a raw input string.
pub fn extract_attachments_from_input(input: &str) -> Vec<Attachment> {
    let tokens = split_args(input);
    let (_remaining, attachments) = extract_attachments_from_tokens(&tokens);
    attachments
}

/// Formats attachments into a single block, compatible with stdin attachments.
pub fn format_attachments(attachments: &[Attachment]) -> Option<String> {
    if attachments.is_empty() {
        return None;
    }
    let mut out = String::new();
    for attachment in attachments {
        out.push_str("\n-- FILE: ");
        out.push_str(&attachment.path);
        out.push_str(" --\n");
        out.push_str(&attachment.content);
        out.push('\n');
    }
    Some(out)
}

/// Formats stdin and file attachments into a single attached files block.
pub fn format_attached_files(stdin: Option<&str>, attachments: &[Attachment]) -> Option<String> {
    let mut sections = Vec::new();
    if let Some(content) = stdin.filter(|content| !content.trim().is_empty()) {
        sections.push(format!(
            "-- FILE: STDIN --\n{}",
            indent_block(content, "      ")
        ));
    }
    for attachment in attachments {
        sections.push(format!(
            "-- FILE: {} --\n{}",
            attachment.path,
            indent_block(&attachment.content, "      ")
        ));
    }
    if sections.is_empty() {
        return None;
    }
    let mut out = String::new();
    out.push_str(":: ATTACHED FILES ::\n\n");
    out.push_str(&sections.join("\n\n"));
    out.push_str("\n\n:: END ATTACHED FILES ::");
    Some(out)
}
