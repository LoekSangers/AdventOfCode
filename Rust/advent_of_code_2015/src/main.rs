use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

mod day_01;


fn main() {
    day_01::run(lines_from_file("./input/day_01"));
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}