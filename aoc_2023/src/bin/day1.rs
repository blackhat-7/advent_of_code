#![allow(dead_code)]

use std::collections::HashMap;


#[path = "../utils.rs"]
mod utils;


fn main() {
    let input = utils::read_input_to_string(1);
    let res = solve2(input);
    println!("{}", res);
}


fn solve(input: String) -> u32 {
    input.split('\n')
        .filter_map(|line| {
            let first = line.chars()
                .find_map(|c| {
                    if c.is_ascii_digit() {
                        c.to_digit(10)
                    } else {
                        None
                    }
                });
            let second = line.chars().rev()
                .find_map(|c| {
                   if c.is_ascii_digit() {
                        c.to_digit(10)
                    } else {
                        None
                    }
                });
            if let (Some(first), Some(second)) = (first, second) {
                Some(first * 10 + second)
            } else {
                None
            }
        })
        .sum()
}


fn solve2(input: String) -> u32 {
    input.split('\n')
        .filter_map(|line| {
            let res;
            let mut v = String::new();
            let first = line.chars()
                .find_map(|c| {
                    v.push(c);
                    get_digit(&v, c)
                });
            let mut v = String::new();
            let second = line.chars().rev()
                .find_map(|c| {
                    v.insert(0, c);
                    get_digit(&v, c)
                });
            if let (Some(first), Some(second)) = (first, second) {
                res = Some(first * 10 + second)
            } else {
                res = None
            }
            if let Some(res) = res {
                println!("{} {}", res, line);
            } else {
                println!("Nothing {}", line);
            }
            res
        })
        .sum()
}

fn get_digit(v: &str, c: char) -> Option<u32> {
    if c.is_ascii_digit() {
        return c.to_digit(10)
    }
    let digit_text = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);
    digit_text.iter()
        .find_map(|(text, num)|  {
            if v.contains(text) {
                Some(num.to_owned())
            } else {
                None
            }
        })
}
