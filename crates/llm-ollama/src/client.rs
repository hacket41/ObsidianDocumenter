use agent_core::ports::{CompletionRequest, CompletionResponse, LlmClient};
use async_trait::async_trait;
use serde_json::json;

pub struct OllamaClient {
    base_url: String,
    http: reqwest::Client,
}

impl OllamaClient {
    pub fn new(base_url: impl Into<String>) -> Self {
        Self { base_url: base_url.into(), http: reqwest::Client::new() }
    }
}

#[async_trait]
impl LlmClient for OllamaClient {
    async fn complete(&self, req: CompletionRequest) -> anyhow::Result<CompletionResponse> {
        let body = json!({
            "model": req.model,
            "prompt": req.prompt,
            "system": req.system,
            "stream": false,
            "format": "json",
            "options": { "temperature": req.temperature }
        });

        let resp = self
            .http
            .post(format!("{}/api/generate", self.base_url))
            .json(&body)
            .send()
            .await?
            .error_for_status()?
            .json::<serde_json::Value>()
            .await?;

        Ok(CompletionResponse {
            text: resp["response"].as_str().unwrap_or_default().to_string(),
        })
    }

    async fn list_models(&self) -> anyhow::Result<Vec<String>> {
        let resp = self
            .http
            .get(format!("{}/api/tags", self.base_url))
            .send()
            .await?
            .error_for_status()?
            .json::<serde_json::Value>()
            .await?;

        Ok(resp["models"]
            .as_array()
            .unwrap_or(&vec![])
            .iter()
            .filter_map(|m| m["name"].as_str().map(String::from))
            .collect())
    }
}
