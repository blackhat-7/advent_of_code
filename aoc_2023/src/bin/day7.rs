#![allow(dead_code)]

use std::collections::HashMap;

use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind
}



#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Hand {
    cards: Vec<char>,
    hand_type: HandType
}

impl Hand {
    pub fn new(cards: Vec<char>) -> Self {
        let hand_type = Hand::get_hand_type(&cards);
        Hand {
            cards,
            hand_type
        }
    }

    pub fn get_hand_type(cards: &[char]) -> HandType {
        let map: HashMap::<char, u64> = cards.iter().map(|x| (*x, 1)).collect();
        if map.values().any(|v| *v == 5) {
            HandType::FiveOfAKind
        } else if map.values().any(|v| *v == 4) {
            HandType::FourOfAKind
        } else if map.values().any(|v| *v == 3) {
            if map.values().any(|v| *v == 2) {
                HandType::FullHouse
            } else {
                HandType::ThreeOfAKind
            }
        } else if map.values().filter(|v| **v == 2).count() == 2 {
            HandType::HighCard
        } else if map.values().filter(|v| **v == 2).count() == 1 {
            HandType::OnePair   
        } else {
            HandType::HighCard
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.hand_type.cmp(&other.hand_type) {
            std::cmp::Ordering::Equal => {
                Some(self.cards.iter()
                    .zip(other.cards.iter())
                    .find_map(|(m, o)| {
                        match m.cmp(o) {
                            std::cmp::Ordering::Equal => None,
                            order => Some(order)
                        }
                    })
                    .unwrap())
            }
            same => Some(same)
        }
    }
}


pub fn main() {
    let input = include_str!("../../inputs/day7.txt");
    let res = solve(input.to_string()); 
    println!("{}", res);
}



pub fn solve(input: String) -> u64 {
    input.split('\n')
        .map(|line| {
            let (cards, num) = line.split_once(' ').unwrap();
            (Hand::new(cards.chars().collect::<Vec<_>>()), num.parse::<u64>().unwrap())
        })
        .sorted_by(|a, b| a.0.partial_cmp(&b.0).unwrap())
        .enumerate()
        .map(|(i, (_, num))| {
            (i as u64 + 1) * num
        })
        .sum()
}



