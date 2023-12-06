use std::collections::{HashMap, VecDeque};
use std::fs;


fn main() {
    let input = fs::read_to_string("inputs/day6.txt").expect("failed to read input");
    let res = solve(input);
    dbg!(res);
}



fn solve(input: String) -> u32 {
    const DISTINCT_CHARS: usize = 14;
    let mut visited = TimedHashMap::new(DISTINCT_CHARS);
    let mut count = 0;
    let mut res = 0;
    input.chars()
        .for_each(|x| {
            count += 1;
            visited.add(x);
            // println!("{} {}", x, count);
            if res == 0 && visited.map.len() == DISTINCT_CHARS {
                res = count;
            }
        });
    res
}


#[derive(Debug)]
struct TimedHashMap {
    map: HashMap<char, u32>,
    chunk_len: usize,
    queue: VecDeque<char>
}

impl TimedHashMap {
    fn new(chunk_len: usize) -> Self {
        TimedHashMap{
            map: HashMap::new(),
            chunk_len,
            queue: VecDeque::new()
        }
    }

    fn add(&mut self, c: char) {
        self.map.insert(c, self.map.get(&c).unwrap_or(&0) + 1);
        self.queue.push_back(c);
        if self.queue.len() > self.chunk_len {
            let popped = self.queue.pop_front().unwrap();
            self.map.insert(popped, self.map.get(&popped).unwrap() - 1);
            if *self.map.get(&popped).unwrap() == 0 {
                self.map.remove_entry(&popped).unwrap();
            }
        }
    }
}
