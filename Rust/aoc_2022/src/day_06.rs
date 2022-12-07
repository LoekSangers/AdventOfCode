use std::{
    collections::HashSet,
    fs::File,
    io::{prelude::*, BufReader},
    time::Instant,
};

pub fn run() {
    println!("Day 5 of 2022");

    let timer_input_start = Instant::now();

    let file = File::open("./input/day_06").expect("no such file");
    let buf = BufReader::new(file);
    let input = buf
        .lines()
        .next()
        .expect("")
        .expect("")
        .chars()
        .collect::<Vec<char>>();

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
    let result = part_2_perf(&input);
    let timer2_end = Instant::now();
    println!(
        "Part 2: {} (in {:?})",
        result,
        timer2_end.duration_since(timer2_start)
    );
}

fn part_1(input: &[char]) -> usize {
    let mut windows = input.windows(4).enumerate();
    while let Some((index, &[elem1, elem2, elem3, elem4])) = windows.next() {
        if !(elem1 == elem2
            || elem1 == elem3
            || elem1 == elem4
            || elem2 == elem3
            || elem2 == elem4
            || elem3 == elem4)
        {
            return index + 4;
        }
    }
    0
}
#[allow(dead_code)]
fn part_2_slow(input: &[char]) -> usize {
    let windows = input.windows(14).enumerate();
    for (index, elems) in windows {
        let set: HashSet<char> = elems.iter().copied().collect();
        if elems.len() == set.len() {
            return index + 14;
        }
    }
    0
}

#[allow(dead_code)]
fn part_2(input: &[char]) -> usize {
    let windows = input.windows(14).enumerate();
    for (index, elems) in windows {
        if !((1..elems.len()).any(|i| elems[i..].contains(&elems[i - 1]))) {
            return index + 14;
        }
    }
    0
}

fn part_2_perf(input: &[char]) -> usize {
    let windows = input.windows(14).enumerate();
    for (index, elems) in windows {
        if !(elems[1..].contains(&elems[0])
            || elems[2..].contains(&elems[1])
            || elems[3..].contains(&elems[2])
            || elems[4..].contains(&elems[3])
            || elems[5..].contains(&elems[4])
            || elems[6..].contains(&elems[5])
            || elems[7..].contains(&elems[6])
            || elems[8..].contains(&elems[7])
            || elems[9..].contains(&elems[8])
            || elems[10..].contains(&elems[9])
            || elems[11..].contains(&elems[10])
            || elems[12..].contains(&elems[11])
            || elems[13] == elems[12])
        {
            return index + 14;
        }
    }
    0
}
