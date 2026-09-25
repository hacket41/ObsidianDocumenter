mod config;
mod orchestrator;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("obsidian-agent starting...");
    Ok(())
}
