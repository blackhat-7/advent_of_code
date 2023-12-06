#![allow(dead_code)]
use std::collections::HashMap;

use anyhow::{Error, Result};

fn main() {
    let input = include_str!("../../inputs/day4.txt");
    let res = solve2(input).unwrap();
    println!("{}", res)
}

fn solve(input: &str) -> Result<i32> {
    Ok(input
        .to_string()
        .split('\n')
        .map(|line| {
            if line.is_empty() {
                return 0;
            }
            let line = &line.to_string()[line.find(':').unwrap() + 1..];
            let (winning, my) = line.split_once('|').unwrap();
            let (winning, my) = (
                winning
                    .trim()
                    .split(' ')
                    .filter(|w| !w.is_empty())
                    .map(|w| w.parse::<i32>().unwrap())
                    .collect::<Vec<_>>(),
                my.trim()
                    .split(' ')
                    .filter(|w| !w.is_empty())
                    .map(|w| w.parse::<i32>().unwrap())
                    .collect::<Vec<_>>(),
            );
            dbg!(&winning, &my);
            let count = winning.iter().filter(|w| my.contains(w)).count() as i32;
            2f32.powi(count - 1) as i32
        })
        .sum())
}

fn solve2(input: &str) -> Result<usize> {
    let mut copies = HashMap::new();
    input
        .to_string()
        .split('\n')
        .enumerate()
        .for_each(|(i, line)| {
            if line.is_empty() {
                return;
            }
            let line = &line.to_string()[line.find(':').unwrap() + 1..];
            let (winning, my) = line.split_once('|').unwrap();
            let (winning, my) = (
                winning
                    .trim()
                    .split(' ')
                    .filter(|w| !w.is_empty())
                    .map(|w| w.parse::<usize>().unwrap())
                    .collect::<Vec<_>>(),
                my.trim()
                    .split(' ')
                    .filter(|w| !w.is_empty())
                    .map(|w| w.parse::<usize>().unwrap())
                    .collect::<Vec<_>>(),
            );
            let count = winning.iter().filter(|w| my.contains(w)).count();
            let cur_card = i + 1;
            copies.insert(cur_card, copies.get(&cur_card).unwrap_or(&0) + 1);
            (1..=count).for_each(|c| {
                let card = i + 1 + c;
                let existing = copies.get(&(cur_card + c)).unwrap_or(&0);
                let to_add = copies.get(&cur_card).unwrap();
                copies.insert(card, existing + to_add);
            })
        });
    Ok(copies.values().sum())
}
