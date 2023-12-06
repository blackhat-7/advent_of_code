use std::fs;
use regex::Regex;

fn main() {
    let input = fs::read_to_string("inputs/day4.txt").expect("Failed to read input");
    let res = solve2(input);
    dbg!(res);
}


fn solve(input: String) -> i32{
    let reg = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();
    input
        .lines()
        .map(|line| {
            if let Some(captures) = reg.captures(line) {
                let a1 = captures[1].parse::<i32>().unwrap();
                let b1 = captures[2].parse::<i32>().unwrap();
                let a2 = captures[3].parse::<i32>().unwrap();
                let b2 = captures[4].parse::<i32>().unwrap();
                if (a1 <= a2 && b1 >= b2) | (a2 <= a1 && b2 >= b1) {
                    1
                } else {
                    0
                }
            } else {
                dbg!("Error parsing line");
                0
            }
        })
        .sum()
}


fn solve2(input: String) -> i32 {
    let reg = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();
    input
        .lines()
        .map(|line| {
            if let Some(captures) = reg.captures(line) {
                let a1 = captures[1].parse::<i32>().unwrap();
                let b1 = captures[2].parse::<i32>().unwrap();
                let a2 = captures[3].parse::<i32>().unwrap();
                let b2 = captures[4].parse::<i32>().unwrap();
                // [a1 - b1] [a2 - b2]
                if (a1 <= a2 && a2 <= b1) | (a2 <= a1 && a1 <= b2) {
                    1
                } else {
                    0
                }
            } else {
                dbg!("Error parsing line");
                0
            }
        })
        .sum()

}
