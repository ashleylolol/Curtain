use serde::{Deserialize, Serialize};
use chrono::Utc;
#[derive(Deserialize, Serialize, Debug)]
pub struct Post{
    pub content: String,
    pub timestamp: chrono::DateTime<Utc>,
}

pub fn create_posts(content: String) -> Post{
    Post{
        content,
        timestamp: Utc::now(),
    }
}
