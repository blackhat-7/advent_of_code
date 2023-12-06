#![allow(dead_code)]

use std::fs;

fn main() {
    let input = fs::read_to_string("inputs/day8.txt").unwrap();
    let mut forest = Forest::new();
    forest.parse(input);
    println!("{}", forest.get_visible_trees_count());
}


#[derive(Debug, Clone)]
struct MaxHeights {
    left: i32,
    right: i32,
    up: i32,
    down: i32,
}
 impl MaxHeights {
    fn new() -> MaxHeights {
        MaxHeights {
            left: -1,
            right: -1,
            up: -1,
            down: -1,

        }
    }
}


struct Forest {
    max_heights: Vec<Vec<MaxHeights>>,
    heights: Vec<Vec<i32>>,
    visible_count: u32,
}

impl Forest {
    fn new() -> Forest {
        Forest {
            max_heights: Vec::new(),
            heights: Vec::new(),
            visible_count: 0,
        }
    }

    fn parse(&mut self, input: String) {
        for (i, line) in input.lines().enumerate() {
            let mut max_heights_row: Vec<MaxHeights> = Vec::new();
            let mut heights_row: Vec<i32> = Vec::new();
            for (j, tree) in line.chars().enumerate() {
                let height = tree.to_string().parse::<i32>().unwrap();
                let mut max_heights = MaxHeights::new();
                if i > 0 {
                    max_heights.up = std::cmp::max(self.max_heights[i-1][j].up, self.heights[i-1][j]);
                    if j == 3 {
                        println!("{:?}, {}", &self.max_heights[i-1][j].up, height);
                    }
                }
                if j > 0 {
                    max_heights.left = std::cmp::max(max_heights_row[j-1].left, heights_row[j-1]);
                }
                max_heights_row.push(max_heights);
                // println!("{}", height);
                heights_row.push(height);
                if i == 4 && j == 3 {
                    dbg!(height);
                }
            }
            self.max_heights.push(max_heights_row);
            self.heights.push(heights_row);
        }
        let (num_rows, num_columns) = (self.heights.len(), self.heights[0].len());

        for i in (0..num_rows).rev() {
            for j in (0..num_columns).rev() {
                let height = self.heights[i][j];
                if i < num_rows - 1 {
                    self.max_heights[i][j].down = std::cmp::max(self.max_heights[i+1][j].down, self.heights[i+1][j]);
                }
                if j < num_columns - 1 {
                    self.max_heights[i][j].right = std::cmp::max(self.max_heights[i][j+1].right, self.heights[i][j+1]);
                }
                if height > self.max_heights[i][j].left || height > self.max_heights[i][j].right || height > self.max_heights[i][j].up || height > self.max_heights[i][j].down {
                    self.visible_count += 1;
                }
            }
        }

        for i in 0..num_rows {
            for j in 0..num_columns {
                let height = self.heights[i][j];
                if height > self.max_heights[i][j].left || height > self.max_heights[i][j].right || height > self.max_heights[i][j].up || height > self.max_heights[i][j].down {
                    print!("{}", "T");
                } else {
                    print!("{}", self.heights[i][j]);
                }
            }
            println!()
        }

        // dbg!(&self.max_heights);
        // dbg!(&self.heights);
    }

    fn get_visible_trees_count(&self) -> u32 {
        self.visible_count
    }
}
