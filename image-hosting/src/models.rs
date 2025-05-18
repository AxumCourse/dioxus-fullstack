use serde::{Deserialize, Serialize};

#[cfg(not(feature = "server"))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Image {
    pub id: String,
    pub hash: String,
    pub file_path: String,
    pub file_size: i64,
}
#[cfg(feature = "server")]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, sqlx::FromRow)]
pub struct Image {
    pub id: String,
    pub hash: String,
    pub file_path: String,
    pub file_size: i64,
}
