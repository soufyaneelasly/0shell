use std::fs;
use std::path::Path;

pub fn cmd_mkdir(args: &[&str])   {
    if args.is_empty() {
        println!("mkdir: missing operand");
    }
    
    for dir in args {
        let path = Path::new(dir);
        if let Err(e) = fs::create_dir(path) {
            println!("mkdir: cannot create directory '{}': {}!", dir, e);
        }
    }
}
