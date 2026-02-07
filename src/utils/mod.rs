use chrono::Local;
use std::env;
use std::io::{self, IsTerminal, Read};
use termimad::{MadSkin, crossterm::style::Color};

pub fn get_stdin() -> String {
    let mut input = String::new();

    if !io::stdin().is_terminal() {
        io::stdin().read_to_string(&mut input).unwrap();
    }
    return input;
}

pub fn get_user() -> String {
    let user = env::var("USER").unwrap_or_else(|_| "user".to_string());
    capitalize(&user)
}

pub fn current_datetime() -> String {
    Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
}

pub fn capitalize(s: &str) -> String {
    s.get(0..1).unwrap_or("").to_uppercase() + s.get(1..).unwrap_or("")
}

pub fn render_markdown(response: &str) -> String {
    let skin = MadSkin::default();

    // Estilo sobrio para CLI
    //skin.bold.set_fg(Color::Cyan);
    //skin.italic.set_fg(Color::Grey);
    //skin.inline_code.set_fg(Color::Yellow);
    //skin.code_block.set_bg(Color::DarkGrey);
    //skin.paragraph.set_fg(Color::White);

    skin.term_text(response).to_string()
}
