
use std::collections::HashMap;
use std::fs;
use regex::Regex;

fn main() {
    let input = fs::read_to_string("inputs/day5.txt").expect("Failed to read input");
    let res = solve2(input);
    dbg!(res);
}


#[derive(Debug, Clone)]
struct Stacks {
    stacks: HashMap<u32, Vec<char>>,
    add_rgx: Regex,
    move_rgx: Regex
}

impl Stacks {
    fn new() -> Self {
        Stacks {
            stacks: HashMap::new(),
            add_rgx: Regex::new(r"([A-Z])").unwrap(),
            move_rgx: Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap()
        }
    }

    fn parse_and_add(&mut self, line: String) {
        let c = self.clone();
        let all_captures = c.add_rgx.captures_iter(&line);
        for captures in all_captures {
            if let Some(capture) = captures.get(1) {
                let letter = capture.as_str().parse::<char>().expect("Could not parse letter");
                let row = capture.start();
                self.add(&letter, row as u32);
            } 
        }
    }

    fn add(&mut self, letter: &char, row: u32) {
        // (x-1)/ 4
        let idx = (&row - 1) / 4;
        if let Some(stack) = self.stacks.get_mut(&idx) {
            stack.insert(0, *letter);
        } else {
            self.stacks.insert(idx, vec![*letter]);
        }
    }

    fn parse_and_move_boxes(&mut self, line: String, model: Option<&str>) {
        if let Some(captures) = self.move_rgx.captures(&line) {
            let count = captures[1].parse::<u32>().expect("Could not parse count of boxes");
            let from = captures[2].parse::<u32>().expect("Could not parse from box row");
            let to = captures[3].parse::<u32>().expect("Could not parse to box row");
            if model.is_some() {
                self.move_boxes_9001(count, from, to);
            } else {
                self.move_boxes(count, from, to);
            }
        } else {
            panic!("Could not parse move line")
        }
    }

    fn move_boxes(&mut self, count: u32, from: u32, to: u32) {
        for _ in 0..count {
            if let Some(popped_box) = self.stacks.get_mut(&(from - 1)).expect("No such row").pop() {
                if let Some(to_row) = self.stacks.get_mut(&(to - 1)) {
                    to_row.push(popped_box);
                } else {
                    panic!("No to row found")
                }
            } else {
                panic!("No boxes to pop");
            }
        }
    }

    fn move_boxes_9001(&mut self, count: u32, from: u32, to: u32) {
        let from_stack = self.stacks.get_mut(&(from - 1)).expect("No such row");
        let popped = from_stack.drain(from_stack.len() - count as usize..).collect::<Vec<char>>();
        let to_stack = self.stacks.get_mut(&(to - 1)).expect("No such row");
        to_stack.extend(popped);
    }


    fn top_view(&self) -> String {
        let mut tops: Vec<(u32, String)> = self.stacks
            .iter()
            .map(|(i, x)| {
                let y = x.clone().last().unwrap_or(&' ').to_owned();
                (i.to_owned(), String::from(y))
            })
            .collect();
        tops.sort_by_key(|x| x.0);
        tops.into_iter()
            .map(|x| {
                x.1
            })
            .collect::<Vec<String>>()
            .join("")
    }
}



fn solve(input: String) -> String {
    let mut stacks = Stacks::new();
    if let Some((struct_lines, move_lines)) = input.split_once("\n\n") {
        struct_lines
            .lines()
            .for_each(|line| stacks.parse_and_add(line.to_string()));
        move_lines
            .lines()
            .for_each(|line| stacks.parse_and_move_boxes(line.to_string(), None));
        stacks.top_view()
    } else {
        panic!("Could not split to struct and move lines");
    }
}


fn solve2(input: String) -> String {
    let mut stacks = Stacks::new();
    if let Some((struct_lines, move_lines)) = input.split_once("\n\n") {
        struct_lines
            .lines()
            .for_each(|line| stacks.parse_and_add(line.to_string()));
        move_lines
            .lines()
            .for_each(|line| stacks.parse_and_move_boxes(line.to_string(), Some("9001")));
        stacks.top_view()
    } else {
        panic!("Could not split to struct and move lines");
    }
}
