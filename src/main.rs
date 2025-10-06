use std::io::Write;
mod commands;


fn main() {
    loup_main()
}

pub fn loup_main() {
    loop {
    print!("$ ");
    std::io::stdout().flush().unwrap();

    let mut input = String::new();
    if std::io::stdin().read_line(&mut input).is_err() {
        break;
    }

    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    if parts.is_empty() { continue; }

    let cmd = parts[0];
    let args = &parts[1..];

    match cmd {
        "ls" =>   commands::cmd_ls(cmd),
        "cat" =>  commands::cmd_cat(cmd),
        "mkdir" => commands::cmd_mkdir(&args),
        "exit" => break,
        _ => println!("Command '{}' not found", cmd),
    }
}

}






