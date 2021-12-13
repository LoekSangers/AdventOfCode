use std::collections::HashSet;
use std::time::Instant;

pub fn run(input: Vec<String>) {
    println!("Day 3 of 2015");

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

fn part_1(input: &Vec<String>) -> usize {
    let mut x = 0;
    let mut y = 0;
    let mut positions = HashSet::new();
    positions.insert((x, y));
    return input[0]
        .chars()
        .fold(positions, |mut set, ch| {
            match ch {
                '>' => x += 1,
                '<' => x -= 1,
                '^' => y += 1,
                'v' => y -= 1,
                _ => panic!("invalid character!"),
            }
            set.insert((x, y));
            set
        })
        .len();
}

fn part_2(input: &Vec<String>) -> usize {
    let mut x = 0;
    let mut y = 0;
    let mut robo_x = 0;
    let mut robo_y = 0;
    let mut positions = HashSet::new();
    positions.insert((x, y));
    return input[0]
        .chars()
        .enumerate()
        .fold(positions, |mut set, (i, ch)| {
            if i % 2 == 0 {
                match ch {
                    '>' => x += 1,
                    '<' => x -= 1,
                    '^' => y += 1,
                    'v' => y -= 1,
                    _ => panic!("invalid character!"),
                }
                set.insert((x, y));
            } else {
                match ch {
                    '>' => robo_x += 1,
                    '<' => robo_x -= 1,
                    '^' => robo_y += 1,
                    'v' => robo_y -= 1,
                    _ => panic!("invalid character!"),
                }
                set.insert((robo_x, robo_y));
            }
            set
        })
        .len();
}
