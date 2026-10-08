//! Shared helpers used across agents, grouped by domain.
//!
//! This layer sits below `core` and `agents`: it must not depend on them.
//!
//! - [`shell`]: running shell commands and capturing their output.
//! - [`terminal`]: rendering and printing model output for the terminal.
//! - [`attach`]: reading files mentioned by the user and formatting them as context.
//! - [`text`]: tokenizing, path detection and block indentation.
//! - [`env`]: environment variables, locale and local time.
//! - [`io`]: standard input.
//! - [`lang`]: language tag normalization and display names.

pub mod attach;
pub mod env;
pub mod io;
pub mod lang;
pub mod shell;
pub mod terminal;
pub mod text;

pub use env::{current_datetime, get_user_lang};
pub use io::{get_stdin, stdin_is_piped};
pub use lang::{lang_display_name, normalize_lang_tag};
pub use text::{looks_like_path, split_args};
