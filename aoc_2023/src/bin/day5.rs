#![allow(dead_code)]

use rayon::prelude::*;
use std::collections::HashMap;
use std::time;

use anyhow::Result;


#[derive(Debug, Eq, PartialEq, Hash)]
struct TypeRange(u64, u64);



struct MRange {
    start: u64,
    end: u64,
}


struct EntityMapping {
    mapping: HashMap<MRange, MRange>,
}


impl MRange {
    
}


fn main() {
    let input = include_str!("../../inputs/day5.txt");
    let start = time::Instant::now();
    let res = solve(input).unwrap();
    println!("Elapsed: {:.2?}", start.elapsed());
    println!("{}", res)
}

fn solve(input: &str) -> Result<u64> {
    let line1 = input.split("\n\n").next().unwrap();
    let container = get_seeds(line1)?;
    // let container = line1[line1.find(':').unwrap() + 2..]
    //     .split(' ')
    //     .map(|n| n.parse::<u64>().unwrap())
    //     .collect::<Vec<_>>();
    let container = get_next_type(
        &fill_map(input.split("\n\n").nth(1).unwrap()).unwrap(),
        container,
    )
    .unwrap();
    Ok(*input
        .split("\n\n")
        .skip(2)
        .map(|mapping_string| fill_map(mapping_string).unwrap())
        .fold(container, |container, mapping| {
            get_next_type(&mapping, container).unwrap()
        })
        .iter()
        .min()
        .unwrap())
}

fn get_next_type(mapping: &HashMap<TypeRange, TypeRange>, types1s: Vec<u64>) -> Result<Vec<u64>> {
    Ok(types1s
        .iter()
        .map(|t1| {
            mapping
                .iter()
                .find_map(|(t1_range, t2_range)| {
                    if t1_range.0 <= *t1 && *t1 < t1_range.1 {
                        Some(t2_range.0 + (t1 - t1_range.0))
                    } else {
                        None
                    }
                })
                .unwrap_or(*t1)
        })
        .collect::<Vec<u64>>())
}

fn fill_map(input: &str) -> Result<HashMap<TypeRange, TypeRange>> {
    let mapping = HashMap::from_iter(input.split('\n').skip(1).filter_map(|line| {
        if line.is_empty() {
            return None;
        }
        let nums = line
            .split(' ')
            .filter_map(|n| n.parse::<u64>().ok())
            .collect::<Vec<_>>();
        Some((
            TypeRange(nums[1], nums[1] + nums[2]),
            TypeRange(nums[0], nums[0] + nums[2]),
        ))
    }));
    Ok(mapping)
}

fn get_seeds(input: &str) -> Result<Vec<u64>> {
    let input = input.to_string();
    let input = &input[input.find(':').unwrap() + 2..];
    let res = input.split_whitespace().collect::<Vec<_>>();
    let res = res.chunks(2).collect::<Vec<_>>();
    Ok(res
        .iter()
        .flat_map(|pair| {
            let (first, second) = (
                pair.first().unwrap().parse::<u64>().unwrap(),
                pair.last().unwrap().parse::<u64>().unwrap(),
            );
            (first..first + second).collect::<Vec<_>>()
        })
        .collect::<Vec<_>>())
}
