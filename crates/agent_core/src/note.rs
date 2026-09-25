use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Note {
    pub title: String,
    pub content: String,
    pub tags: Vec<String>,
    pub links: Vec<String>,
}

impl Note {
    pub fn to_markdown(&self) -> String {
        let frontmatter = format!("---\ntags: [{}]\n---\n\n", self.tags.join(", "));
        let links = if self.links.is_empty() {
            String::new()
        } else {
            let list = self
                .links
                .iter()
                .map(|l| format!("- [[{}]]", l))
                .collect::<Vec<_>>()
                .join("\n");
            format!("\n\n## Related\n{list}\n")
        };
        format!("{frontmatter}{}{links}", self.content)
    }
}
