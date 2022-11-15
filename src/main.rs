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
use std::io::{self, prelude::*, BufReader};




pub const RELEASE: bool            = false;
// pub const RELEASE: bool            = true;
pub const VERSION: &str            = env!("CARGO_PKG_VERSION");


fn main() {
    let arguments: Vec<String>    = env::args().collect();
    let mut line_counter = 0;
    let mut success = 0;
    let mut failed = 0;

    // If no argument given
    if arguments.len() < 2 {
        let message = "No list file given!".to_string();
        feedback(Feedback::Error, message);
        exit(17);
    }

    // If file does not exist
    let path = Path::new(&arguments[1]);
    if ! path.exists() {
        let message = "File (or path) does not exist!".to_string();
        feedback(Feedback::Error, message);
        exit(17);
    }
    
    
    // MAIN LOOP
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    
    for line in reader.lines() {
        line_counter += 1;
        
        if line.is_err() {
            println!("Error in line {}",line_counter);
            exit(17);

        }

        let res = delete_file(line.unwrap());
        match res.is_ok() {
            true => {
                success += 1;
            }
            false => {
                println!("{}",res.err().unwrap());
                failed += 1;
            }
        }


    }







  





    println!("{}",arguments[1]);
}


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










