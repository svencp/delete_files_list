/*
    This is my program to delete files in a given text list. The list could be gotten like:
    plocate /path ? list.txt 
    
    2022.11.14      Sven Ponelat

    run it like: delete_files_list list.txt


*/


pub const RELEASE: bool            = false;
// pub const RELEASE: bool            = true;
pub const VERSION: &str            = env!("CARGO_PKG_VERSION");



fn main() {
    println!("Hello, world!");
}
