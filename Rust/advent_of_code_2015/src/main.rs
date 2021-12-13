use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

mod day_01;
mod day_02;
mod day_03;
mod day_04;


fn main() {
    day_01::run(lines_from_file("./input/day_01"));
    day_02::run(lines_from_file("./input/day_02"));
    day_03::run(lines_from_file("./input/day_03"));
    day_04::run(lines_from_file("./input/day_04"));
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}