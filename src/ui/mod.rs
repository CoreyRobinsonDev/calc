use std::collections::VecDeque;

use crate::terminal::color::*;
use crate::terminal::*; 
use crate::utils::*;

impl Ui {
    pub fn new() -> Self {
        Self { 
            history: (9, 10),
            equation: (9, 1),
        }
    }

    pub fn init(&self) {
        let mut list = Vec::<String>::new();

        // history
        for _ in 0..self.history.1 {
            list.push(format!("{:#b} {:#x} 0", 0, 0));
        }

        println!("{}", red(box_(list.join("\n"))));
        println!("{}", cyan(box_(
            format!("{:#b} {:#x} 0 + {:#b} {:#x} 0\n", 
                0, 0, 0, 0) +
            &format!("={:#b} {:#x} 0", 0,0)
        )));
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
        }

        if list.len() < height {
            for _ in 0..=(height-list.len()) {
                let padding: String = std::iter::repeat(' ')
                    .take(width)
                    .collect();
                list.push_front(padding);
            }
        } else if list.len() > height {
            for _ in 0..(list.len()-height) {
                list.pop_front();
            }
        }
        list.pop_back();
        let list = list
            .iter()
            .map(|num| {
                let conv: i128 = num.parse().unwrap_or(0);
                match conv {
                    -128..=127 => {
                        let conv: i8 = conv.try_into().unwrap();
                        return format!("{conv:#b} {conv:#x} {conv}");
                    },
                    -32768..=32767 => {
                        let conv: i16 = conv.try_into().unwrap();
                        return format!("{conv:#b} {conv:#x} {conv}");
                    },
                    -2147483648..=2147483647 => {
                        let conv: i32 = conv.try_into().unwrap();
                        return format!("{conv:#b} {conv:#x} {conv}");
                    },
                    -9223372036854775808..=9223372036854775807 => {
                        let conv: i64 = conv.try_into().unwrap();
                        return format!("{conv:#b} {conv:#x} {conv}");
                    }
                    _ => {
                        return format!("{conv:#b} {conv:#x} {conv}");
                    }
                }
            }) 
            .collect::<Vec<String>>()
            .join("\n");

        println!("{}", red(box_(list)));
    }

    pub fn equation(&self, expression: Vec<String>) {
        let a: i32 = expression[0].parse().unwrap_or(0);
        let b: i32 = expression[2].parse().unwrap_or(0);
        let op = &expression[1];
        let eq: i32 = expression[3].parse().unwrap_or(0);
        println!("{}", cyan(box_(
            format!("{a:#b} {a:#x} {} {} {b:#b} {b:#x} {}\n", 
                a, op, b) +
            &format!("={eq:#b} {eq:#x} {}", eq)
        )));
    }
}


pub struct Ui {
    // (width, height)
    history: (usize, usize),
    equation: (usize, usize)
}

