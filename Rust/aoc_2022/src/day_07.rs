use std::{
    collections::HashMap,
    fs::File,
    io::{prelude::*, BufReader},
    time::Instant,
};

pub fn run() {
    println!("Day 7 of 2022");

    let timer_input_start = Instant::now();

    let file = File::open("./input/day_07").expect("no such file");
    let buf = BufReader::new(file);
    let input = buf
        .lines()
        .map(|l| l.unwrap())
        .fold(Vec::new(), |mut commands, line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            match parts[0] {
                "$" => match parts[1] {
                    "cd" => commands.push((parts[1].to_owned(), parts[2].to_owned(), 0_usize)),
                    "ls" => commands.push((parts[1].to_owned(), "".to_owned(), 0_usize)),
                    _ => panic!(),
                },
                "dir" => (),
                _ => {
                    let (com, _, size) = commands.pop().unwrap();

                    commands.push((
                        com,
                        "".to_string(),
                        size + parts[0].parse::<usize>().unwrap(),
                    ))
                }
            };
            commands
        })
        .iter()
        .fold(
            (HashMap::from([("".to_owned(), 0_usize)]), "".to_owned()),
            |(mut dict, mut dir), (command, folder, size)| {
                match command.as_str() {
                    "cd" => match folder.as_str() {
                        ".." => {
                            dir.pop();
                            while !dir.is_empty() && dir.pop().unwrap() != '-' {}
                            dir.push('-');
                        }
                        "/" => dir = "-".to_owned(),
                        _ => {
                            dir.push_str(folder);
                            dir.push('-');
                        }
                    },
                    "ls" => {
                        let folders: Vec<&str> = dir.split('-').collect();
                        for i in 1..folders.len() {
                            dict.entry(folders[0..i].join("/"))
                                .and_modify(|s| *s += size)
                                .or_insert(*size);
                        }
                    }
                    _ => panic!(),
                };
                (dict, dir)
            },
        )
        .0;

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

fn part_1(input: &HashMap<String, usize>) -> usize {
    input
        .clone()
        .into_values()
        .filter(|&size| size < 100000)
        .sum::<usize>()
}

fn part_2(input: &HashMap<String, usize>) -> usize {
    let available_space = 70000000 - *input.get("").unwrap();
    input
        .clone()
        .into_values()
        .filter(|&size| (size + available_space) >= 30000000)
        .min()
        .unwrap()
}
