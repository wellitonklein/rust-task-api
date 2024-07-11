use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTaskSchema {
    pub title: String,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FilterOptions {
    pub page: Option<usize>,
    pub limit: Option<usize>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateTaskSchema {
    pub title: Option<String>,
    pub content: Option<String>,
}
