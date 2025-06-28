use std::path::Path;
mod user;

fn main(){
    let file_path = user::get_file_path();
    if Path::new(&file_path).exists(){
        println!("File path already exists! ");
    } else {
        println!("User.json does not exist. Please enter the information below: ");
        let user = user::create_user_profile();
    }
}
