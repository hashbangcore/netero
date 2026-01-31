mod core;
mod task;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ai = core::Codestral::new();
    let prompt = task::commit::prompt::generate();

    let result = ai.complete(&prompt).await?;

    println!("##### PROMPT #####");
    println!("{}", prompt);
    println!("##### RESPONSE #####");
    println!("{}", result);

    Ok(())
}
