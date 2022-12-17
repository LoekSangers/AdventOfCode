use std::{
    cmp::max,
    fs::File,
    io::{prelude::*, BufReader},
    time::Instant,
};

struct Range {
    x: isize,
    y: isize,
    radius: usize,
}

impl Range {
    fn new(s: &str) -> Self {
        let parts: Vec<&str> = s.split(&['=', ',', ':']).collect();
        let x = parts[1].parse().unwrap();
        let y = parts[3].parse().unwrap();

        let b_x: isize = parts[5].parse().unwrap();
        let b_y: isize = parts[7].parse().unwrap();
        Range {
            x,
            y,
            radius: (x - b_x as isize).unsigned_abs() + (y - b_y as isize).unsigned_abs(),
        }
    }

    fn row_coverage(&self, target_y: isize) -> Option<(isize, isize)> {
        let dif = (target_y - self.y).unsigned_abs();
        if dif <= self.radius {
            return Some((
                self.x - (self.radius - dif) as isize,
                self.x + (self.radius - dif) as isize,
            ));
        }
        None
    }
}

pub fn run() {
    println!("Day 15 of 2022");

    let timer_input_start = Instant::now();

    let file = File::open("./input/day_15").expect("no such file");
    let buf = BufReader::new(file);
    let input: Vec<Range> = buf.lines().map(|l| Range::new(&l.unwrap())).collect();

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

fn part_1(input: &[Range]) -> usize {
    let mut tmp: Vec<(isize, isize)> = input
        .iter()
        .filter_map(|r| r.row_coverage(2000000))
        .collect();
    tmp.sort_by(|a, b| (a.0).partial_cmp(&b.0).unwrap());

    let start = tmp[0];
    let (minimum, maximum) =
        tmp.iter()
            .skip(1)
            .fold(start, |(min_x, max_x), (new_min_x, new_max_x)| {
                if new_min_x <= &max_x {
                    (min_x, max(max_x, *new_max_x))
                } else {
                    panic!()
                }
            });

    (maximum - minimum).try_into().unwrap()
}

fn part_2(input: &[Range]) -> usize {
    for y in 0..4000000 {
        let mut tmp: Vec<(isize, isize)> = input.iter().filter_map(|r| r.row_coverage(y)).collect();
        tmp.sort_by(|a, b| (a.0).partial_cmp(&b.0).unwrap());

        let start = (tmp[0].0, tmp[0].1, 0);
        let (minimum, maximum, res) =
            tmp.iter()
                .skip(1)
                .fold(start, |(min_x, max_x, res), (new_min_x, new_max_x)| {
                    if new_min_x <= &max_x {
                        (min_x, max(max_x, *new_max_x), 0)
                    } else if res == 0 {
                        println!("x: {:?}, y: {:?}", max_x + 1, y);
                        (0, 0, (max_x + 1) * 4000000 + y)
                    } else {
                        (min_x, max_x, res)
                    }
                });
        if res != 0 {
            return res.try_into().unwrap();
        } else if (maximum - minimum) < 4000000 {
            println!("y: {:?}", y);
            return y.try_into().unwrap();
        }
    }
    0
}
