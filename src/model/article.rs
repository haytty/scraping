use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Article {
    title: String,
    published_at: String,
    url: String,
}

impl Article {
    pub fn new(title: String, published_at: String, url: String) -> Self {
        Self {
            title,
            published_at,
            url,
        }
    }
}