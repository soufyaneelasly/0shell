use std::env;
use std::path::Path;

pub fn cmd_cd(args: &[&str]) {
    let target = if args.is_empty() {
        env::var("HOME")
            .or_else(|_| env::var("USERPROFILE"))
            .unwrap_or_else(|_| {
                eprintln!("cd: HOME not set");
                return String::new();
            })
    } else if args.len() > 1 {
        eprintln!("cd: too many arguments");
        return;
    } else {
        args[0].to_string()
    };

    if let Err(e) = env::set_current_dir(Path::new(&target)) {
        eprintln!("cd: {}: {}", target, e);
    }
}
