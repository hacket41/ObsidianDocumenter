#[derive(Debug, Clone, Default)]
pub struct WorkingMemory {
    pub turns: Vec<(String, String)>, // (role, content)
}

impl WorkingMemory {
    pub fn push(&mut self, role: &str, content: &str) {
        self.turns.push((role.to_string(), content.to_string()));
    }

    pub fn as_context(&self) -> String {
        self.turns
            .iter()
            .map(|(r, c)| format!("{r}: {c}"))
            .collect::<Vec<_>>()
            .join("\n")
    }
}
