use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs::File,
    io::{prelude::*, BufReader},
    time::Instant,
};

use itertools::{iproduct, Itertools};

pub fn run() {
    println!("Day 19 of 2022");

    let timer_input_start = Instant::now();

    let file = File::open("./input/day_19").expect("no such file");
    let buf = BufReader::new(file);
    let input: HashSet<(isize, isize, isize)> = buf.lines().fold(HashSet::new(), |mut set, l| {
        let parts: Vec<String> = l.unwrap().split(',').map(|s| s.to_owned()).collect();
        set.insert((
            parts[0].parse().unwrap(),
            parts[1].parse().unwrap(),
            parts[2].parse().unwrap(),
        ));
        set
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

fn part_1(input: &HashSet<(isize, isize, isize)>) -> usize {
    let cube_faces = vec![
        (0, 0, -1),
        (0, 0, 1),
        (0, -1, 0),
        (0, 1, 0),
        (-1, 0, 0),
        (1, 0, 0),
    ];
    input
        .iter()
        .flat_map(|(x, y, z)| {
            cube_faces
                .iter()
                .filter_map(move |(x_offset, y_offset, z_offset)| {
                    let new_pos = (x + x_offset, y + y_offset, z + z_offset);
                    if input.contains(&new_pos) {
                        None
                    } else {
                        Some(new_pos)
                    }
                })
        })
        .count()
}

fn part_2(input: &HashSet<(isize, isize, isize)>) -> usize {
    let cube_faces = vec![
        (0, 0, -1),
        (0, 0, 1),
        (0, -1, 0),
        (0, 1, 0),
        (-1, 0, 0),
        (1, 0, 0),
    ];
    let faces: HashMap<(isize, isize, isize), usize> =
        input
            .clone()
            .iter()
            .fold(HashMap::new(), |mut map, (x, y, z)| {
                for (x_offset, y_offset, z_offset) in &cube_faces {
                    let new_pos = (x + x_offset, y + y_offset, z + z_offset);
                    if !input.contains(&new_pos) {
                        map.entry(new_pos).and_modify(|c| *c += 1).or_insert(1);
                    }
                }
                map
            });

    println!("Faces {:?}", faces);
    let mut sum = 0;
    let max_x = input.iter().map(|(x, _y, _z)| x).max().unwrap();
    let max_y = input.iter().map(|(_x, y, _z)| y).max().unwrap();
    let max_z = input.iter().map(|(_x, _y, z)| z).max().unwrap();

    for ((x, y, z), count) in faces {
        if !((0..x).any(|i| input.contains(&(i, y, z)))
            && (x..=*max_x).any(|i| input.contains(&(i, y, z)))
            && (0..y).any(|i| input.contains(&(x, i, z)))
            && (y..=*max_y).any(|i| input.contains(&(x, i, z)))
            && (0..z).any(|i| input.contains(&(x, y, i)))
            && (z..=*max_z).any(|i| input.contains(&(x, y, i))))
        {
            sum += count;
        }
    }

    sum
}
