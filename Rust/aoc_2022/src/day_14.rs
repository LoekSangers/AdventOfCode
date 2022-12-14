use std::{
    cmp::{max, min},
    collections::HashMap,
    fs::File,
    io::{prelude::*, BufReader},
    time::Instant,
};

pub fn run() {
    println!("Day 14 of 2022");

    let timer_input_start = Instant::now();

    let file = File::open("./input/day_14").expect("no such file");
    let buf = BufReader::new(file);
    let input: Vec<Vec<(usize, usize)>> = buf
        .lines()
        .map(|l| {
            l.unwrap()
                .split(" -> ")
                .flat_map(|s| {
                    s.split(',')
                        .map(|x| x.parse().unwrap())
                        .collect::<Vec<usize>>()
                        .chunks(2)
                        .fold(Vec::new(), |mut v, elem| {
                            v.push((elem[0], elem[1]));
                            v
                        })
                })
                .collect()
        })
        .collect();
    let mut cave_system: HashMap<(usize, usize), char> = HashMap::new();
    let mut min_x = 500;
    let mut max_x = 500;
    let mut min_y = 0;
    let mut max_y = 0;

    for line in input {
        for coords in line.windows(2) {
            let start = coords[0];
            let end = coords[1];
            for x in min(start.0, end.0)..=max(start.0, end.0) {
                if x < min_x {
                    min_x = x;
                } else if x > max_x {
                    max_x = x;
                }
                for y in min(start.1, end.1)..=max(start.1, end.1) {
                    if y < min_y {
                        min_y = y;
                    } else if y > max_y {
                        max_y = y;
                    }
                    cave_system.insert((x, y), '#');
                }
            }
        }
    }
    for x in min_x..=max_x {
        for y in min_y..=max_y {
            cave_system.entry((x, y)).or_insert('.');
        }
    }
    // for y in 0..=9 {
    //     println!(
    //         "{:?}",
    //         (494..=503)
    //             .map(|i| *cave_system.entry((i, y)).or_default())
    //             .collect::<String>()
    //     );
    // }

    let timer_input_end = Instant::now();
    println!(
        "Input parsed in {:?}",
        timer_input_end.duration_since(timer_input_start)
    );

    let timer1_start = Instant::now();
    let result = part_1(cave_system.clone());
    let timer1_end = Instant::now();
    println!(
        "Part 1: {} (in {:?})",
        result,
        timer1_end.duration_since(timer1_start)
    );

    let timer2_start = Instant::now();
    let mut extended_cave_system = cave_system.clone();
    for x in (min_x - max_y)..=(max_x + max_y) {
        for y in min_y..=max_y {
            extended_cave_system.entry((x, y)).or_insert('.');
        }
    }
    for x in (min_x - max_y)..=(max_x + max_y) {
        extended_cave_system.entry((x, max_y + 1)).or_insert('.');
        extended_cave_system.entry((x, max_y + 2)).or_insert('#');
    }
    let result = part_2(extended_cave_system);
    let timer2_end = Instant::now();
    println!(
        "Part 2: {} (in {:?})",
        result,
        timer2_end.duration_since(timer2_start)
    );
}

fn part_1(mut input: HashMap<(usize, usize), char>) -> usize {
    let mut cur_sand_pos = (500, 0);
    let mut sand_count = 0;
    while input.get(&cur_sand_pos).unwrap_or(&'~') != &'~' {
        if input
            .get(&(cur_sand_pos.0, cur_sand_pos.1 + 1))
            .unwrap_or(&'~')
            == &'.'
        {
            cur_sand_pos = (cur_sand_pos.0, cur_sand_pos.1 + 1);
        } else if input
            .get(&(cur_sand_pos.0 - 1, cur_sand_pos.1 + 1))
            .unwrap_or(&'~')
            == &'.'
        {
            cur_sand_pos = (cur_sand_pos.0 - 1, cur_sand_pos.1 + 1);
        } else if input
            .get(&(cur_sand_pos.0 + 1, cur_sand_pos.1 + 1))
            .unwrap_or(&'~')
            == &'.'
        {
            cur_sand_pos = (cur_sand_pos.0 + 1, cur_sand_pos.1 + 1);
        } else if input
            .get(&(cur_sand_pos.0, cur_sand_pos.1 + 1))
            .unwrap_or(&'~')
            == &'~'
            || input
                .get(&(cur_sand_pos.0 - 1, cur_sand_pos.1 + 1))
                .unwrap_or(&'~')
                == &'~'
            || input
                .get(&(cur_sand_pos.0 + 1, cur_sand_pos.1 + 1))
                .unwrap_or(&'~')
                == &'~'
        {
            break;
        } else {
            input.entry(cur_sand_pos).and_modify(|c| *c = '0');
            cur_sand_pos = (500, 0);
            sand_count += 1;
        }
    }
    sand_count
}

fn part_2(mut input: HashMap<(usize, usize), char>) -> usize {
    let mut cur_sand_pos = (500, 0);
    let mut sand_count = 0;
    while input.get(&(500, 0)).unwrap_or(&'~') != &'0' {
        if input
            .get(&(cur_sand_pos.0, cur_sand_pos.1 + 1))
            .unwrap_or(&'~')
            == &'.'
        {
            cur_sand_pos = (cur_sand_pos.0, cur_sand_pos.1 + 1);
        } else if input
            .get(&(cur_sand_pos.0 - 1, cur_sand_pos.1 + 1))
            .unwrap_or(&'~')
            == &'.'
        {
            cur_sand_pos = (cur_sand_pos.0 - 1, cur_sand_pos.1 + 1);
        } else if input
            .get(&(cur_sand_pos.0 + 1, cur_sand_pos.1 + 1))
            .unwrap_or(&'~')
            == &'.'
        {
            cur_sand_pos = (cur_sand_pos.0 + 1, cur_sand_pos.1 + 1);
        } else {
            input.entry(cur_sand_pos).and_modify(|c| *c = '0');
            cur_sand_pos = (500, 0);
            sand_count += 1;
        }
    }
    // for y in 0..=13 {
    //     println!(
    //         "{:?}",
    //         (490..=510)
    //             .map(|i| *input.entry((i, y)).or_default())
    //             .collect::<String>()
    //     );
    // }
    sand_count
}
