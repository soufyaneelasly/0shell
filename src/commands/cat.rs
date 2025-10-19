use std::fs::File;
use std::io::{self, stdin, stdout, Read};
use std::path::Path;

pub fn cmd_cat(args: &[&str]) {
    if args.is_empty() {
        if let Err(e) = io::copy(&mut stdin(), &mut stdout()) {
            eprintln!("cat: {}", e);
        }
        return;
    }

    for file in args {
        if *file == "-" || *file == "--" {
            if let Err(e) = io::copy(&mut stdin(), &mut stdout()) {
                eprintln!("cat: {}", e);
            }
            continue;
        }

        let path = Path::new(file);
        let file_handle = match File::open(path) {
            Ok(f) => f,
            Err(_) => {
                eprintln!("cat: {}: No such file or directory", file);
                continue;
            }
        };

        let mut reader = file_handle;
        let mut buffer = Vec::new();
        if let Err(e) = reader.read_to_end(&mut buffer) {
            eprintln!("cat: {}", e);
            continue;
        }

        let content = String::from_utf8_lossy(&buffer);
        print!("{}", content);
    }
}
