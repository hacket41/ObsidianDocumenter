mod config;
mod orchestrator;

use agent_core::agent::Agent;
use llm_ollama::client::OllamaClient;
use obsidian_store::vault::VaultStore;
use std::sync::Arc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let topic = std::env::args().nth(1).unwrap_or_else(|| "Rust ownership model".to_string());
    let model = std::env::args().nth(2).unwrap_or_else(|| "llama3.1".to_string());

    let llm = Arc::new(OllamaClient::new("http://localhost:11434"));
    let store = Arc::new(VaultStore::new("./vault"));

    let agent = Agent::new(llm, store, model);

    println!("Generating note for topic: \"{topic}\" using model \"{}\"", agent.model);
    let note = agent.process_topic(&topic).await?;
    println!("Note Documented: {}", note.title);

    Ok(())
}
