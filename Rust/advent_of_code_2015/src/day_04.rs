use std::time::Instant;

extern crate crypto;

use crypto::md5::Md5;
use crypto::digest::Digest;

//https://gist.github.com/gkbrk/2e4835e3a17b3fb6e1e7
pub fn run(input: Vec<String>) {
    println!("Day 4 of 2015");

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

fn part_1(input: &Vec<String>) -> u64 {
    let mut hasher = Md5::new();
    let key = input[0].as_bytes();
    
    let mut output = [0; 16]; // An MD5 is 16 bytes
    for i in 0..std::u64::MAX{
        hasher.input(key);
        hasher.input(i.to_string().as_bytes());
        
        hasher.result(&mut output);

        if output[..2] == [0, 0] && output[2] <= 0x0F {
            return i;
        }
        hasher.reset();        
    }
    return 0;
}

fn part_2(input: &Vec<String>) -> u64 {
    let mut hasher = Md5::new();
    let key = input[0].as_bytes();
    
    let mut output = [0; 16]; // An MD5 is 16 bytes
    for i in 0..std::u64::MAX{
        hasher.input(key);
        hasher.input(i.to_string().as_bytes());
        
        hasher.result(&mut output);

        if output[..3] == [0, 0, 0] {
            return i;
        }
        hasher.reset();        
    }
    return 0;
}
