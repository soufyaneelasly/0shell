use std::fs;
use std::path::Path;

pub fn cmd_rm(args: &[&str]) {

    println!("args :: {:?}",args);
    if args.len() < 1 {
        println!("rm: missing operand");
        return;
    }

    let mut recursive = false;
    let mut targets: Vec<&str> = Vec::new();

    for &arg in args {
        if arg == "-r" {
            recursive = true;
        } else {
            targets.push(arg);
        }
    }

    if targets.is_empty() {
        println!("rm: missing operand");
        return;
    }

    for &item in &targets {
        let path = Path::new(item);

        if !path.exists() {
            println!("rm: cannot remove '{}': No such file or directory", item);
            continue;
        }

        if path.is_file() {
            if let Err(e) = fs::remove_file(path) {
                println!("rm: cannot remove '{}': {}", item, e);
            }
        } else if path.is_dir() {
            if recursive {
                if let Err(e) = fs::remove_dir_all(path) {
                    println!("rm: cannot remove '{}': {}", item, e);
                }
            } else {
                println!("rm: cannot remove '{}': Is a directory", item);
            }
        } else {
            println!("rm: cannot remove '{}': Unknown file type", item);
        }
    }
}
