use std::fs;
use std::path::Path;

pub fn cmd_mkdir(dirs: &[&str]) {
    if dirs.is_empty() {
        eprintln!("mkdir: missing operand");
        return;
    }

    for dir in dirs {
        if let Err(e) = fs::create_dir(Path::new(dir)) {
            eprintln!("mkdir: cannot create directory '{}': {}", dir, e);
        }
    }
}
