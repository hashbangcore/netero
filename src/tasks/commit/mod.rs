//! Commit message generation task and helpers.
pub mod action;
mod git;
mod utils;

pub use action::dispatch;
