use std::fs;
use std::path::Path;

pub fn cmd_mkdir(dirs: &[&str])   {
    if dirs.is_empty() {
        println!("mkdir: missing operand");
    }
    for dir in dirs {
        let path = Path::new(dir);
        if let Err(e) = fs::create_dir(path) {
            println!("mkdir: cannot create directory '{}': {}!", dir, e);
        }
    }
}
