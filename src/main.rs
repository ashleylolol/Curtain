use std::path::Path;
mod user;

fn main(){
    let dir_path = user::get_file_path().to_string();
    let file_path = format!("{}/user.json", dir_path).to_string();
    if Path::new(&file_path).exists(){
        println!("Welcome Back!");
    } else {
        println!("User.json does not exist. Please enter the information below: ");
        let _user = user::create_user_profile();
    }
    user::read_file();
}
