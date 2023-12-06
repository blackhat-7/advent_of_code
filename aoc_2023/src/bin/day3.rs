#![allow(dead_code)]

#[path = "../utils.rs"]
mod utils;

fn main() {
    let input = utils::read_input_to_string(3);
    let res = solve(input);
    println!("{}", res);
}

fn solve(input: String) -> u32 {
    let input_vec: Vec<Vec<char>> = input
        .split('\n')
        .filter_map(|line| {
            if line.is_empty() {
                return None
            }
            Some(line.chars().collect())
        })
        .collect();

    input_vec.iter().enumerate().map(|(i, line)| {
        
        let mut add_num = false;
        let mut num = String::from("");
        let mut acc = line.iter().enumerate().fold(0, |mut acc, (j, c)| {
            if c.is_ascii_digit() {
                num.push(*c);
                if has_special_around(&input_vec, i, j) {
                    add_num = true;   
                }
            } else if *c == '.' {
                if add_num {
                    acc += num.parse::<u32>().unwrap();
                }
                num = "".to_string();
                add_num = false;
            }
            acc
        });
        if add_num {
            dbg!(&num);
            acc += num.parse::<u32>().unwrap();
        }
        acc
    })
    .sum()
}

fn has_special_around(input_vec: &[Vec<char>], i: usize, j: usize) -> bool {
    let (mut top_left, mut top, mut top_right, mut right, mut bottom_right, mut bottom, mut bottom_left, mut left) =
        (false, false, false, false, false, false, false, false);
    if i > 0 && j > 0 {
        top_left = is_special(&input_vec[i - 1][j - 1]);
    }
    if i > 0 {
        top = is_special(&input_vec[i - 1][j]);
    }
    if i > 0 && j < input_vec[0].len() - 1 {
        top_right = is_special(&input_vec[i - 1][j + 1]);
    }
    if j < input_vec[0].len() - 1 {
        right = is_special(&input_vec[i][j + 1]);
    }
    if i < input_vec.len() - 1 && j < input_vec[0].len() - 1 {
        bottom_right = is_special(&input_vec[i + 1][j + 1]);
    }
    if i < input_vec.len() - 1 {
        bottom = is_special(&input_vec[i+1][j]);
    }
    if i < input_vec.len() - 1 && j > 0 {
        bottom_left = is_special(&input_vec[i + 1][j - 1]);
    }
    if j > 0 {
        left = is_special(&input_vec[i][j - 1]);
    }
    top_left || top || top_right || right || bottom_right || bottom || bottom_left || left
}


fn is_special(c: &char) -> bool {
    !c.is_ascii_digit() && *c != '.'
}
