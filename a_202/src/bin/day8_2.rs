#![allow(dead_code)]

use std::fs;

fn main() {
    let input = fs::read_to_string("inputs/day8.txt").unwrap();
    let mut forest = Forest::new();
    forest.build_forest(input);
    // dbg!(&forest.trees);
    println!("{}", forest.find_best_scenic_score());
}


#[derive(Debug, Clone)]
struct Tree {
    height: i32,
    location: (i32, i32),
}

impl Tree {
    fn new(height: i32, location: (i32, i32)) -> Tree {
        Tree {
            height,
            location,
        }
    }
}


struct Forest {
    trees: Vec<Vec<Tree>>,
    best_scenic_score: i32,
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Forest {
    fn new() -> Forest {
        Forest {
            trees: Vec::new(),
            best_scenic_score: 0,
        }
    }

    fn build_forest(&mut self, input: String) {
        let mut x = -1;
        self.trees = input.lines()
            .map(|line| {
                let mut y = -1;
                x += 1;
                line.chars()
                    .map(|t| {
                        y += 1;
                        let height = t.to_string().parse::<i32>().unwrap();
                        let location = (x, y);
                        Tree::new(height, location)
                    })
                    .collect::<Vec<Tree>>()
            })
            .collect::<Vec<Vec<Tree>>>()
    }

    fn find_best_scenic_score(&self) -> i32 {
        self.trees.iter()
            .map(|row| {
                row.iter()
                    .map(|tree| {
                        self.get_tree_scenic_score(tree)
                    })
                    .max()
                    .unwrap()
            })
            .max()
            .unwrap()
    }

    fn get_tree_scenic_score(&self, tree: &Tree) -> i32 {
        let mut score = 1;
        for direction in [Direction::Up, Direction::Down, Direction::Left, Direction::Right] {
            score *= self.get_tree_scenic_score_direction(tree, direction);
        }
        score
    }

    fn get_tree_scenic_score_direction(&self, tree: &Tree, direction: Direction) -> i32 {
        let increment = match direction {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        };
        let mut cur_tree = tree;
        let mut score = 0;
        while cur_tree.location == tree.location || cur_tree.height < tree.height {
            let (x, y) = (cur_tree.location.0 + increment.0, cur_tree.location.1 + increment.1);
            if x < 0 || x >= self.trees.len() as i32 || y < 0 || y >= self.trees[0].len() as i32 {
                break;
            }
            score += 1;
            cur_tree = &self.trees[x as usize][y as usize];
        }
        score
    }
}
