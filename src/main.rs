mod terminal;
mod ui;
mod utils;

use crate::terminal::*;

use std::io;

fn main() {
    screen::clear();
    screen::reset_cursor();
    let mut buf = String::new();
    let ui = ui::Ui::new();
    ui.init();

    loop {
        let mut line = String::new();
        match io::stdin().read_line(&mut line) {
            Ok(_) => {
                screen::clear();
                buf += &line;
                ui.history(&buf);
                ui.conversions(&line);
            }
            Err(error) => println!("error: {error}"),
        };
    }
}

