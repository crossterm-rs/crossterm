use crossterm::input;

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
    read_line();
    read_char();
}
