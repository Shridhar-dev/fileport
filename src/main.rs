use std::env;
use std::fs;

mod file_porter;

fn main() {

    let args: Vec<String> = env::args().collect();

    let option = if args.len() > 3 {
        Some(&args[3])
    } else {
        None
    };

    file_porter::port_files(&args[1],&args[2],0);

    if option != None { 
        match fs::remove_dir_all(&args[1]){
            Ok(result) => result,
            Err(_) => println!("\nWell, there was some issue deleting the folder!\n")
        };
    }
    println!("\nDone Porting 🚀!\n");
}