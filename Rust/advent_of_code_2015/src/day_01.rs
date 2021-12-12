use std::time::Instant;

pub fn run(input: Vec<String>) {
    println!("Day 1 of 2015");

    let timer1_start = Instant::now();
    let result = part_1(&input);
    let timer1_end = Instant::now();
    println!("Part 1: {} (in {:?})", result, timer1_end.duration_since(timer1_start));


    let timer2_start = Instant::now();
    let result = part_2(&input);
    let timer2_end = Instant::now();
    println!("Part 2: {} (in {:?})", result, timer2_end.duration_since(timer2_start));

}

fn part_1(input: &Vec<String>) -> i32{
    return input[0].chars().map(|ch| {
        match ch {
            '(' => 1,
            ')' => -1,
            _ => panic!("invalid character!"),
        }
    }).sum();
}

fn part_2(input: &Vec<String>) -> usize{
    let mut floor = 0;
    return 1 + input[0].chars().map(|ch| {
        match ch {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => panic!("invalid character!"),
        }
        floor
    }).position(|step| step < 0).unwrap();
}