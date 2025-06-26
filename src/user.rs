use std::io;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
struct User {
    username: String,
    emoji: char,
    link: String,
}

pub fn get_file_path() -> std::string::String{
    let mut home: String = std::env::var("HOME").expect("Couldn't fetch your HOME directory variable");
    let fullpath = format!("{}/.curtain", home);
    fullpath
}


pub fn user(){
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
    println!("{}", get_file_path());
    make_file(to_json);
    println!("Done generating your file!")
}

pub fn make_file(content: String){
    let path = get_file_path();
    std::fs::create_dir_all(&path).expect("Some error happened while creating the directory");
    let files_path = format!("{}/user.json", path);
    std::fs::write(&files_path, content).expect("Some error occured while creating the file!");
}

