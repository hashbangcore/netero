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
    texto: Option<String>,
    #[arg(long, short, default_value = "codestral")]
    provider: String,
    #[arg(short, long, global = true)]
    verbose: bool,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Commit { hint: Option<String> },
    Chat { texto: String },
}

pub struct AppContext {
    pub ai: core::Service,
    pub verbose: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    let ai = core::Service::new(Some(&args.provider));

    let ctx = AppContext {
        ai,
        verbose: args.verbose,
    };

    execute(&ctx, args).await?;

    Ok(())
}

async fn execute(ctx: &AppContext, args: Cli) -> Result<(), Box<dyn std::error::Error>> {
    match args.command {
        Some(Commands::Commit { hint }) => generate_commit(ctx, hint.as_deref()).await?,
        Some(Commands::Chat { texto }) => send_chat(ctx, &texto).await?,
        None => {
            if let Some(texto) = args.texto {
                send_chat(ctx, &texto).await?;
            } else {
                eprintln!("Error: a message is required for chat or commit");
            }
        }
    }

    Ok(())
}

async fn generate_commit(
    ctx: &AppContext,
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

async fn send_chat(ctx: &AppContext, mensaje: &str) -> Result<(), Box<dyn std::error::Error>> {
    let user = env::var("USER").unwrap_or("user".to_string());

    if ctx.verbose {
        println!("{}: {} \n", user, mensaje);
    }

    let respuesta = ctx.ai.complete(mensaje).await?;
    println!("assistant: {}", respuesta);
    Ok(())
}
