use crossterm_input::input;

fn read_char() {
    let input = input();

    match input.read_char() {
        Ok(s) => println!("char typed: {}", s),
        Err(e) => println!("char error : {}", e),
    }
}

fn read_line() {
    let input = input();

    match input.read_line() {
        Ok(s) => println!("string typed: {}", s),
        Err(e) => println!("error: {}", e),
    }
}

fn main() {
    // un-comment below and run with
    // `cargo run --example input`:
    read_char();
    read_line();
}
