use std::{
    fs::File,
    io::{prelude::*, BufReader},
    time::Instant,
};

pub fn run() {
    println!("Day 5 of 2022");

    let timer_input_start = Instant::now();

    let file = File::open("./input/day_05").expect("no such file");
    let buf = BufReader::new(file);
    let mut input: Vec<String> = buf.lines().map(|l| l.unwrap()).collect();
    let mut initial_stacks: [Vec<char>; 9] = [
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
    ];
    let crates: &mut [String] = &mut input[0..8];
    crates.reverse();
    for row in crates.iter() {
        let mut chars = row.chars();
        if let Some(c) = chars.nth(1) {
            if c != ' ' {
                initial_stacks[0].push(c);
            }
        }
        if let Some(c) = chars.nth(3) {
            if c != ' ' {
                initial_stacks[1].push(c);
            }
        }
        if let Some(c) = chars.nth(3) {
            if c != ' ' {
                initial_stacks[2].push(c);
            }
        }
        if let Some(c) = chars.nth(3) {
            if c != ' ' {
                initial_stacks[3].push(c);
            }
        }
        if let Some(c) = chars.nth(3) {
            if c != ' ' {
                initial_stacks[4].push(c);
            }
        }
        if let Some(c) = chars.nth(3) {
            if c != ' ' {
                initial_stacks[5].push(c);
            }
        }
        if let Some(c) = chars.nth(3) {
            if c != ' ' {
                initial_stacks[6].push(c);
            }
        }
        if let Some(c) = chars.nth(3) {
            if c != ' ' {
                initial_stacks[7].push(c);
            }
        }
        if let Some(c) = chars.nth(3) {
            if c != ' ' {
                initial_stacks[8].push(c);
            }
        }
    }

    let commands: Vec<Vec<usize>> = input[10..]
        .iter()
        .map(|l| {
            l.chars()
                .filter(|c| c.is_ascii_digit() || c.is_whitespace())
                .collect::<String>()
                .split_whitespace()
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
    let result = part_1(initial_stacks.clone().as_mut(), &commands);
    let timer1_end = Instant::now();
    println!(
        "Part 1: {} (in {:?})",
        result,
        timer1_end.duration_since(timer1_start)
    );

    let timer2_start = Instant::now();
    let result = part_2(initial_stacks.clone().as_mut(), &commands);
    let timer2_end = Instant::now();
    println!(
        "Part 2: {} (in {:?})",
        result,
        timer2_end.duration_since(timer2_start)
    );
}

fn part_1(stacks: &mut [Vec<char>], commands: &[Vec<usize>]) -> String {
    for command in commands.iter() {
        let times = command[0];
        let from = command[1];
        let to = command[2];
        for _ in 0..times {
            let elem = stacks[from - 1].pop().unwrap();
            stacks[to - 1].push(elem);
        }
    }
    stacks
        .iter_mut()
        .map(|stack| stack.pop().unwrap())
        .collect()
}

fn part_2(stacks: &mut [std::vec::Vec<char>], commands: &[Vec<usize>]) -> String {
    for command in commands.iter() {
        let times = command[0];
        let from = command[1];
        let to = command[2];

        let mut tail = stacks[from - 1].split_off(stacks[from - 1].len().saturating_sub(times));
        stacks[to - 1].append(&mut tail);
    }
    stacks
        .iter_mut()
        .map(|stack| stack.pop().unwrap())
        .collect()
}
