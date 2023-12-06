#![allow(dead_code)]
// use core::slice::SliceIndex;
// use std::rc::Rc;
//
// use anyhow::{Result, Error};

use core::usize;
use std::collections::HashMap;
use std::path::Path;

fn main() {
    let input = std::fs::read_to_string("inputs/day7.txt").unwrap();
    let res = solve(input);
    dbg!(res);
}

fn solve(input: String) -> usize {
    let mut cmd = CMDParser::new();
    input.lines().for_each(|line| {
        match line.get(..1) {
            // is a command
            Some("$") => match line.get(2..4) {
                Some(x) => {
                    if x == "cd" {
                        cmd.cd_dir(line.get(5..).unwrap().to_string());
                    }
                }
                _ => println!("wat 2"),
            },
            // result of ls (dir)
            Some("d") => {
                cmd.ls_dir(line.get(4..).unwrap().to_string());
            }
            // result of ls (file)
            _ => {
                cmd.add_size(line.split_once(' ').unwrap().0.parse::<usize>().unwrap());
            }
        }
    });

    cmd.calc_dir_size("/".to_string());
    // cmd.get_sum_size_of_dirs_atmost_size(100_000)
    cmd.min_size_to_clear()
}

#[derive(Debug)]
struct Dir {
    path: String,
    file_size: usize,
    children: Vec<String>,
}

impl Dir {
    fn new(path: String) -> Self {
        Self {
            path,
            file_size: 0,
            children: Vec::new(),
        }
    }
}

struct CMDParser {
    cur_dir: Option<String>,
    dir_map: HashMap<String, Dir>,
}

impl CMDParser {
    fn new() -> Self {
        Self {
            cur_dir: None,
            dir_map: HashMap::new(),
        }
    }

    fn add_size(&mut self, size: usize) {
        // dbg!(&self.dir_map);
        // dbg!(&self.cur_dir);
        let dir_name = self.cur_dir.take().unwrap();
        self.dir_map.get_mut(&dir_name).unwrap().file_size += size;
        self.cur_dir = Some(dir_name);
    }

    fn cd_dir(&mut self, name: String) {
        if name == "/" {
            self.cur_dir = Some(name.to_owned());
            self.dir_map
                .insert(name.to_owned(), Dir::new(name.to_owned()));
        } else if name == ".." {
            dbg!(&self.cur_dir);
            self.cur_dir = Some(
                Path::new(&self.cur_dir.take().unwrap())
                    .parent()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_owned(),
            );
            dbg!(&self.cur_dir);
        } else {
            self.cur_dir = Some(
                Path::new(&self.cur_dir.take().unwrap())
                    .join(name)
                    .to_str()
                    .unwrap()
                    .to_owned(),
            );
        }
    }

    fn ls_dir(&mut self, name: String) {
        match &self.cur_dir {
            Some(cur_dir) => {
                let new_path = Path::new(&cur_dir.clone())
                    .join(name)
                    .to_str()
                    .unwrap()
                    .to_owned();
                self.dir_map
                    .get_mut(cur_dir)
                    .unwrap()
                    .children
                    .push(new_path.clone());
                self.dir_map
                    .insert(new_path.clone(), Dir::new(new_path.clone()));
            }
            None => panic!("cur dir cant be empty before ls"),
        }
    }

    fn calc_dir_size(&mut self, name: String) -> usize {
        let children = self.dir_map.get(&name).unwrap().children.clone();
        let children_size: usize = children
            .iter()
            .map(|x| self.calc_dir_size(x.to_owned()))
            .sum();
        self.dir_map.get_mut(&name).unwrap().file_size += children_size;
        println!("{}: {}", name, self.dir_map.get(&name).unwrap().file_size);
        self.dir_map.get(&name).unwrap().file_size
    }

    fn get_sum_size_of_dirs_atmost_size(&self, size: usize) -> usize {
        self.dir_map
            .iter()
            .filter_map(|k_v| {
                if k_v.1.file_size <= size {
                    Some(k_v.1.file_size)
                } else {
                    None
                }
            })
            .sum()
    }

    fn min_size_to_clear(&self) -> usize {
        let total_size = 70_000_000;
        let filled_size = self.dir_map.get("/").unwrap().file_size;
        let unused = total_size - filled_size;
        dbg!(unused);
        let to_clear = 30_000_000 - unused;

        self.dir_map
            .values()
            .filter_map(|y| {
                if y.file_size > to_clear {
                    Some(y.file_size)
                } else {
                    None
                }
            })
            .min()
            .unwrap()
    }
}
