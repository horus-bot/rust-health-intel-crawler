use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Article {
    pub title: String,
    pub link: String,
    pub published: Option<String>,
}