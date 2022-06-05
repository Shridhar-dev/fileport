use std::fs;
use ansi_term::Colour;

pub fn port_files(from:&String,to:&String){

    let metadata = fs::metadata(from).unwrap();
    let file_type = metadata.file_type();
    
    if file_type.is_file(){
        fs::copy(from, to).expect("Error copying files!");
    }
    else{
        println!("{} {:?}",Colour::Green.paint("Making file:").to_string(),to);

        match fs::create_dir_all(to){
            Ok(result) => result,
            Err(_) => println!("Well, there was some issue with the folder!")
        };

        for entry in fs::read_dir(from).unwrap() {

            let entry = &entry.unwrap();
            let file_name = entry.file_name().into_string().unwrap();  
            let path = entry.path();

            if path.is_dir() {
                port_files(&path.into_os_string().into_string().unwrap(), &(to.to_owned() + "/" + &file_name));
             
            } else {
                fs::copy(path.into_os_string().into_string().unwrap(), &(to.to_owned() + "/" + &file_name))
                .expect("Error copying files!");
            }
        }
    }
}