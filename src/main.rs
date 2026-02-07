mod core;
mod tasks;
mod utils;

use clap::Parser;
use core::interfaz;
use tasks::commit;
use tasks::message;
use tasks::simple_chat;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stdin = utils::get_stdin();
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
        Some(interfaz::Commands::Commit { hint }) => commit::generate(ctx, hint.as_deref()).await?,
        Some(interfaz::Commands::Prompt { input }) => message::generate(ctx, &input).await?,
        Some(interfaz::Commands::Chat) => simple_chat::generate(ctx).await,
        None => {
            if let Some(prompt) = args.prompt {
                message::generate(ctx, &prompt).await?;
            } else {
                eprintln!("Error: a message is required for chat or commit");
            }
        }
    }

    Ok(())
}
