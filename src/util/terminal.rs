use std::io::IsTerminal;
use termimad::MadSkin;

/// Resets terminal styling.
const RESET: &str = "\x1b[0m";
/// Turns on bold styling.
const BOLD: &str = "\x1b[1m";
/// Turns on green coloring.
const GREEN: &str = "\x1b[32m";

/// How model output should be formatted for display.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OutputFormat {
    /// Render markdown to plain text (no ANSI codes)
    Plain,
    /// Keep raw markdown
    Markdown,
}

impl OutputFormat {
    /// Parses an output format name, returning `None` for unknown values.
    pub fn from_name(name: &str) -> Option<Self> {
        match name.trim().to_ascii_lowercase().as_str() {
            "plain" => Some(Self::Plain),
            "markdown" => Some(Self::Markdown),
            _ => None,
        }
    }
}

/// Renders markdown to terminal-friendly output (auto-detects terminal).
pub fn render_markdown(response: &str) -> String {
    render_markdown_auto(response, std::io::stdout().is_terminal())
}

/// Renders markdown depending on whether the destination is a terminal.
fn render_markdown_auto(response: &str, is_tty: bool) -> String {
    if !is_tty {
        return response.to_string();
    }
    let skin = MadSkin::default();
    skin.term_text(response).to_string()
}

/// Renders markdown using a forced output format, ignoring terminal detection.
pub fn render_markdown_with(response: &str, output: Option<OutputFormat>) -> String {
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
    let mut chars = s.chars();

    while let Some(c) = chars.next() {
        if c != '\x1b' {
            result.push(c);
            continue;
        }
        match chars.next() {
            // CSI sequences run until a final byte in the 0x40..=0x7e range.
            Some('[') => {
                for c in chars.by_ref() {
                    if ('\x40'..='\x7e').contains(&c) {
                        break;
                    }
                }
            }
            Some(_) => {}
            None => {}
        }
    }

    result
}

/// Prints raw prompt text highlighted, used by verbose mode.
pub fn print_verbose(text: &str) {
    println!("{GREEN}{text}{RESET}");
}

/// Prints a bold label followed by a text block, used by verbose mode.
pub fn print_labeled(label: &str, text: &str) {
    println!("{BOLD}{label}:{RESET}\n\n{text}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_known_format_names() {
        assert_eq!(OutputFormat::from_name("plain"), Some(OutputFormat::Plain));
        assert_eq!(OutputFormat::from_name("PLAIN"), Some(OutputFormat::Plain));
        assert_eq!(
            OutputFormat::from_name(" markdown "),
            Some(OutputFormat::Markdown)
        );
        assert_eq!(OutputFormat::from_name("bogus"), None);
    }

    #[test]
    fn plain_output_has_no_ansi_codes() {
        let rendered = render_markdown_with("un **texto**", Some(OutputFormat::Plain));
        assert!(!rendered.contains('\x1b'), "output was {rendered:?}");
        assert!(rendered.contains("texto"), "output was {rendered:?}");
    }

    #[test]
    fn strips_ansi_sequences_without_eating_text() {
        assert_eq!(strip_ansi_codes("\x1b[31mrojo\x1b[0m"), "rojo");
        assert_eq!(strip_ansi_codes("antes\x1b[2Jdespues"), "antesdespues");
        assert_eq!(strip_ansi_codes("sin escapes"), "sin escapes");
    }

    #[test]
    fn piped_output_is_not_rendered() {
        assert_eq!(render_markdown_auto("**crudo**", false), "**crudo**");
        assert!(!render_markdown_auto("**crudo**", true).contains("**"));
    }
}
