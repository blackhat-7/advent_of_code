#![allow(dead_code)]

#[path = "../utils.rs"]
mod utils;

fn main() {
    let input = utils::read_input_to_string(2);
    let res = solve2(input);
    dbg!(res);
}

fn solve(input: String) -> u32 {
    let max_cubeset = CubeSet {
        red: 12,
        green: 13,
        blue: 14,
    };
    input
        .split('\n')
        .filter_map(|line| {
            if line.is_empty() {
                return None
            }
            let game = get_game(line.to_string());
            let cube_sets = get_sets(line.to_string());
            if is_game_possible(cube_sets, &max_cubeset) {
                Some(game)
            } else {
                None
            }
        })
        .sum()
}

fn solve2(input: String) -> u32 {
    input
        .split('\n')
        .filter_map(|line| {
            if line.is_empty() {
                return None
            }
            let cube_sets = get_sets(line.to_string());
            let (max_red, max_green, max_blue) = cube_sets
                .iter()
                .fold((0, 0, 0), |mut acc, cs| {
                    acc.0 = std::cmp::max(acc.0, cs.red);
                    acc.1 = std::cmp::max(acc.1, cs.green);
                    acc.2 = std::cmp::max(acc.2, cs.blue);
                    acc
                });
            Some(max_red * max_green * max_blue)
        })
        .sum()
}

fn get_game(inp: String) -> u32 {
    inp[inp.find(' ').unwrap() + 1usize..inp.find(':').unwrap()]
        .parse::<u32>()
        .unwrap()
}

#[derive(Debug)]
struct CubeSet {
    red: u32,
    green: u32,
    blue: u32,
}

fn get_sets(inp: String) -> Vec<CubeSet> {
    let inp = &inp[inp.find(':').unwrap() + 1usize..];
    inp.split(';')
        .map(|str_set| {
            let mut cubeset = CubeSet{red: 0, green: 0, blue: 0};
            str_set.trim()
                .split(',')
                .for_each(|str_cube| {
                    match str_cube.trim().split_once(' ') {
                        Some((num, "red")) => cubeset.red = num.parse::<u32>().unwrap(),
                        Some((num, "green")) => cubeset.green = num.parse::<u32>().unwrap(),
                        Some((num, "blue")) => cubeset.blue = num.parse::<u32>().unwrap(),
                        x => println!("Skipping {:?}", x)
                    }
                });
            cubeset
        })
        .collect()
}

fn is_game_possible(cube_sets: Vec<CubeSet>, max_cubeset: &CubeSet) -> bool {
    cube_sets.iter()
        .all(|cs| {
            // &dbg!(cs);
            cs.red <= max_cubeset.red && cs.green <= max_cubeset.green && cs.blue <= max_cubeset.blue
        })
}
