use std::time::Instant;

pub fn run(input: Vec<String>) {
    println!("Day 2 of 2015");

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
    return input.iter().map(|s| {
        let mut dim: Vec<i32> = s.split("x").flat_map(|num|
            num.parse::<i32>()).collect();
        dim.sort();
        let l: i32 = dim[0];
        let w: i32 = dim[1];
        let h: i32 = dim[2];
        return 3*l*w + 2*w*h + 2*h*l;
    }).sum();
}

fn part_2(input: &Vec<String>) -> i32{
    return input.iter().map(|s| {
        let mut dim: Vec<i32> = s.split("x").flat_map(|num|
            num.parse::<i32>()).collect();
        dim.sort();
        let l: i32 = dim[0];
        let w: i32 = dim[1];
        let h: i32 = dim[2];
        return 2*w + 2*l + l*w*h;
    }).sum();
}