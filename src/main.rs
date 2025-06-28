// I think I need to do better error handling, and probably start working on other features. 
// I have wasted 3 days just to get the User system working (include sob emoji here)
// Rust really is hard for a beginner like me ;_;
use std::path::Path;
pub mod user;

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
    user::read_file();
}
