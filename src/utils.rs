pub fn print_is_debug(cli_debug: &bool) {
    match cli_debug {
        true => println!("Debug mode is on"),
        false => println!("Debug mode is off")
    }
}

pub fn print_separator() {
    println!("========================================");
}