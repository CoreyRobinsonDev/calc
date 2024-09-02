mod terminal;
mod ui;
mod utils;

use utils::calc;

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
                let out = calc(&line);
                buf += &(out[3].to_owned() + "\n");
                ui.history(&buf);
                ui.equation(out);
            }
            Err(error) => println!("error: {error}"),
        };
    }
}

