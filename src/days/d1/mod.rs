use std::fs::File;
use std::io::{BufRead, BufReader};
use itertools::Itertools;

pub fn answer1() -> usize {
    let file = File::open("./data/d1/input.txt").unwrap();
    let lines = BufReader::new(file).lines();

    lines
        .filter_map(|item| item.ok()?.parse::<usize>().ok())
        .tuple_windows::<(_, _)>()
        .into_iter()
        .filter(|window| {
            window.0 < window.1
        })
        .count()
}

pub fn answer2() -> usize {
    let file = File::open("./data/d1/input.txt").unwrap();
    let lines = BufReader::new(file).lines();

    lines
        .filter_map(|item| item.ok())
        .filter_map(|item| item.parse::<usize>().ok())
        .tuple_windows::<(_, _, _)>()
        .into_iter()
        .map(|window| {
            window.0 + window.1 + window.2
        })
        .tuple_windows::<(_,_)>()
        .into_iter()
        .filter(|window| {
            window.0 < window.1
        })
        .count()
}

