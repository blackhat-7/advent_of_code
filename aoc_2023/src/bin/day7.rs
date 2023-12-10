#![allow(dead_code)]

use std::collections::HashMap;

use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Card {
    A, 
    K, 
    Q, 
    T, 
    Nine, 
    Eight, 
    Seven, 
    Six, 
    Five, 
    Four, 
    Three, 
    Two,
    J
}


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
    cards: Vec<Card>,
    hand_type: HandType
}

impl From<char> for Card {
    fn from(c: char) -> Self {
        match c {
            'A' => Card::A,
            'K' => Card::K,
            'Q' => Card::Q,
            'T' => Card::T,
            '9' => Card::Nine,
            '8' => Card::Eight,
            '7' => Card::Seven,
            '6' => Card::Six,
            '5' => Card::Five,
            '4' => Card::Four,
            '3' => Card::Three,
            '2' => Card::Two,
            'J' => Card::J,
            _ => panic!("Invalid card")
        }
    }
}

impl Hand {
    pub fn new(cards: Vec<char>) -> Self {
        let hand_type = Hand::get_hand_type(&cards);
        let cards = cards.into_iter().map(Card::from).collect();
        Hand {
            cards,
            hand_type
        }
    }

    pub fn get_hand_type(cards: &[char]) -> HandType {
        let mut map = cards.iter().fold(HashMap::<char, u64>::new(), |mut acc, x| {
            *acc.entry(*x).or_insert(0) += 1;           
            acc
        });
        // add count of j to 2nd most common and remove j from map
        let non_j_highest_card = map.iter()
            .fold(None, |mut acc, (card, count)| {
                if *card != 'J' {
                    acc = std::cmp::max(acc, Some((*count, *card)));
                }
                acc
            });

        if let Some(non_j_highest_card) = non_j_highest_card {
            *map.entry(non_j_highest_card.1).or_default() += *map.get(&'J').unwrap_or(&0);
            map.remove(&'J');
        }

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
            HandType::TwoPair
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
                            order => Some(order.reverse())
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
        .filter_map(|line| {
            if line.is_empty() {
                return None
            }
            let (cards, num) = line.split_once(' ').unwrap();
            Some((Hand::new(cards.chars().collect::<Vec<_>>()), num.parse::<u64>().unwrap()))
        })
        .sorted_by(|a, b| a.0.partial_cmp(&b.0).unwrap())
        .enumerate()
        .inspect(|(i, (hand, num))| {
                println!("{}: {:?} {}", i + 1, hand, num)
            }
        )
        .map(|(i, (_, num))| {
            (i as u64 + 1) * num
        })
        .sum()
}



