use std::{
    fs::File,
    io::{prelude::*, BufReader},
    time::Instant,
};

pub fn run() {
    println!("Day 4 of 2022");

    let timer_input_start = Instant::now();

    let file = File::open("./input/day_04").expect("no such file");
    let buf = BufReader::new(file);
    let input: Vec<Vec<usize>> = buf
        .lines()
        .map(|l| {
            l.unwrap()
                .split(&[',', '-'])
                .map(|coord| coord.parse::<usize>().unwrap())
                .collect()
        })
        .collect();

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
    input.iter().fold(0_usize, |mut acc, coords| {
        if (coords[0] <= coords[2] && coords[1] >= coords[3])
            || (coords[0] >= coords[2] && coords[1] <= coords[3])
        {
            acc += 1;
        }
        acc
    })
}

fn part_2(input: &[Vec<usize>]) -> usize {
    input.iter().fold(0_usize, |mut acc, coords| {
        if (coords[0] <= coords[2] && coords[1] >= coords[3])
            || (coords[0] >= coords[2] && coords[1] <= coords[3])
            || (coords[0] >= coords[2] && coords[0] <= coords[3])
            || (coords[1] >= coords[2] && coords[1] <= coords[3])
        {
            acc += 1;
        }
        acc
    })
}
