use std::collections::VecDeque;

use crate::terminal::color::*;
use crate::terminal::*; 
use crate::utils::*;

impl Ui {
    pub fn new() -> Self {
        Self { 
            history: (20, 10)
        }
    }

    pub fn init(&self) {
        let mut list = Vec::<String>::new();

        for _ in 0..self.history.1 {
            let padding: String = std::iter::repeat(' ')
                .take(self.history.0)
                .collect();
            list.push(padding);
        }
        
        println!("{}", red(box_(list.join("\n"))));
    }

    pub fn history(&self, list: impl Into<String>) {
        screen::reset_cursor();
        let list: String = list.into();
        let mut list: VecDeque<String> = list
            .split("\n")
            .map(|el| el.to_string())
            .collect();
        let width = self.history.0;
        let height = self.history.1;

        for item in list.iter_mut() {
            if item.len() > width {
                *item = item
                    .get(0..width).unwrap()
                    .to_string();
                continue;
            }
            for _ in 0..(width-item.len()) {
               *item += " "; 
            }
        }

        if list.len() < height {
            for _ in 0..=(height-list.len()) {
                let padding: String = std::iter::repeat(' ')
                    .take(width)
                    .collect();
                list.push_front(padding);
            }
        } else if list.len() >= height {
            for _ in 0..(list.len()-height) {
                list.pop_front();
            }
        }
        list.pop_back();
        let list = list
            .iter()
            .map(|el| el.to_owned())
            .collect::<Vec<String>>()
            .join("\n");

        println!("{}", red(box_(list)));
    }

    pub fn conversions(&self, num: i32) {
        println!("{}", cyan(box_(
            format!("{num}\n") +
            &format!("{num:#x}\n") +
            &format!("{num:#b}\n") 
        )));
    }
}


pub struct Ui {
    // (width, height)
    history: (usize, usize)
}

