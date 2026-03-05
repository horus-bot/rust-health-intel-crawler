use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Article {
    pub title: String,
    pub link: String,
    pub published: Option<String>,
    pub content: Option<String>,
}