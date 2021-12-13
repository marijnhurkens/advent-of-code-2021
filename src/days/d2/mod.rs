use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn answer1() -> usize {
    let file = File::open("./data/d2/input.txt").unwrap();
    let lines = BufReader::new(file).lines();

    let mut horizontal: usize = 0;
    let mut depth: usize = 0;

    for line in lines.filter_map(|str| str.ok()) {
        let mut split = line.split(" ");
        match split.next() {
            Some("forward") => horizontal += split.next().unwrap().parse::<usize>().unwrap(),
            Some("down") => depth += split.next().unwrap().parse::<usize>().unwrap(),
            Some("up") => depth -= split.next().unwrap().parse::<usize>().unwrap(),
            _ => ()
        };
    }

    horizontal * depth
}