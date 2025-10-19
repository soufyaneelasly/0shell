use std::fs;

pub fn cmd_ls(args: &[&str]) {
    let path = if args.is_empty() { "." } else { args[0] };

    match fs::read_dir(path) {
        Ok(entries) => {
            for entry in entries.flatten() {
                if let Ok(name) = entry.file_name().into_string() {
                    println!("{}", name);
                }
            }
        }
        Err(e) => eprintln!("ls: {}: {}", path, e),
    }
}