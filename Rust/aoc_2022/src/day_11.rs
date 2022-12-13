use std::{collections::VecDeque, time::Instant};

#[derive(Clone, Debug)]
struct Monkey {
    items: VecDeque<usize>,
    operation: fn(usize) -> usize,
    test: fn(usize) -> bool,
    target_true: usize,
    target_false: usize,
    handled: usize,
}

impl Monkey {
    fn new(
        items: VecDeque<usize>,
        operation: fn(usize) -> usize,
        test: fn(usize) -> bool,
        target_true: usize,
        target_false: usize,
    ) -> Self {
        Self {
            items,
            operation,
            test,
            target_true,
            target_false,
            handled: 0,
        }
    }

    fn handle(&mut self) {
        self.handled += 1;
    }
}

pub fn run() {
    println!("Day 11 of 2022");

    let timer_input_start = Instant::now();

    let mut input: [Monkey; 8] = [
        Monkey::new(
            VecDeque::from([83, 62, 93]),
            |val| val * 17,
            |val| val % 2 == 0,
            1,
            6,
        ),
        Monkey::new(
            VecDeque::from([90, 55]),
            |val| val + 1,
            |val| val % 17 == 0,
            6,
            3,
        ),
        Monkey::new(
            VecDeque::from([91, 78, 80, 97, 79, 88]),
            |val| val + 3,
            |val| val % 19 == 0,
            7,
            5,
        ),
        Monkey::new(
            VecDeque::from([64, 80, 83, 89, 59]),
            |val| val + 5,
            |val| val % 3 == 0,
            7,
            2,
        ),
        Monkey::new(
            VecDeque::from([98, 92, 99, 51]),
            |val| val * val,
            |val| val % 5 == 0,
            0,
            1,
        ),
        Monkey::new(
            VecDeque::from([68, 57, 95, 85, 98, 75, 98, 75]),
            |val| val + 2,
            |val| val % 13 == 0,
            4,
            0,
        ),
        Monkey::new(
            VecDeque::from([74]),
            |val| val + 4,
            |val| val % 7 == 0,
            3,
            2,
        ),
        Monkey::new(
            VecDeque::from([68, 64, 60, 68, 87, 80, 82]),
            |val| val * 19,
            |val| val % 11 == 0,
            4,
            5,
        ),
    ];

    let mut input2 = input.clone();

    let timer_input_end = Instant::now();
    println!(
        "Input parsed in {:?}",
        timer_input_end.duration_since(timer_input_start)
    );

    let timer1_start = Instant::now();
    let result = part_1(&mut input);
    let timer1_end = Instant::now();
    println!(
        "Part 1: {} (in {:?})",
        result,
        timer1_end.duration_since(timer1_start)
    );

    let timer2_start = Instant::now();
    let result = part_2(&mut input2);
    let timer2_end = Instant::now();
    println!(
        "Part 2: {} (in {:?})",
        result,
        timer2_end.duration_since(timer2_start)
    );
}

fn part_1(input: &mut [Monkey; 8]) -> usize {
    for _ in 0..20 {
        for i in 0..input.len() {
            let mut items = input[i].items.clone();
            input[i].items.clear();
            while !items.is_empty() {
                input[i].handle();
                let item = items.pop_front().unwrap();
                let val = (input[i].operation)(item) / 3;
                if (input[i].test)(val) {
                    input[input[i].target_true].items.push_back(val);
                } else {
                    input[input[i].target_false].items.push_back(val);
                }
            }
        }
    }
    input
        .clone()
        .map(|m| m.handled)
        .iter()
        .fold([0; 2], |mut acc, elem| {
            if elem > &acc[0] {
                acc[1] = acc[0];
                acc[0] = *elem;
            } else if elem > &acc[1] {
                acc[1] = *elem;
            }
            acc
        })
        .iter()
        .product::<usize>()
}

fn part_2(input: &mut [Monkey; 8]) -> usize {
    for _ in 0..10000 {
        for i in 0..input.len() {
            let mut items = input[i].items.clone();
            input[i].items.clear();
            while !items.is_empty() {
                input[i].handle();
                let item = items.pop_front().unwrap();
                let val = (input[i].operation)(item) % 9699690;
                if (input[i].test)(val) {
                    input[input[i].target_true].items.push_back(val);
                } else {
                    input[input[i].target_false].items.push_back(val);
                }
            }
        }
    }
    input
        .clone()
        .map(|m| m.handled)
        .iter()
        .fold([0; 2], |mut acc, elem| {
            if elem > &acc[0] {
                acc[1] = acc[0];
                acc[0] = *elem;
            } else if elem > &acc[1] {
                acc[1] = *elem;
            }
            acc
        })
        .iter()
        .product::<usize>()
}
