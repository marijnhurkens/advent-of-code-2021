use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn answer1() -> usize {
    let file = File::open("./data/d2/input.txt").unwrap();
    let lines = BufReader::new(file).lines();

    let mut horizontal: usize = 0;
    let mut depth: usize = 0;

    for line in lines.filter_map(|str| str.ok()) {
        let mut split = line.split(" ");

        let command = split.next().unwrap();
        let x = split.next().unwrap().parse::<usize>().unwrap();

        match command {
            "forward" => horizontal += x,
            "down" => depth += x,
            "up" => depth -= x,
            _ => ()
        };
    }

    horizontal * depth
}

pub fn answer2() -> usize {
    let file = File::open("./data/d2/input.txt").unwrap();
    let lines = BufReader::new(file).lines();

    let mut horizontal: usize = 0;
    let mut aim: usize = 0;
    let mut depth: usize = 0;

    for line in lines.filter_map(|str| str.ok()) {
        let mut split = line.split(" ");

        let command = split.next().unwrap();
        let x = split.next().unwrap().parse::<usize>().unwrap();

        match command {
            "forward" => {
                horizontal += x;
                depth += aim * x;
            },
            "down" => aim += x,
            "up" => aim -= x,
            _ => ()
        };
    }

    horizontal * depth
}