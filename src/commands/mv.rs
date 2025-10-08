

use std::fs;
use std::path::Path;

pub fn cmd_mv(s :&[&str]){

    if s.len() < 2 {
        println!("mv: missing file operand");
        return 
    }

    let path = Path::new(s.last().unwrap());


     if path.exists() {

        if path.is_dir() {

            for file in &s[..s.len()-1] {
            let destination = path.join(file); 
            match fs::rename(file, destination.clone()) {
                Ok(_) => println!("Moved '{}' to '{}'", file, destination.display()),
                Err(e) => eprintln!("mv: cannot move '{}': {}", file, e),
            }
            }
        }

     }

}