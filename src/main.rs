mod core;
mod task;

use clap::Parser;
use core::interfaz;
use std::env;
use std::io::{self, IsTerminal, Read};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut stdin = String::new();

    if !io::stdin().is_terminal() {
        io::stdin().read_to_string(&mut stdin).unwrap();
    }

    let args = interfaz::Cli::parse();
    let ai = core::Service::new(Some(&args.provider));

    let ctx = core::CliContext {
        ai,
        stdin,
        verbose: args.verbose,
        provider: args.provider.to_string(),
    };

    execute(&ctx, args).await?;

    Ok(())
}

async fn execute(
    ctx: &core::CliContext,
    args: interfaz::Cli,
) -> Result<(), Box<dyn std::error::Error>> {
    match args.command {
        Some(interfaz::Commands::Commit { hint }) => generate_commit(ctx, hint.as_deref()).await?,
        Some(interfaz::Commands::Prompt { input }) => send_chat(ctx, &input).await?,
        None => {
            if let Some(prompt) = args.prompt {
                send_chat(ctx, &prompt).await?;
            } else {
                eprintln!("Error: a message is required for chat or commit");
            }
        }
    }

    Ok(())
}

async fn generate_commit(
    ctx: &core::CliContext,
    hint: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    let prompt = task::commit::prompt::generate(hint);

    if ctx.verbose {
        println!("{}\n\n", prompt);
    }

    let result = ctx.ai.complete(&prompt).await?;

    println!("{}", result);

    Ok(())
}

async fn send_chat(
    ctx: &core::CliContext,
    request: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let user = env::var("USER").unwrap_or_else(|_| "user".to_string());

    let prompt = if ctx.stdin.trim().is_empty() {
        format!(
            "== USER REQUEST ==\n{}\n== END USER REQUEST ==",
            request.trim()
        )
    } else {
        format!(
            "== STDIN FILE ==\n{}\n== END STDIN FILE ==\n\n== USER REQUEST ==\n{}\n== END USER REQUEST ==",
            ctx.stdin.trim(),
            request.trim()
        )
    };

    let response = ctx.ai.complete(&prompt).await?;

    if ctx.verbose {
        println!("\x1b[1m{}:\x1b[0m\n\n{}\n", user.to_uppercase(), prompt);
        println!(
            "\x1b[1m{}:\x1b[0m\n\n{}",
            ctx.provider.to_uppercase(),
            response.trim()
        );
    } else {
        println!("{}", response.trim());
    }

    Ok(())
}
