#![allow(unused_variables)]
use core::panic;
use std::{
    fs::{self, File},
    io::{self, ErrorKind, Read},
};

fn main() {
    // Question mark operator on option
    let last_char_option = last_char_of_first_line("Lots of text");
    dbg!(&last_char_option);

    // Use the std lib
    let username_result = fs_read_username_from_file();
    dbg!(&username_result);

    //Even better implmentation of propigated error
    let username_result = even_better_read_username_from_file();
    dbg!(&username_result);

    // Better implementation of propigated error
    let username_result = better_read_username_from_file();
    dbg!(&username_result);

    // Propigated Error
    let username_result = read_username_from_file();
    dbg!(&username_result);

    // Handled Error
    let greeting_file_result = File::open("hello.txt");
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };

    //Shortcuts
    let greeting_file = File::open("hello2.txt").unwrap();
    // or
    let greeting_file =
        File::open("hello2.txt").expect("hello2.txt should be included in this project.");

    // Unimplemented panic
    let v = vec![1, 2, 3];
    v[99];

    // Implemented panic
    panic!("crash and burn");
}

// Error propigation
fn read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = match File::open("hello.txt") {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

// Question mark operator
fn better_read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

// Question mark operator chaining
fn even_better_read_username_from_file() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

// Just use the std lib dummy
fn fs_read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

// Question mark operator on option
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
