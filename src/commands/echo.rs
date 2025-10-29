pub fn cmd_echo(args: &[&str]) {
    if args.is_empty() {
        println!();
    } else {
        println!("{}", args.join(" "));
    }
}