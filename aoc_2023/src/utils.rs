#![allow(dead_code)]

use std::fs;

pub fn read_input_to_string(day: u32) -> String {
    let path = format!("inputs/day{}.txt", day);
    fs::read_to_string(&path).unwrap_or_else(|_| panic!("Could not find {}", path))
}
