#![allow(dead_code)]

use std::collections::HashSet;
use std::time;

#[path = "../utils.rs"]
mod utils;

fn main() {
    let start = time::Instant::now();
    let input = utils::read_input_to_string(3);
    let res = solve2(input);
    println!("{}", res);
    println!("{:?}", time::Instant::now().duration_since(start));
}


#[derive(PartialEq, Eq, Debug, Clone)]
enum Symbol {
    Dot,
    Asterisk,
    Digit(u8),
    Other,
}


impl From<char> for Symbol {
    fn from(c: char) -> Self {
        match c {
            '.' => Symbol::Dot,
            '0'..='9' => Symbol::Digit(c as u8 - b'0'),
            '*' => Symbol::Asterisk,
            _ => Symbol::Other
        }
    }
}

fn solve(input: String) -> u64 {
    let input = input.to_string();
    let map = build_map(input);
    map.iter()
        .enumerate()
        .map(|(i, row)| {
            let mut cur_num = 0;
            let mut add_num = false;
            let mut sum = row.iter()
                .enumerate()
                .fold(0, |mut acc, (j, symbol)| {
                    match symbol {
                        Symbol::Digit(digit) => {
                            cur_num = cur_num * 10 + (*digit as u64);
                            if has_other_around(&map, i, j) {
                                add_num = true;
                            }
                        }
                        _ => {
                            if add_num {
                                acc += cur_num;
                                cur_num = 0;
                                add_num = false;
                            } else {
                                cur_num = 0
                            }
                        }
               }
                    acc
                });
            if add_num {
                println!("{cur_num}");
                sum += cur_num;
            }
            sum
        })
        .sum()
}



fn build_map(input: String) -> Vec<Vec<Symbol>> {
    input.lines()
        .filter_map(|line| {
            if line.is_empty() {
                return None
            }
            Some(line.chars().map(Symbol::from).collect())
        })
        .collect()
}


fn has_other_around(map: &Vec<Vec<Symbol>>, i: usize, j: usize) -> bool {
    if i > 0 {
        if map[i - 1][j] == Symbol::Other {
            return true;
        }
        if j > 0 && map[i - 1][j - 1] == Symbol::Other {
            return true;
        }
    }
    if j > 0 {
        if map[i][j - 1] == Symbol::Other {
            return true;
        }
        if i < map.len() - 1 && map[i + 1][j - 1] == Symbol::Other {
            return true;
        }
    }
    if i < map.len() - 1 {
        if map[i + 1][j] == Symbol::Other {
            return true;
        }
        if j < map[0].len() - 1 && map[i + 1][j + 1] == Symbol::Other {
            return true;
        }
    }
    if j < map[0].len() - 1 {
        if map[i][j + 1] == Symbol::Other {
            return true;
        }
        if i > 0 && map[i - 1][j + 1] == Symbol::Other {
            return true;
        }
    }
    false
}


struct NumIndexPair((usize, usize), (usize, usize));

struct NumLoc {
    num: u64,
    loc: (usize, usize),
}


fn solve2(input: String) -> u64 {
    let input = input.to_string();
    let map = build_map(input);
    map.iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter()
                .enumerate()
                .fold(0, |mut acc, (j, symbol)| {
                    if symbol == &Symbol::Asterisk {
                        if let Some(n) = get_product_if_pair(&map, i, j) {
                            acc += n;
                        }
                    }
                    acc
                })
        })
        .sum()
}


fn get_product_if_pair(map: &Vec<Vec<Symbol>>, i: usize, j: usize) -> Option<u64> {
    let mut nums = vec![];
    if i > 0 {
        if let Symbol::Digit(_) = map[i - 1][j] {
            nums.push(get_num(map, i - 1, j));
        }
        if let Symbol::Digit(_) = map[i - 1][j - 1] {
            nums.push(get_num(map, i - 1, j - 1));
        }
    }
    if j > 0 {
        if let Symbol::Digit(_) = map[i][j - 1] {
            nums.push(get_num(map, i, j - 1));
        }
        if let Symbol::Digit(_) = map[i + 1][j - 1] {
            nums.push(get_num(map, i + 1, j - 1));
        }
    }
    if i < map.len() - 1 {
        if let Symbol::Digit(_) = map[i + 1][j] {
            nums.push(get_num(map, i + 1, j));
        }
        if let Symbol::Digit(_) = map[i + 1][j + 1] {
            nums.push(get_num(map, i + 1, j + 1));
        }
    }
    if j < map[0].len() - 1 {
        if let Symbol::Digit(_) = map[i][j + 1] {
            nums.push(get_num(map, i, j + 1));
        }
        if let Symbol::Digit(_) = map[i - 1][j + 1] {
            nums.push(get_num(map, i - 1, j + 1));
        }
    }

    let nums = deduplicate_nums(nums);

    if nums.len() == 2 {
        Some(nums[0] * nums[1])
    } else {
        None
    }

}


fn get_num(map: &[Vec<Symbol>], end_i: usize, end_j: usize) -> NumLoc {
    let (i, mut j) = (end_i, end_j as i64);
    let mut num = 0u64;
    let mut times = 1;
    let mut j_start = 0usize;

    while j >= 0 {
        if let Symbol::Digit(n) = map[i][j as usize] {
            num += n as u64 * times;
            j_start = j as usize;
        } else {
            break
        }
        times *= 10;
        j -= 1;
    }

    j = end_j as i64 + 1;
    while j < map[0].len() as i64 {
        if let Symbol::Digit(n) = map[i][j as usize] {
            num = num * 10 + n as u64;
        } else {
            break
        }
        j += 1;
    }
    NumLoc {
        num,
        loc: (i, j_start),
    }
}

fn deduplicate_nums(nums: Vec<NumLoc>) -> Vec<u64> {
    let mut set = HashSet::new();
    let mut filtered_nums = vec![];
    nums.iter()
        .for_each(
            |NumLoc { num, loc }| {
                if !set.contains(loc) {
                    set.insert(loc);
                    filtered_nums.push(*num);
                }
            }
        );
    filtered_nums
}
