pub fn is_cursor_out_of_range(x: i16, y: i16) {
    if x < 0 || x >= <i16>::max_value() {
        panic!("Argument Out of Range Exception");
    }

    if y < 0 || y >= <i16>::max_value() {
        panic!("Argument Out of Range Exception");
    }
}
