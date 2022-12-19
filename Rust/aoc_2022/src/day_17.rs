use std::{
    collections::HashSet,
    fs::File,
    io::{prelude::*, BufReader},
    time::Instant,
};

#[derive(Clone, Debug)]
struct Tunnel {
    active_rock: Vec<(usize, usize)>,
    fallen_rocks: HashSet<(usize, usize)>,
    airflow: Vec<char>,
    airflow_index: usize,
    airflow_max: usize,
    shapes: Vec<Vec<(usize, usize)>>,
    shape_index: usize,
    shape_max: usize,
    fallen_rock_count: usize,
}

impl Tunnel {
    fn new() -> Self {
        let mut fallen_rocks: HashSet<(usize, usize)> = HashSet::new();
        fallen_rocks.insert((0, 0));
        fallen_rocks.insert((1, 0));
        fallen_rocks.insert((2, 0));
        fallen_rocks.insert((3, 0));
        fallen_rocks.insert((4, 0));
        fallen_rocks.insert((5, 0));
        fallen_rocks.insert((6, 0));

        let file = File::open("./input/day_17").expect("no such file");
        let buf = BufReader::new(file);

        let airflow: Vec<char> = buf.lines().next().unwrap().unwrap().chars().collect();

        Tunnel {
            active_rock: vec![(2, 4), (3, 4), (4, 4), (5, 4)],
            fallen_rocks,
            airflow_max: airflow.len(),
            airflow,
            airflow_index: 0,
            shapes: vec![
                vec![(2, 4), (3, 4), (4, 4), (5, 4)],
                vec![(3, 4), (2, 5), (3, 5), (4, 5), (3, 6)],
                vec![(2, 4), (3, 4), (4, 4), (4, 5), (4, 6)],
                vec![(2, 4), (2, 5), (2, 6), (2, 7)],
                vec![(2, 4), (3, 4), (2, 5), (3, 5)],
            ],
            shape_index: 0,
            shape_max: 5,
            fallen_rock_count: 0,
        }
    }

    fn max_height(&self) -> usize {
        *self.fallen_rocks.iter().map(|(_x, y)| y).max().unwrap()
    }

    fn fall(&mut self) -> usize {
        let mut pushed_rock = self.active_rock.clone();
        let rock_size = self.active_rock.len();
        match self.airflow[self.airflow_index] {
            '<' => {
                pushed_rock = pushed_rock
                    .iter()
                    .filter_map(|(x, y)| {
                        if *x > 0 && !self.fallen_rocks.contains(&(x - 1, *y)) {
                            Some((x - 1, *y))
                        } else {
                            None
                        }
                    })
                    .collect();
                if pushed_rock.len() != rock_size {
                    pushed_rock = self.active_rock.clone();
                }
            }
            '>' => {
                pushed_rock = pushed_rock
                    .iter()
                    .filter_map(|(x, y)| {
                        if *x < 6 && !self.fallen_rocks.contains(&(x + 1, *y)) {
                            Some((x + 1, *y))
                        } else {
                            None
                        }
                    })
                    .collect();
                if pushed_rock.len() != rock_size {
                    pushed_rock = self.active_rock.clone();
                }
            }
            _ => panic!(),
        }
        self.airflow_index = (self.airflow_index + 1) % self.airflow_max;

        // if self.airflow_index == 0 {
        //     println!("Rock {:?}", self.active_rock);
        //     println!("Count {:?}", self.fallen_rock_count);
        //     println!("Height {:?}", self.max_height());
        // }

        let mut falling_rock = pushed_rock.clone();
        falling_rock = falling_rock
            .iter()
            .filter_map(|(x, y)| {
                let new_pos = (*x, y - 1);
                if !self.fallen_rocks.contains(&new_pos) {
                    Some(new_pos)
                } else {
                    None
                }
            })
            .collect();

        // println!("PRock {:?}", pushed_rock);
        if falling_rock.len() != rock_size {
            for pos in pushed_rock {
                self.fallen_rocks.insert(pos);
            }

            self.shape_index = (self.shape_index + 1) % self.shape_max;

            let height = self.max_height();
            // println!("Height {:?}", height);

            self.active_rock = self.shapes[self.shape_index]
                .clone()
                .iter()
                .map(|(x, y)| (*x, y + height))
                .collect();

            self.fallen_rock_count += 1;
        } else {
            self.active_rock = falling_rock;
        }
        self.fallen_rock_count
    }
}

pub fn run() {
    println!("Day 17 of 2022");

    let timer_input_start = Instant::now();

    let tunnel = Tunnel::new();

    let timer_input_end = Instant::now();
    println!(
        "Input parsed in {:?}",
        timer_input_end.duration_since(timer_input_start)
    );

    let timer1_start = Instant::now();
    let result = part_1(tunnel.clone());
    let timer1_end = Instant::now();
    println!(
        "Part 1: {} (in {:?})",
        result,
        timer1_end.duration_since(timer1_start)
    );

    let timer2_start = Instant::now();
    let result = part_2(tunnel);
    let timer2_end = Instant::now();
    println!(
        "Part 2: {} (in {:?})",
        result,
        timer2_end.duration_since(timer2_start)
    );
}

fn part_1(mut input: Tunnel) -> usize {
    while input.fall() < 2022 {}
    input.max_height()
}

fn part_2(mut input: Tunnel) -> i64 {
    while input.fall() < 6 + 134 + 1735 {}
    input.max_height() as i64 + 2695 * (576368875_i64)
}
