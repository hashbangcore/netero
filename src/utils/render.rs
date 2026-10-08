use std::io::IsTerminal;
use termimad::MadSkin;

use crate::core::interface::OutputFormat;

/// Renders markdown to terminal-friendly output (auto-detects terminal).
pub fn render_markdown(response: &str) -> String {
    if !std::io::stdout().is_terminal() {
        return response.to_string();
    }
    let skin = MadSkin::default();
    skin.term_text(response).to_string()
}

/// Renders markdown using a forced output format, ignoring terminal detection.
pub fn render_markdown_with(response: &str, output: Option<&OutputFormat>) -> String {
    match output {
        Some(OutputFormat::Markdown) => response.to_string(),
        Some(OutputFormat::Plain) => {
            let skin = MadSkin::default();
            let rendered = skin.term_text(response).to_string();
            strip_ansi_codes(&rendered)
        }
        None => render_markdown(response),
    }
}

fn strip_ansi_codes(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut in_escape = false;
    for c in s.chars() {
        if in_escape {
            if c == 'm' {
                in_escape = false;
            }
            continue;
        }
        if c == '\x1b' {
            in_escape = true;
            continue;
        }
        result.push(c);
    }
    result
}
