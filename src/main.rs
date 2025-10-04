use std::io::Write;



fn main() {
    println!("Hello, world!");
    loup_main()
}















pub fn loup_main() {
    loop {
    print!("$ ");
    std::io::stdout().flush().unwrap();
    println!("hhhh{:?}",std::io::stdout().flush().unwrap());

    let mut input = String::new();
    if std::io::stdin().read_line(&mut input).is_err() {
        break;
    }

    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    if parts.is_empty() { continue; }

    let cmd = parts[0];
    //let args = &parts[1..];
    println!("{}",cmd);

    match cmd {
        "ls" => cmd_ls(cmd),
        "cat" => cmd_cat(cmd),
        "exit" => break,
        _ => println!("Command '{}' not found", cmd),
    }
}

}

fn cmd_ls(s :&str){
    println!("hhh :{}",s);
    match s.is_empty() {
        false=>println!("comande ls"),
        true=>println!("khawilkhwa")
    }
}

fn cmd_cat(s :&str){
    match s.is_empty() {
        false=>println!("comande cat"),
        true=>println!("khawilkhwa")
    }
}


