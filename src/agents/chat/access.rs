use crate::core;
use crate::utils;
use crate::utils::{attach, render};

use super::commands::{
    handle_add, handle_clean, handle_eval, handle_help, handle_save, handle_stream, handle_trans,
};
use super::inline_exec::run_inline_commands;
use super::input::{new_editor, open_tty_reader, read_user_input};
use super::parse::strip_inline_commands;
use super::prompt::create_prompt;
use super::stream::stream_completion;

/// Starts the interactive chat session and handles all supported commands.
pub async fn dispatch(
    service: &core::Service,
    args: &core::Cli,
    stdin: String,
    stdin_is_piped: bool,
) {
    let mut history: Vec<String> = Vec::new();
    let mut pending_stdin = if stdin.trim().is_empty() {
        None
    } else {
        Some(stdin)
    };
    let mut stream_enabled = false;
    let mut rl = new_editor();
    // When stdin is piped, read user input from TTY so the chat stays interactive.
    let mut tty_reader = match open_tty_reader(stdin_is_piped) {
        Ok(reader) => reader,
        Err(err) => {
            eprintln!("{}", err);
            return;
        }
    };

    loop {
        let user_input = match read_user_input(&mut rl, &mut tty_reader) {
            Ok(Some(line)) => line,
            Ok(None) => break,
            Err(err) => {
                eprintln!("{}", err);
                break;
            }
        };

        if user_input.is_empty() {
            continue;
        }

        if handle_clean(&user_input, &mut history) {
            continue;
        }

        if handle_help(&user_input) {
            continue;
        }

        if handle_add(&user_input, &mut history, &mut pending_stdin) {
            continue;
        }

        if handle_stream(&user_input, &mut stream_enabled) {
            continue;
        }

        match handle_trans(&user_input, service, args).await {
            Ok(true) => continue,
            Ok(false) => {}
            Err(err) => {
                eprintln!("{}", err);
                break;
            }
        }

        match handle_save(&user_input, service, args, &history).await {
            Ok(true) => continue,
            Ok(false) => {}
            Err(err) => {
                eprintln!("{}", err);
                break;
            }
        }

        if handle_eval(&user_input) {
            continue;
        }

        // Build the prompt with history, inline command output, and attachments.
        let dialog = history.join("\n");
        let command_output = run_inline_commands(&user_input);
        let cleaned_input = strip_inline_commands(&user_input);
        let (cleaned_input, attachments) = attach::extract_attachments_from_input(&cleaned_input);
        let attachment_block = attach::format_attachments(&attachments);
        let mut merged_stdin = String::new();
        if let Some(existing) = pending_stdin.as_deref() {
            merged_stdin.push_str(existing);
        }
        if let Some(extra) = attachment_block.as_deref() {
            merged_stdin.push_str(extra);
        }
        let merged_stdin = if merged_stdin.is_empty() {
            None
        } else {
            Some(merged_stdin)
        };
        let prompt = create_prompt(
            &utils::get_user(),
            &utils::current_datetime(),
            &utils::get_user_lang(),
            &dialog,
            &cleaned_input,
            command_output.as_deref(),
            merged_stdin.as_deref(),
        );
        if pending_stdin.is_some() {
            pending_stdin = None;
        }

        if args.verbose {
            println!("\x1b[32m{}\x1b[0m", prompt);
        }

        // Use streaming mode when enabled by the user.
        let response = if stream_enabled {
            match stream_completion(service, &prompt).await {
                Ok(text) => text,
                Err(err) => {
                    eprintln!("AI error: {}", err);
                    break;
                }
            }
        } else {
            match service.complete(&prompt).await {
                Ok(text) => {
                    let output = render::render_markdown(&text);
                    println!("\n{}", output);
                    text
                }
                Err(err) => {
                    eprintln!("AI error: {}", err);
                    break;
                }
            }
        };

        // Store the last exchange for future context.
        history.push(format!("{}: {}", utils::get_user(), cleaned_input));
        history.push(format!("Assistant: {}\n", response));
    }
}
