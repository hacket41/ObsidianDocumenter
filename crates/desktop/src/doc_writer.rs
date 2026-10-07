use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

/// Writes a markdown snapshot into the vault. Right now the "activity"
/// being documented is a placeholder string — step 2 (hooking this up to
/// actually watch a coding project and summarize what changed) is a
pub fn write_snapshot(vault_dir: &PathBuf, activity_summary: &str) -> anyhow::Result<PathBuf> {
    std::fs::create_dir_all(vault_dir)?;

    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
    let filename = format!("session-{timestamp}.md");
    let path = vault_dir.join(&filename);

    // Obsidian renders '#' as a heading automatically; this just writes
    // standard markdown headings, which IS how Obsidian expects it — no
    // special escaping needed on our end.
    let content = format!(
        "# Session Log\n\n## Summary\n{activity_summary}\n\n## Related\n- [[Welcome]]\n"
    );

    std::fs::write(&path, content)?;
    Ok(path)
}
