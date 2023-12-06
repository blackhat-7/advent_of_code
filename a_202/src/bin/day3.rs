#![feature(iter_array_chunks)]
use std::collections::{HashSet, HashMap};
use std::fs;
use std::char;



fn main() {
    let input = fs::read_to_string("inputs/day3.txt").expect("Failed to read input");
    let res = solve2(input);
    dbg!(res);
}



fn solve(input: String) -> u32 {
    let mut char_scores = HashMap::new();
    for i in 1..27 {
        let c = char::from_u32(i+64).unwrap();
        char_scores.insert(c, i+26);
        let c = char::from_u32(i+96).unwrap();
        char_scores.insert(c, i);
    }
    input.split('\n')
        .fold(0u32, |acc, x| {
            let length = x.len();
            let (a, b) = x.split_at(length/2);
            let mut b_set = HashSet::new();
            for i in b.chars() {
                b_set.insert(i);
            }
            let mut score = 0;
            for i in a.chars() {
                if b_set.contains(&i) {
                    score += char_scores.get(&i).unwrap();
                    b_set.remove(&i);
                }
            }
            acc + score
        })
}


fn solve2(input: String) -> u32 {
    let mut char_scores = HashMap::new();
    for i in 1..27 {
        let c = char::from_u32(i+64).unwrap();
        char_scores.insert(c, i+26);
        let c = char::from_u32(i+96).unwrap();
        char_scores.insert(c, i);
    }
    input.split('\n')
        .array_chunks::<3>()
        .map(|chunk| {
            let mut chunk_char_count = HashMap::<char, i32>::new();
            chunk.iter()
                .for_each(|line| {
                    let mut visited = HashSet::<char>::new();
                    line.chars()
                        .for_each(|c| {
                            if !visited.contains(&c) {
                                chunk_char_count.insert(c, chunk_char_count.get(&c).unwrap_or(&0) + 1);
                            }
                        })
                });
            chunk_char_count.iter()
                .find_map(|(k, v)| {
                    if *v == 3 {
                        Some(char_scores.get(&k).unwrap())
                    } else {
                        None
                    }
                    
               })
                    .unwrap()
        })
        .sum::<u32>();
    0
}
