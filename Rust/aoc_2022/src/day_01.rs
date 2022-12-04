use std::{
    fs::File,
    io::{prelude::*, BufReader},
    time::Instant,
};

pub fn run() {
    println!("Day 1 of 2022");

    let timer_input_start = Instant::now();

    let file = File::open("./input/day_01").expect("no such file");
    let buf = BufReader::new(file);
    let input = buf
        .lines()
        .map(|l| l.unwrap())
        .fold(Vec::new(), |mut acc, elem| {
            if acc.is_empty() {
                acc.push(Vec::new());
            }

            if elem.is_empty() {
                acc.push(Vec::new());
            } else {
                acc.last_mut()
                    .unwrap()
                    .push(elem.parse::<usize>().expect("parse error"));
            }
            acc
        });

    let timer_input_end = Instant::now();
    println!(
        "Input parsed in {:?}",
        timer_input_end.duration_since(timer_input_start)
    );

    let timer1_start = Instant::now();
    let result = part_1(&input);
    let timer1_end = Instant::now();
    println!(
        "Part 1: {} (in {:?})",
        result,
        timer1_end.duration_since(timer1_start)
    );

    let timer2_start = Instant::now();
    let result = part_2(&input);
    let timer2_end = Instant::now();
    println!(
        "Part 2: {} (in {:?})",
        result,
        timer2_end.duration_since(timer2_start)
    );
}

fn part_1(input: &[Vec<usize>]) -> usize {
    return input
        .iter()
        .map(|elem| elem.iter().sum::<usize>())
        .max()
        .unwrap();
}

fn part_2(input: &[Vec<usize>]) -> usize {
    input
        .iter()
        .map(|elem| elem.iter().sum::<usize>())
        .fold([0; 3], |mut acc, elem| {
            if elem > acc[0] {
                acc[2] = acc[1];
                acc[1] = acc[0];
                acc[0] = elem;
            } else if elem > acc[1] {
                acc[2] = acc[1];
                acc[1] = elem;
            } else if elem > acc[2] {
                acc[2] = elem;
            }
            acc
        })
        .iter()
        .sum::<usize>()
}
