use std::{
    collections::{HashMap, VecDeque},
    fs::File,
    io::{prelude::*, BufReader},
    time::Instant,
};

use itertools::Itertools;

#[derive(Debug, Clone)]
struct ParsingNode {
    name: String,
    rate: usize,
    neighbors: Vec<String>,
}

impl ParsingNode {
    fn new(s: &str) -> Self {
        let parts: Vec<&str> = s.split(&['=', ';', ',', ' ']).collect();
        ParsingNode {
            name: parts[1].to_owned(),
            rate: parts[5].parse::<usize>().unwrap(),
            neighbors: parts[11..]
                .iter()
                .filter_map(|&s| {
                    if !s.is_empty() {
                        Some(s.to_owned())
                    } else {
                        None
                    }
                })
                .collect::<Vec<String>>(),
        }
    }
}

#[derive(Debug, Clone)]
struct Node {
    rate: usize,
    neighbors: HashMap<String, usize>,
}

pub fn run() {
    println!("Day 16 of 2022");

    let timer_input_start = Instant::now();

    let file = File::open("./input/day_16").expect("no such file");
    let buf = BufReader::new(file);
    let parsed_nodes: HashMap<String, ParsingNode> =
        buf.lines().fold(HashMap::new(), |mut map, l| {
            let node = ParsingNode::new(&l.unwrap());
            map.insert(node.name.clone(), node);
            map
        });

    let mut nodes: HashMap<String, Node> = HashMap::new();
    let target_nodes = parsed_nodes
        .values()
        .clone()
        .filter(|pn| pn.name == "AA" || pn.rate > 0);
    for node in target_nodes.clone() {
        nodes.insert(
            node.name.clone(),
            Node {
                rate: node.rate,
                neighbors: target_nodes.clone().filter(|tn| tn.rate > 0).fold(
                    HashMap::new(),
                    |mut nbs, target| {
                        if target.name != node.name {
                            let mut route_map: HashMap<String, String> = HashMap::new();
                            let mut visited: Vec<String> = vec![];
                            let mut to_visit: VecDeque<String> = VecDeque::new();
                            let mut current_id = node.name.clone();

                            while current_id != target.name {
                                for nb in parsed_nodes.get(&current_id).unwrap().neighbors.clone() {
                                    if !to_visit.contains(&nb) && !visited.contains(&nb) {
                                        route_map.insert(nb.clone(), current_id.clone());
                                        to_visit.push_back(nb.clone());
                                    }
                                }

                                visited.push(current_id.clone());
                                current_id = to_visit.pop_front().unwrap();
                            }

                            let mut route: VecDeque<String> = VecDeque::new();
                            let target_id = node.name.clone();
                            while current_id != target_id {
                                route.push_front(current_id.clone());
                                current_id = route_map.get(&current_id).unwrap().to_string();
                            }

                            nbs.insert(target.name.clone(), route.len() + 1);
                        }
                        nbs
                    },
                ),
            },
        );
    }
    println!("Input: {:?}", nodes);

    let timer_input_end = Instant::now();
    println!(
        "Input parsed in {:?}",
        timer_input_end.duration_since(timer_input_start)
    );

    let timer1_start = Instant::now();
    let result = part_1(nodes.clone());
    let timer1_end = Instant::now();
    println!(
        "Part 1: {} (in {:?})",
        result,
        timer1_end.duration_since(timer1_start)
    );

    let timer2_start = Instant::now();
    let result = part_2(nodes.clone());
    let timer2_end = Instant::now();
    println!(
        "Part 2: {} (in {:?})",
        result,
        timer2_end.duration_since(timer2_start)
    );
}

fn part_1(input: HashMap<String, Node>) -> usize {
    input
        .keys()
        .clone()
        .filter_map(|id| match id.as_str() {
            "AA" => None,
            _ => Some(id.clone()),
        })
        .permutations(input.len() - 1)
        .map(|perm| {
            let mut turn_count = 30_isize;
            let mut released_pressure = 0;
            turn_count -= *input.get("AA").unwrap().neighbors.get(&perm[0]).unwrap() as isize;
            released_pressure += turn_count as usize * input.get(&perm[0]).unwrap().rate;
            for step in perm.windows(2) {
                turn_count -= *input
                    .get(&step[0])
                    .unwrap()
                    .neighbors
                    .get(&step[1])
                    .unwrap() as isize;
                if turn_count <= 0 {
                    return released_pressure;
                }
                released_pressure += turn_count as usize * input.get(&step[1]).unwrap().rate;
            }

            released_pressure
        })
        .max()
        .unwrap()
}

fn part_2(input: HashMap<String, Node>) -> usize {
    0
}
