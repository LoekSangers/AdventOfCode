use std::collections::HashSet;
use std::{
    fs::File,
    io::{prelude::*, BufReader},
    time::Instant,
};

pub fn run() {
    println!("Day 3 of 2022");

    let timer_input_start = Instant::now();

    let file = File::open("./input/day_03").expect("no such file");
    let buf = BufReader::new(file);
    let input: Vec<String> = buf.lines().map(|l| l.unwrap()).collect();

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

fn part_1(input: &[String]) -> usize {
    input.iter().fold(0_usize, |acc, elem| {
        let (bp1, bp2) = elem.split_at(elem.chars().count() / 2);
        let set: HashSet<char> = bp1.chars().collect();

        acc + to_priority(bp2.chars().fold(' ', |mut char, element| {
            if set.contains(&element) {
                char = element;
            }
            char
        }))
    })
}

fn part_2(input: &[String]) -> usize {
    input.chunks(3).fold(0_usize, |acc, backpacks| {
        let sets: [Vec<char>; 2] = [
            backpacks[1].chars().collect(),
            backpacks[2].chars().collect(),
        ];

        acc + to_priority(
            backpacks[0]
                .chars()
                .find(move |c| sets.iter().all(|s| s.contains(c)))
                .unwrap(),
        )
    })
}

fn to_priority(item: char) -> usize {
    match item {
        'a' => 1,
        'b' => 2,
        'c' => 3,
        'd' => 4,
        'e' => 5,
        'f' => 6,
        'g' => 7,
        'h' => 8,
        'i' => 9,
        'j' => 10,
        'k' => 11,
        'l' => 12,
        'm' => 13,
        'n' => 14,
        'o' => 15,
        'p' => 16,
        'q' => 17,
        'r' => 18,
        's' => 19,
        't' => 20,
        'u' => 21,
        'v' => 22,
        'w' => 23,
        'x' => 24,
        'y' => 25,
        'z' => 26,

        'A' => 1 + 26,
        'B' => 2 + 26,
        'C' => 3 + 26,
        'D' => 4 + 26,
        'E' => 5 + 26,
        'F' => 6 + 26,
        'G' => 7 + 26,
        'H' => 8 + 26,
        'I' => 9 + 26,
        'J' => 10 + 26,
        'K' => 11 + 26,
        'L' => 12 + 26,
        'M' => 13 + 26,
        'N' => 14 + 26,
        'O' => 15 + 26,
        'P' => 16 + 26,
        'Q' => 17 + 26,
        'R' => 18 + 26,
        'S' => 19 + 26,
        'T' => 20 + 26,
        'U' => 21 + 26,
        'V' => 22 + 26,
        'W' => 23 + 26,
        'X' => 24 + 26,
        'Y' => 25 + 26,
        'Z' => 26 + 26,

        _ => 0,
    }
}
