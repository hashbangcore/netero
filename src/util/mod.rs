//! Shared helpers used across agents, grouped by domain.
//!
//! This layer sits below `core` and `agents`: it must not depend on them.
//!
//! - [`shell`]: running shell commands and capturing their output.
//! - [`terminal`]: rendering and printing model output for the terminal.
//! - [`attach`]: reading files mentioned by the user and formatting them as context.
//! - [`text`]: tokenizing, path detection and block indentation.
//! - [`env`]: environment variables and local time.
//! - [`io`]: standard input.

pub mod attach;
pub mod env;
pub mod io;
pub mod shell;
pub mod terminal;
pub mod text;

pub use env::current_datetime;
pub use io::{get_stdin, stdin_is_piped};
pub use text::{looks_like_path, split_args};
