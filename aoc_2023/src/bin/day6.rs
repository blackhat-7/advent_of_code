

pub fn main() {
    let input = include_str!("../../inputs/day6.txt");
    let res = solve(input);
    println!("{res}");
}


pub fn solve(input: &str) -> u64 {
    let mut input = input.split('\n');
    let time = get_field(input.next().unwrap());
    let distance = get_field(input.next().unwrap());
    dbg!(&time, &distance);
    get_possible_ways(time, distance)
}


pub fn get_field(line: &str) -> u64 {
    let line = line.to_string();
    line.chars()
        .fold(String::new(), |mut acc, c| {
            match c {
                ' ' => {}
                c => if c.is_ascii_digit() {
                    acc.push(c);
                }
            }
            acc
        })
        .parse::<u64>()
        .unwrap()
}


pub fn get_possible_ways(time: u64, distance: u64) -> u64 {
    /*
    * (t-x)*x - d >= 0
    * tx -x**2 -d >= 0
    * x**2 -tx + d <= 0
    * t +- root(t**2 - 4d) / 2
    */
    let (mut min, mut max) = (1, u64::MAX);
    (1..time).for_each(|x| {
            match min {
                1 => {
                    if (time - x)*x > distance {
                        min = x                   
                    }
                }
                _ => {
                    if (time - x)*x > distance {
                        max = x    
                    }
                }
            }
        });
    dbg!(min, max);
    max - min + 1
}


pub fn filter_possible_sols(time: u64, distance: u64, press: Vec<u64>) -> Vec<u64> {
    let time = time as i32;
    let distance = distance as i32;
    let press = press.iter().map(|x| *x as i32).collect::<Vec<_>>();
    press.into_iter()
        .filter_map(|x| {
            if x*x - time*x + distance <= 0 {
                // dbg!(x, x*x - time*x + distance);
                Some(x as u64)               
            } else {
                None
            }      
        })
        .collect()
}
