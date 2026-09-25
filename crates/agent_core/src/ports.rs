use crate::note::Note;
use async_trait::async_trait;

#[derive(Debug, Clone)]
pub struct CompletionRequest {
    pub model: String,
    pub system: Option<String>,
    pub prompt: String,
    pub temperature: f32,
}

#[derive(Debug, Clone)]
pub struct CompletionResponse {
    pub text: String,
}

#[async_trait]
pub trait LlmClient: Send + Sync {
    async fn complete(&self, req: CompletionRequest) -> anyhow::Result<CompletionResponse>;
    async fn list_models(&self) -> anyhow::Result<Vec<String>>;
}

#[async_trait]
pub trait NoteStore: Send + Sync {
    async fn write_note(&self, note: &Note) -> anyhow::Result<()>;
    async fn read_note(&self, title: &str) -> anyhow::Result<Option<Note>>;
    async fn list_notes(&self) -> anyhow::Result<Vec<String>>;
}
