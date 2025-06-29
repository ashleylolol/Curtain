// I think I need to do better error handling, and probably start working on other features. 
// I have wasted 3 days just to get the User system working (include sob emoji here)
// Rust really is hard for a beginner like me ;_;
use std::path::Path;
use std::io;
pub mod user;
mod posts;
use crate::posts::Post;
fn main(){
    let dir_path = user::get_dir_path().to_string();
    let file_path = format!("{}/user.json", dir_path).to_string();
    if Path::new(&file_path).exists(){
        println!("Welcome Back!");
        let _user = user::read_file();
        println!("[{} {}]: {} ", _user.emoji, _user.username, _user.link);
    } else {
        println!("User.json does not exist. Please enter the information below: ");
        let _user = user::create_user_profile();
    }
    // user::read_file();
    let mut user_input: String = String::new();
    println!("Enter your post here: ");
    io::stdin().read_line(&mut user_input).expect("Some random goofy error occured.");
    let user_input = user_input.trim().to_string();
    let post: Post = posts::create_posts(user_input);
    println!("Post: {:?}", post);
}
