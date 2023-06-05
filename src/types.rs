use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Clip {
    pub id: String,
    pub content: String,
    pub timestamp: String,
    pub starred: bool,
}
