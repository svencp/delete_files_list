/*
    This is my program to delete files in a given text list. The list could be gotten like:
    plocate /path ? list.txt 
    
    2022.11.14      Sven Ponelat

    run it like: delete_files_list list.txt


*/

mod library;

use library::my_utils::*;
use std::env;
use std::process::exit;
use std::path::Path;
use std::fs::*;
use std::io::{prelude::*, BufReader};




pub const VERSION: &str            = env!("CARGO_PKG_VERSION");



fn main() {
    let arguments: Vec<String> = env::args().collect();
    let path:Option<&Path>;
    let mut file_success = 0;
    let mut file_failed = 0;
    let mut folder_success = 0;
    let mut folder_failed = 0;

    // Check on arguments
    match arguments.len() {
        // no arguments
        1 => {
            let mut message = "No list file given!".to_string();
            feedback(Feedback::Error, message);
            println!();
            message = "something like:  delete_files /home/path/to/list.txt".to_string();
            feedback(Feedback::Info, message);
            exit(17);
        }
        2 => {
            match arguments[1].as_str() {
                "ver" | "-v" | "-ver" | "v" => {
                    let message = format!("Version is {}",VERSION);
                    feedback(Feedback::Info, message);
                    exit(17);
                }
                _ => {
                    // If file does not exist
                    path = Some(Path::new(&arguments[1]));
                    if ! path.unwrap().exists() {
                        let message = "File (or path) does not exist!".to_string();
                        feedback(Feedback::Error, message);
                        exit(17);
                    }
                }
            }
        }
        _ => {
            let mut message = "Too many arguments (parameters) given!".to_string();
            feedback(Feedback::Error, message);
            println!();
            message = "something like:  delete_files /home/path/to/list.txt".to_string();
            feedback(Feedback::Info, message);
            exit(17);
        }
    }
   
    let file = BufReader::new(File::open(path.unwrap()).unwrap());
    let mut lines: Vec<_> = file.lines().map(|line| { line.unwrap() }).collect();
    lines.reverse();

    
    // MAIN LOOP  -  DO ONLY FILES  
    for line in lines.iter() {
        match Path::new(&line).is_file() {
            true => {
                let res = delete_file(line.to_string());
                match res.is_ok() {
                    true => {
                        file_success += 1;
                    }
                    false => {
                        println!("{}",res.err().unwrap());
                        file_failed += 1;
                    }
                }
            }
            false => {
                continue;
            }
        }
    }
    
    for folder in lines.iter() {

        match Path::new(&folder).is_dir() {
            true => {
                let res = delete_folder(folder.to_string());
                match res.is_ok() {
                    true => {
                        folder_success += 1;
                    }
                    false => {
                        println!("{}",res.err().unwrap());
                        folder_failed += 1;
                    }
                }
            }
            false => {
                continue;
            }
        }
    }

    // Wrap up
    println!("");
    println!("Files Deleted:    {}",file_success);
    println!("Failed deletions: {}",file_failed);
    println!();
    println!("Folders Deleted:  {}",folder_success);
    println!("Failed deletions: {}",folder_failed);
    println!();
    println!("Done - {}",arguments[1]);
}


// ************************************************ Functions ***********************************************************
// function to delete file coming in as string
pub fn delete_file(str: String ) -> Result<(), String> {

    let deletion = remove_file(str.clone());
    match deletion.is_ok() {
        true => {
            Ok(())
        }
        false => {
            let message: String = format!("File: {} was not deleted!", str.clone());
            return Err(message);
        }
    }
}

// function to delete folder coming in as string
pub fn delete_folder(str: String ) -> Result<(), String> {

    let deletion = remove_dir(str.clone());
    match deletion.is_ok() {
        true => {
            Ok(())
        }
        false => {
            let message: String = format!("Folder: {} was not deleted!", str.clone());
            return Err(message);
        }
    }
}









