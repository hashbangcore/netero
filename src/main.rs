mod core;
mod task;

use clap::{Parser, Subcommand};
use std::env;

#[derive(Parser, Debug)]
#[command(name = "rave")]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
    prompt: Option<String>,
    #[arg(long, short, default_value = "codestral")]
    provider: String,
    #[arg(short, long, global = true)]
    verbose: bool,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Commit { hint: Option<String> },
    Chat { prompt: String },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    let ai = core::Service::new(Some(&args.provider));

    let ctx = core::CliContext {
        ai,
        verbose: args.verbose,
        provider: args.provider.to_string(),
    };

    execute(&ctx, args).await?;

    Ok(())
}

async fn execute(ctx: &core::CliContext, args: Cli) -> Result<(), Box<dyn std::error::Error>> {
    match args.command {
        Some(Commands::Commit { hint }) => generate_commit(ctx, hint.as_deref()).await?,
        Some(Commands::Chat { prompt }) => send_chat(ctx, &prompt).await?,
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
    let user = env::var("USER").unwrap_or("user".to_string());

    let response = ctx.ai.complete(request).await?;

    if ctx.verbose {
        println!("\x1b[1m{}:\x1b[0m\n\n{}\n", user.to_uppercase(), request);
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
