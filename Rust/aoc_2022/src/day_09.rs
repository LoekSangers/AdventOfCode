use std::{
    collections::HashSet,
    fs::File,
    io::{prelude::*, BufReader},
    time::Instant,
};

pub fn run() {
    println!("Day 9 of 2022");

    let timer_input_start = Instant::now();

    let file = File::open("./input/day_09").expect("no such file");
    let buf = BufReader::new(file);
    let input: Vec<(String, usize)> = buf
        .lines()
        .map(|l| {
            let tmp: Vec<String> = l
                .unwrap()
                .split_ascii_whitespace()
                .map(|s| s.to_owned())
                .collect();
            (
                tmp.first().unwrap().to_owned(),
                tmp.last().unwrap().parse::<usize>().unwrap(),
            )
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

fn part_1(input: &[(String, usize)]) -> usize {
    let mut positions: HashSet<(i16, i16)> = HashSet::new();
    let mut tail_pos = (0_i16, 0_i16);
    positions.insert(tail_pos);

    let mut head_pos = (0_i16, 0_i16);

    for (dir, steps) in input {
        let mov = match dir.as_str() {
            "R" => (1, 0),
            "U" => (0, 1),
            "L" => (-1, 0),
            "D" => (0, -1),
            _ => panic!(),
        };
        for _ in 0..*steps {
            head_pos = (head_pos.0 + mov.0, head_pos.1 + mov.1);
            let tail_move = match (head_pos.0 - tail_pos.0, head_pos.1 - tail_pos.1) {
                (0, 0) => (0, 0),
                (0, 1) => (0, 0),
                (0, -1) => (0, 0),
                (1, 0) => (0, 0),
                (1, 1) => (0, 0),
                (1, -1) => (0, 0),
                (-1, 0) => (0, 0),
                (-1, 1) => (0, 0),
                (-1, -1) => (0, 0),

                (0, 2) => (0, 1),
                (1, 2) => (1, 1),
                (-1, 2) => (-1, 1),

                (0, -2) => (0, -1),
                (1, -2) => (1, -1),
                (-1, -2) => (-1, -1),

                (2, 0) => (1, 0),
                (2, 1) => (1, 1),
                (2, -1) => (1, -1),

                (-2, 0) => (-1, 0),
                (-2, 1) => (-1, 1),
                (-2, -1) => (-1, -1),

                _ => panic!(),
            };
            tail_pos = (tail_pos.0 + tail_move.0, tail_pos.1 + tail_move.1);

            positions.insert(tail_pos);
        }
    }

    positions.len()
}

fn part_2(input: &[(String, usize)]) -> usize {
    let mut positions: HashSet<(i16, i16)> = HashSet::new();
    let mut snake_pos = [
        (0_i16, 0_i16),
        (0_i16, 0_i16),
        (0_i16, 0_i16),
        (0_i16, 0_i16),
        (0_i16, 0_i16),
        (0_i16, 0_i16),
        (0_i16, 0_i16),
        (0_i16, 0_i16),
        (0_i16, 0_i16),
        (0_i16, 0_i16),
    ];
    positions.insert(snake_pos[9]);

    for (dir, steps) in input {
        let mov = match dir.as_str() {
            "R" => (1, 0),
            "U" => (0, 1),
            "L" => (-1, 0),
            "D" => (0, -1),
            _ => panic!(),
        };
        for _ in 0..*steps {
            snake_pos[0] = (snake_pos[0].0 + mov.0, snake_pos[0].1 + mov.1);
            snake_pos[1] = move_tail(&snake_pos[0], &snake_pos[1]);
            snake_pos[2] = move_tail(&snake_pos[1], &snake_pos[2]);
            snake_pos[3] = move_tail(&snake_pos[2], &snake_pos[3]);
            snake_pos[4] = move_tail(&snake_pos[3], &snake_pos[4]);
            snake_pos[5] = move_tail(&snake_pos[4], &snake_pos[5]);
            snake_pos[6] = move_tail(&snake_pos[5], &snake_pos[6]);
            snake_pos[7] = move_tail(&snake_pos[6], &snake_pos[7]);
            snake_pos[8] = move_tail(&snake_pos[7], &snake_pos[8]);
            snake_pos[9] = move_tail(&snake_pos[8], &snake_pos[9]);
            positions.insert(snake_pos[9]);
        }
    }
    positions.len()
}

fn move_tail(p1: &(i16, i16), p2: &(i16, i16)) -> (i16, i16) {
    let tail_move = match (p1.0 - p2.0, p1.1 - p2.1) {
        (0, 0) => (0, 0),
        (0, 1) => (0, 0),
        (0, -1) => (0, 0),
        (1, 0) => (0, 0),
        (1, 1) => (0, 0),
        (1, -1) => (0, 0),
        (-1, 0) => (0, 0),
        (-1, 1) => (0, 0),
        (-1, -1) => (0, 0),

        (0, 2) => (0, 1),
        (1, 2) => (1, 1),
        (-1, 2) => (-1, 1),

        (0, -2) => (0, -1),
        (1, -2) => (1, -1),
        (-1, -2) => (-1, -1),

        (2, 0) => (1, 0),
        (2, 1) => (1, 1),
        (2, -1) => (1, -1),

        (-2, 0) => (-1, 0),
        (-2, 1) => (-1, 1),
        (-2, -1) => (-1, -1),

        (2, 2) => (1, 1),
        (2, -2) => (1, -1),

        (-2, 2) => (-1, 1),
        (-2, -2) => (-1, -1),

        _ => panic!(),
    };
    (p2.0 + tail_move.0, p2.1 + tail_move.1)
}
