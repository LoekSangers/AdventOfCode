use std::{
    fs::File,
    io::{prelude::*, BufReader},
    time::Instant,
};

pub fn run() {
    println!("Day 11 of 2022");

    let timer_input_start = Instant::now();

    let file = File::open("./input/day_11").expect("no such file");
    let buf = BufReader::new(file);
    let input: Vec<(String, isize)> = buf
        .lines()
        .map(|l| {
            let tmp: Vec<String> = l
                .unwrap()
                .split_ascii_whitespace()
                .map(|s| s.to_owned())
                .collect();
            (
                tmp.first().unwrap().to_owned(),
                tmp.last()
                    .unwrap_or(&String::from("0"))
                    .parse::<isize>()
                    .unwrap_or(0),
            )
        })
        .fold(
            Vec::new(),
            |mut commands: Vec<(String, isize)>, base_command| {
                if base_command.0 == "addx" {
                    commands.push((String::from("noop"), 0))
                }
                commands.push(base_command);
                commands
            },
        );

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
    part_2(&input);
    let timer2_end = Instant::now();
    println!("Part 2 in {:?}", timer2_end.duration_since(timer2_start));
}

fn part_1(input: &[(String, isize)]) -> isize {
    #[allow(non_snake_case)]
    let mut X: isize = 1;
    let mut res = 0;
    for (cycle, command) in input.iter().enumerate() {
        if cycle == 19 || (cycle > 19 && (cycle - 19) % 40 == 0) {
            res += ((cycle + 1) as isize) * X;
        }
        if command.0 == "addx" {
            X += command.1;
        }
    }
    res
}

fn part_2(input: &[(String, isize)]) {
    #[allow(non_snake_case)]
    let mut X: isize = 1;
    let mut screen: [[char; 40]; 6] = [
        [
            '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
            '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
            '.', '.', '.', '.', '.', '.',
        ],
        [
            '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
            '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
            '.', '.', '.', '.', '.', '.',
        ],
        [
            '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
            '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
            '.', '.', '.', '.', '.', '.',
        ],
        [
            '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
            '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
            '.', '.', '.', '.', '.', '.',
        ],
        [
            '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
            '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
            '.', '.', '.', '.', '.', '.',
        ],
        [
            '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
            '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
            '.', '.', '.', '.', '.', '.',
        ],
    ];
    for (cycle, command) in input.iter().enumerate() {
        let draw_pos = (cycle) % 40;
        if draw_pos >= (X - 1).try_into().unwrap_or(0) && draw_pos <= (X + 1).try_into().unwrap() {
            screen[cycle / 40][draw_pos] = '#';
        }

        if command.0 == "addx" {
            X += command.1;
        }
    }
    println!("{:?}", screen[0].iter().collect::<String>());
    println!("{:?}", screen[1].iter().collect::<String>());
    println!("{:?}", screen[2].iter().collect::<String>());
    println!("{:?}", screen[3].iter().collect::<String>());
    println!("{:?}", screen[4].iter().collect::<String>());
    println!("{:?}", screen[5].iter().collect::<String>());
}
