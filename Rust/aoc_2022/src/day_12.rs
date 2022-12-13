use std::{
    collections::{HashMap, VecDeque},
    fs::File,
    io::{prelude::*, BufReader},
    time::Instant,
};

pub fn run() {
    println!("Day 12 of 2022");

    let timer_input_start = Instant::now();

    let file = File::open("./input/day_12").expect("no such file");
    let buf = BufReader::new(file);
    let (input, start, end) = buf.lines().map(|l| l.unwrap()).enumerate().fold(
        (HashMap::new(), (0, 0), (0, 0)),
        |(dict, start, end), (i, line)| {
            line.chars().enumerate().fold(
                (dict, start, end),
                |(mut d, mut start, mut end), (j, map_pos)| {
                    d.insert((i as isize, j as isize), to_height(map_pos));
                    match map_pos {
                        'S' => start = (i, j),
                        'E' => end = (i, j),
                        _ => (),
                    };
                    (d, start, end)
                },
            )
        },
    );

    let timer_input_end = Instant::now();
    println!(
        "Input parsed in {:?}",
        timer_input_end.duration_since(timer_input_start)
    );

    let timer1_start = Instant::now();
    let result = part_1(&input, start, end);
    let timer1_end = Instant::now();
    println!(
        "Part 1: {} (in {:?})",
        result,
        timer1_end.duration_since(timer1_start)
    );

    let timer2_start = Instant::now();
    let result = part_2(&input, start, end);
    let timer2_end = Instant::now();
    println!(
        "Part 2: {} (in {:?})",
        result,
        timer2_end.duration_since(timer2_start)
    );
}

fn part_1(
    input: &HashMap<(isize, isize), usize>,
    start: (usize, usize),
    end: (usize, usize),
) -> usize {
    let mut route_map: HashMap<(isize, isize), (isize, isize)> = HashMap::new();
    let mut visited: Vec<(isize, isize)> = vec![];
    let mut to_visit: VecDeque<(isize, isize)> = VecDeque::new();
    let mut current: (isize, isize) = (start.0 as isize, start.1 as isize);
    let target: (isize, isize) = (end.0 as isize, end.1 as isize);

    while !(current.0 == target.0 && current.1 == target.1) {
        let neighours = [
            (current.0 + 1, current.1),
            (current.0 - 1, current.1),
            (current.0, current.1 + 1),
            (current.0, current.1 - 1),
        ];

        for pos in neighours {
            if !to_visit.contains(&pos)
                && !visited.contains(&pos)
                && input.get(&pos).unwrap_or(&99) <= &(input.get(&current).unwrap() + 1)
            {
                route_map.insert(pos, current);
                to_visit.push_back(pos);
            }
        }

        visited.push(current);
        current = to_visit.pop_front().unwrap();
    }

    let mut route: VecDeque<(isize, isize)> = VecDeque::new();
    let target: (isize, isize) = (start.0 as isize, start.1 as isize);
    while !(current.0 == target.0 && current.1 == target.1) {
        route.push_front(current);
        current = *route_map.get(&current).unwrap();
    }

    route.len()
}

fn part_2(
    input: &HashMap<(isize, isize), usize>,
    start: (usize, usize),
    end: (usize, usize),
) -> usize {
    let mut route_map: HashMap<(isize, isize), (isize, isize)> = HashMap::new();
    let mut visited: Vec<(isize, isize)> = vec![];
    let mut to_visit: VecDeque<(isize, isize)> = VecDeque::new();
    let mut current: (isize, isize) = (end.0 as isize, end.1 as isize);

    while input.get(&current).unwrap() != &1 {
        let neighours = [
            (current.0 + 1, current.1),
            (current.0 - 1, current.1),
            (current.0, current.1 + 1),
            (current.0, current.1 - 1),
        ];

        for pos in neighours {
            if !to_visit.contains(&pos)
                && !visited.contains(&pos)
                && input.get(&pos).unwrap_or(&0) >= &(input.get(&current).unwrap() - 1)
            {
                route_map.insert(pos, current);
                to_visit.push_back(pos);
            }
        }

        visited.push(current);
        current = to_visit.pop_front().unwrap();
    }

    let mut route: VecDeque<(isize, isize)> = VecDeque::new();
    let target: (isize, isize) = (start.0 as isize, start.1 as isize);
    while !(current.0 == target.0 && current.1 == target.1) {
        route.push_front(current);
        current = *route_map.get(&current).unwrap_or(&target);
    }

    route.len() - 1
}

fn to_height(item: char) -> usize {
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

        'S' => 1,
        'E' => 26,
        _ => panic!("char: {:?}", item),
    }
}
