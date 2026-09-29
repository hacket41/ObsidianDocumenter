use agent_core::note::Note;
use agent_core::ports::NoteStore;
use async_trait::async_trait;
use std::path::PathBuf;

pub struct VaultStore {
    dir: PathBuf,
}

impl VaultStore {
    pub fn new(dir: impl Into<PathBuf>) -> Self {
        Self { dir: dir.into() }
    }
}

#[async_trait]
impl NoteStore for VaultStore {
    async fn write_note(&self, note: &Note) -> anyhow::Result<()> {
        crate::writer::write(&self.dir, note).await
    }

    async fn read_note(&self, title: &str) -> anyhow::Result<Option<Note>> {
        let path = crate::writer::note_path(&self.dir, title);
        if !path.exists() {
            return Ok(None);
        }
        let content = tokio::fs::read_to_string(&path).await?;
        Ok(Some(Note {
            title: title.to_string(),
            content,
            tags: vec![],
            links: vec![],
        }))
    }

    async fn list_notes(&self) -> anyhow::Result<Vec<String>> {
        if !self.dir.exists() {
            return Ok(vec![]);
        }
        let mut names = vec![];
        for entry in walkdir::WalkDir::new(&self.dir).max_depth(1) {
            let entry = entry?;
            if entry.path().extension().map(|e| e == "md").unwrap_or(false) {
                if let Some(stem) = entry.path().file_stem() {
                    names.push(stem.to_string_lossy().to_string());
                }
            }
        }
        Ok(names)
    }
}
