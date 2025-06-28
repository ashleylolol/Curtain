use std::io;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub emoji: char,
    pub link: String,
}
// I KID YOU NOT, I DIDN'T KNOW THAT THE VARIABLES INSIDE a struct had to public as well (includue
// sob emoji here) this is such a sad moment as I wasted 30 mins of my life debugging this small
// mistake of mine 

pub fn get_dir_path() -> std::string::String{
    let home: String = std::env::var("HOME").expect("Couldn't fetch your HOME directory variable");
    let fullpath = format!("{}/.curtain", home);
    fullpath
}
pub fn make_file(content: String){
    let path = get_dir_path().to_string();
    // println!("{}", path); WAS USED FOR DEBUGGING THIS SHITTY ERROR 
    std::fs::create_dir_all(&path).expect("Some error happened while creating the directory");
    let full_path = format!("{}/user.json", path); 
    std::fs::write(&full_path, content).expect("Some error occured while creating the file!");
}

pub fn read_file() -> User{
    let file_pathh = get_dir_path().to_string();
    let file_path = format!("{}/user.json", file_pathh);
    let content_in_json = std::fs::read_to_string(file_path).unwrap();
    let user: User = serde_json::from_str(&content_in_json).unwrap();
   //  println!("[{} {}]'s link is {}", user.emoji, user.username, user.link);
    user
    
}

pub fn create_user_profile(){
    let mut username: String = String::new();
    let mut emoji: String = String::new();
    let mut link: String = String::new();

    println!("Please enter your username, in lower caps.");
    io::stdin().read_line(&mut username).expect("Some error occured while reading your username.");
    println!("Please enter your emoji. ");
    io::stdin().read_line(&mut emoji).expect("Some error occured while reading your emoji.");
    println!("Please enter your Curtain link.");
    io::stdin().read_line(&mut link).expect("Some error occured while reading your link.");

    // Parsing their input.
    let emoji: char = emoji.trim().parse().expect("Some error occured while parsing your emoji. Please make sure that you have entered a valid emoji.");
    let username = username.trim().to_string();
    let link = link.trim().to_string();


    let user = User {
        username,
        emoji,
        link,
    };

    let to_json = serde_json::to_string_pretty(&user).expect("Failed to serialize");
    // println!("{}", get_file_path());
    make_file(to_json);
    println!("Done generating your file!")
}



