use agent_core::note::Note;
use std::path::PathBuf;

pub fn note_path(vault_dir: &std::path::Path, title: &str) -> PathBuf {
    let safe = title
        .chars()
        .map(|c| if c.is_alphanumeric() || c == ' ' || c == '-' || c == '_' { c } else { '_' })
        .collect::<String>();
    vault_dir.join(format!("{safe}.md"))
}

pub async fn write(vault_dir: &std::path::Path, note: &Note) -> anyhow::Result<()> {
    tokio::fs::create_dir_all(vault_dir).await?;
    let path = note_path(vault_dir, &note.title);
    tokio::fs::write(path, note.to_markdown()).await?;
    Ok(())
}
