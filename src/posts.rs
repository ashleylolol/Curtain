use serde::{Deserialize, Serialize};
use chrono::Utc;
use crate::user;


#[derive(Deserialize, Serialize, Debug)]
pub struct Post{
    pub content: String,
    pub timestamp: chrono::DateTime<Utc>,
}


pub fn create_posts(content: String){
    let dir_path = user::get_dir_path();
    // println!("{}", dir_path);
    let file_path = format!("{}/posts.json");
}

