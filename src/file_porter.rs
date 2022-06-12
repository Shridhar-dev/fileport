use std::fs;
use ansi_term::Colour;

pub fn port_files(from:&String,to:&String,level:u8){

    let metadata = fs::metadata(from).unwrap();
    let file_type = metadata.file_type();
    
    //println!("{:?}",op);

    if file_type.is_file(){
        fs::copy(from, to).expect("Error copying files!");
    }
    else{
        
       
        match fs::create_dir_all(to){
            Ok(result) => result,
            Err(_) => println!("Well, there was some issue with the folder!")
        };

        let paths = fs::read_dir(from).unwrap();
        let filecount = paths.count();

        let mut count = filecount;
       
        for entry in fs::read_dir(from).unwrap() {

            let entry = &entry.unwrap();
            let file_name = entry.file_name().into_string().unwrap();  
            let path = entry.path();

         
            
            if level == 0 {
                print!("{esc}c", esc = 27 as char);
                println!("\nPorting......\n");
                println!("{} {:?}",Colour::Green.paint("Making file:").to_string(),path);
                count = count + 1;
                let percent = ((count-filecount) as f32 /filecount as f32 )*20 as f32;
                let roundp = percent.floor() as usize; 

                let repeated = " ".repeat(roundp);
                let repeated_dash = " ".repeat(20-roundp);
                println!("🚪{}🎈{}🚪 - {}/{}", repeated, repeated_dash, count-filecount, filecount);
          
            }

            

            if path.is_dir() {
                
                port_files(&path.into_os_string().into_string().unwrap(), &(to.to_owned() + "/" + &file_name),1);
             
            } else {
                fs::copy(path.into_os_string().into_string().unwrap(), &(to.to_owned() + "/" + &file_name))
                .expect("Error copying files!");
            }
        }
    }
}