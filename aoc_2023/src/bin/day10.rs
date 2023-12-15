#![allow(dead_code)]

use anyhow::Result;
use std::collections::VecDeque;
use std::time;



fn main() {
    let start = time::Instant::now();
    let mut input = include_str!("../../inputs/day10.txt").to_string();
    input = input[..input.len() - 1].to_string();
    let res = solve(&input).unwrap();
    println!("{res}");
    println!("Time: {:?}", start.elapsed());
}

/*
| is a vertical pipe connecting north and south.
- is a horizontal pipe connecting east and west.
L is a 90-degree bend connecting north and east.
J is a 90-degree bend connecting north and west.
7 is a 90-degree bend connecting south and west.
F is a 90-degree bend connecting south and east.
. is ground; there is no pipe in this tile.
S is the starting position of the animal.
*/

#[derive(PartialEq, Eq, Debug, Clone)]
enum Direction {
    Top,
    Down,
    Left,
    Right,
}

enum Immovable {
    Ground,
    Start,
}

#[derive(PartialEq, Eq, Debug)]
enum Pipe {
    Moveable(Direction, Direction),
    Ground,
    Start,
}

struct Map {
    map: Vec<Vec<Pipe>>,
    start_idx: (i64, i64),
}


#[derive(Debug, Clone)]
struct TraverseArgs {
    i: usize,
    j: usize,
    direction: Direction,
    steps: u64,
}

impl Direction {
    fn iterate() -> impl Iterator<Item = &'static Direction> {
        static DIRECTIONS: [Direction; 4] = [
            Direction::Top,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ];
        DIRECTIONS.iter()
    }

    fn reverse(&self) -> &'static Direction {
        match self {
            Direction::Top => &Direction::Down,
            Direction::Down => &Direction::Top,
            Direction::Left => &Direction::Right,
            Direction::Right => &Direction::Left,
        }
    }
}

impl From<&Direction> for (i64, i64) {
    fn from(d: &Direction) -> Self {
        match d {
            Direction::Top => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        }
    }
}

impl From<char> for Pipe {
    fn from(c: char) -> Self {
        match c {
            '|' => Self::Moveable(Direction::Top, Direction::Down),
            '-' => Self::Moveable(Direction::Left, Direction::Right),
            'L' => Self::Moveable(Direction::Top, Direction::Right),
            'J' => Self::Moveable(Direction::Top, Direction::Left),
            '7' => Self::Moveable(Direction::Left, Direction::Down),
            'F' => Self::Moveable(Direction::Right, Direction::Down),
            '.' => Self::Ground,
            'S' => Self::Start,
            _ => unreachable!(),
        }
    }
}

impl Pipe {
    fn get_move_idx(&self, to: &Direction) -> Option<(i64, i64)> {
        match self {
            Pipe::Moveable(dir1, dir2) => {
                if to == dir1 {
                    Some(dir2.into())
                } else {
                    Some(dir1.into())
                }
            }
            _ => None,
        }
    }
}

impl Map {
    fn move_direction(
        &self,
        i: i64,
        j: i64,
        prev_direction: &Direction,
        direction: &Direction,
    ) -> Result<(usize, usize)> {
        let (i_add, b_add) = direction.into();
        if i + i_add < 0
            || j + b_add < 0
            || (i + i_add) as usize >= self.map.len()
            || (j + b_add) as usize >= self.map[0].len()
        {
            return Err(anyhow::anyhow!("dead end"));
        }
        let (pi_add, pj_add) = prev_direction.into();
        if i_add + pi_add == 0 && b_add + pj_add == 0 {
            return Err(anyhow::anyhow!("dead end"));
        }
        Ok(((i + i_add) as usize, (j + b_add) as usize))
    }

    fn get_steps(&self, start_i: usize, start_j: usize) -> Result<u64> {
        let mut stack = VecDeque::new();
        stack.push_back(TraverseArgs {
            i: start_i, j: start_j, direction: Direction::Top, steps: 0
        });
        loop {
            let popped = stack.pop_back();
            if let Some(traverse_args) = popped {
                for args in self.traverse(traverse_args).iter() {
                    let pipe = &self.map[args.i][args.j];
                    if pipe == &Pipe::Start && args.steps != 0{
                        return Ok(args.steps);
                    }
                    stack.push_back(args.clone());
                }
            } else {
                return Err(anyhow::anyhow!("dead end"));
            }
        }
    }


    fn traverse(&self, traverse_args: TraverseArgs) -> Vec<TraverseArgs> {
        let TraverseArgs { i, j, direction, steps } = traverse_args;
        match &self.map[i][j] {
            Pipe::Start => {
                if steps != 0 {
                    unreachable!()
                } else {
                    let mut next_steps = vec![];
                    for dir in Direction::iterate() {
                        if let Ok((some_i, some_j)) =
                            self.move_direction(i as i64, j as i64, &direction, dir)
                        {
                            next_steps.push(TraverseArgs {
                                i: some_i, j: some_j, direction: dir.clone(), steps: steps + 1
                            })
                        }
                    }
                    next_steps
                }
            }
            Pipe::Ground => vec![],
            Pipe::Moveable(dir1, dir2) => {
                if dir1 != direction.reverse() && dir2 != direction.reverse() {
                    return vec![]
                }
                if let Ok((some_i, some_j)) =
                    self.move_direction(i as i64, j as i64, &direction, dir1)
                {
                    return vec![TraverseArgs {
                        i: some_i, j: some_j, direction: dir1.clone(), steps: steps + 1
                    }]
                }
                if let Ok((some_i, some_j)) =
                    self.move_direction(i as i64, j as i64, &direction, dir2)
                {
                    return vec![TraverseArgs {
                        i: some_i, j: some_j, direction: dir2.clone(), steps: steps + 1
                    }]
                }
                vec![]
            }
        }
    }
}


fn solve(input: &str) -> Result<u64> {
    let map = build_map(input)?;
    let steps = map.get_steps(
        map.start_idx.0 as usize,
        map.start_idx.1 as usize,
    )?;
    Ok(steps / 2)
}

fn build_map(input: &str) -> Result<Map> {
    let (mut start_i, mut start_j) = (0, 0);
    let map = input
        .lines()
        .enumerate()
        .filter_map(|(i, l)| {
            if l.is_empty() {
                None
            } else {
                if l.contains('S') {
                    (start_i, start_j) = (i as i64, l.find('S')? as i64);
                }
                Some(l.chars().map(Pipe::from).collect::<Vec<Pipe>>())
            }
        })
        .collect();
    Ok(Map {
        map,
        start_idx: (start_i, start_j),
    })
}
