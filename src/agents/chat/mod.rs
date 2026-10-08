//! Chat task implementation and helpers.
pub mod access;
mod commands;
mod eval;
mod inline_exec;
mod input;
mod lang;
mod parse;
mod prompt;
mod stream;

pub use access::dispatch;
