use std::{
    collections::HashMap,
    fs::File,
    io::{prelude::*, BufReader},
    time::Instant,
};

pub fn run() {
    println!("Day 8 of 2022");

    let timer_input_start = Instant::now();

    let file = File::open("./input/day_08").expect("no such file");
    let buf = BufReader::new(file);
    #[allow(unused_mut)]
    let input: HashMap<(usize, usize), u32> =
        buf.lines()
            .map(|l| l.unwrap())
            .enumerate()
            .fold(HashMap::new(), |mut dict, (i, line)| {
                line.chars()
                    .enumerate()
                    .fold(dict, |mut d, (j, tree_height)| {
                        d.insert((i, j), tree_height.to_digit(10_u32).unwrap());
                        d
                    })
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

fn part_1(input: &HashMap<(usize, usize), u32>) -> usize {
    input
        .iter()
        .filter(|((x, y), tree_height)| {
            !((0..*x).any(|i| -> bool { input.get(&(i, *y)).unwrap() >= tree_height }))
                || !(((*x + 1)..99)
                    .any(|i| -> bool { input.get(&(i, *y)).unwrap() >= tree_height }))
                || !((0..*y).any(|j| -> bool { input.get(&(*x, j)).unwrap() >= tree_height }))
                || !(((*y + 1)..99)
                    .any(|j| -> bool { input.get(&(*x, j)).unwrap() >= tree_height }))
        })
        .count()
}

fn part_2(input: &HashMap<(usize, usize), u32>) -> usize {
    input
        .iter()
        .fold(0_usize, |max_scenic_score, ((x, y), tree_height)| {
            let mut up = 1;
            if *x != 0 {
                let mut i = *x - 1;
                while i > 0 && input.get(&(i, *y)).unwrap_or(&10_u32) < tree_height {
                    up += 1;
                    i -= 1;
                }
            } else {
                up = 0;
            }

            let mut left = 1;
            if *y != 0 {
                let mut j = *y - 1;
                while j > 0 && input.get(&(*x, j)).unwrap_or(&10_u32) < tree_height {
                    left += 1;
                    j -= 1;
                }
            } else {
                left = 0;
            }

            let mut down = 1;
            if *x != 98 {
                let mut i = *x + 1;
                while i < 98 && input.get(&(i, *y)).unwrap_or(&10_u32) < tree_height {
                    down += 1;
                    i += 1;
                }
            } else {
                down = 0;
            }

            let mut right = 1;
            if *y != 98 {
                let mut j = *y + 1;
                while j < 98 && input.get(&(*x, j)).unwrap_or(&10_u32) < tree_height {
                    right += 1;
                    j += 1;
                }
            } else {
                right = 0;
            }

            let new_score = up * left * down * right;
            if new_score > max_scenic_score {
                new_score
            } else {
                max_scenic_score
            }
        })
}
