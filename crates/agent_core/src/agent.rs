use crate::note::Note;
use crate::ports::{CompletionRequest, LlmClient, NoteStore};
use std::sync::Arc;

pub struct Agent {
    pub llm: Arc<dyn LlmClient>,
    pub store: Arc<dyn NoteStore>,
    pub model: String,
}

impl Agent {
    pub fn new(llm: Arc<dyn LlmClient>, store: Arc<dyn NoteStore>, model: impl Into<String>) -> Self {
        Self { llm, store, model: model.into() }
    }

    pub fn set_model(&mut self, model: impl Into<String>) {
        self.model = model.into();
    }

    /// Generates a note on `topic`, aware of existing vault notes, and writes it.
    pub async fn process_topic(&self, topic: &str) -> anyhow::Result<Note> {
        let existing = self.store.list_notes().await?;

        let system = "You are a knowledge-graph building assistant for an Obsidian vault. \
            Respond ONLY with valid JSON, no markdown fences, no preamble. \
            Schema: {\"title\": string, \"content\": string, \"tags\": string[], \"links\": string[]}. \
            \"links\" should be concept titles related to the topic, preferring names from \
            the existing notes list when relevant, otherwise new concept titles worth creating.";

        let prompt = format!(
            "Existing notes in vault: {:?}\n\nCreate a concise, well-structured Obsidian note about: \"{}\".",
            existing, topic
        );

        let resp = self
            .llm
            .complete(CompletionRequest {
                model: self.model.clone(),
                system: Some(system.to_string()),
                prompt,
                temperature: 0.4,
            })
            .await?;

        let cleaned = resp.text.trim().trim_start_matches("```json").trim_end_matches("```").trim();
        let note: Note = serde_json::from_str(cleaned)
            .map_err(|e| anyhow::anyhow!("failed to parse model output as Note JSON: {e}\nraw: {cleaned}"))?;

        self.store.write_note(&note).await?;
        Ok(note)
    }
}
