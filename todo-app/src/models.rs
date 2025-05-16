use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Todo {
    pub id: String,
    pub title: String,
    pub is_done: bool,
}

#[cfg(feature = "server")]
impl Todo {
    pub fn new(title: impl Into<String>) -> Self {
        let id = xid::new().to_string();
        Self {
            id,
            title: title.into(),
            is_done: false,
        }
    }
}
