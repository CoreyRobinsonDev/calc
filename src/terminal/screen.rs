
pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT
}

pub fn clear() {
    use std::io::stdout;
    use std::io::Write;
    print!("\x1b[2J");
    stdout().flush().unwrap();
}

pub fn move_cursor(dir: Direction, amount: u8) {
    use std::io::stdout;
    use std::io::Write;
    let dir_char;

    match dir {
        Direction::UP => dir_char = 'A',
        Direction::DOWN => dir_char = 'B',
        Direction::LEFT => dir_char = 'D',
        Direction::RIGHT => dir_char = 'C',
    }
    print!("\x1b[{}{dir_char}", amount);
    stdout().flush().unwrap();
}

pub fn reset_cursor() {
    move_cursor(Direction::UP, u8::MAX);
    move_cursor(Direction::LEFT, u8::MAX);
}
