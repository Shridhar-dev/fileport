use std::env;
use std::fs;

mod file_porter;

fn main() {
    println!("Starting up!");

    let args: Vec<String> = env::args().collect();

    file_porter::port_files(&args[1],&args[2]);
    
    match fs::remove_dir_all(&args[1]){
        Ok(result) => result,
        Err(_) => println!("Well, there was some issue deleting the folder!")
    };
    
    println!("Done Porting 🚀!");
}
