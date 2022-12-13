use std::{
    cmp::Ordering,
    fmt,
    fs::File,
    io::{prelude::*, BufReader},
    time::Instant,
};

#[derive(Clone, Debug, Eq)]
struct Part {
    value: isize,
    children: Vec<Part>,
}

impl PartialEq for Part {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
            && self.children.len() == other.children.len()
            && (0..self.children.len()).all(|i| self.children[i] == other.children[i])
    }
}

impl PartialOrd for Part {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.value != -1 && other.value != -1 {
            self.value.partial_cmp(&other.value)
        } else if self.value == -1 && other.value == -1 {
            for i in 0..self.children.len() {
                if i < other.children.len() {
                    let res = self.children[i].partial_cmp(&other.children[i]);
                    if res != Some(Ordering::Equal) {
                        return res;
                    }
                } else {
                    return Some(Ordering::Greater);
                }
            }
            if self.children.len() == other.children.len() {
                Some(Ordering::Equal)
            } else {
                Some(Ordering::Less)
            }
        } else if self.value == -1 {
            self.partial_cmp(&Part {
                value: -1,
                children: vec![other.clone()],
            })
        } else {
            Part {
                value: -1,
                children: vec![self.clone()],
            }
            .partial_cmp(other)
        }
    }
}

impl Ord for Part {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.value != -1 && other.value != -1 {
            self.value.partial_cmp(&other.value).unwrap()
        } else if self.value == -1 && other.value == -1 {
            for i in 0..self.children.len() {
                if i < other.children.len() {
                    let res = self.children[i].partial_cmp(&other.children[i]).unwrap();
                    if res != Ordering::Equal {
                        return res;
                    }
                } else {
                    return Ordering::Greater;
                }
            }
            if self.children.len() == other.children.len() {
                Ordering::Equal
            } else {
                Ordering::Less
            }
        } else if self.value == -1 {
            self.partial_cmp(&Part {
                value: -1,
                children: vec![other.clone()],
            })
            .unwrap()
        } else {
            Part {
                value: -1,
                children: vec![self.clone()],
            }
            .partial_cmp(other)
            .unwrap()
        }
    }
}

impl fmt::Display for Part {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        if self.is_list() {
            let mut s = "[".to_owned();
            for child in &self.children {
                s.push_str(&format!("{},", child));
            }
            let c = s.pop().unwrap();
            if c == '[' {
                s.push(c);
            }

            s.push(']');
            write!(f, "{}", s)
        } else {
            write!(f, "{}", self.value)
        }
    }
}

impl Part {
    fn new(origin: String) -> Self {
        if origin.chars().all(char::is_numeric) {
            Self {
                value: origin.parse::<isize>().unwrap(),
                children: vec![],
            }
        } else {
            let mut children: Vec<Part> = Vec::new();

            let to_parse = &origin[1..(origin.len() - 1)];

            let mut start_index = 0;
            let mut bracket_count = 0;

            for (i, char) in to_parse.chars().enumerate() {
                if bracket_count == 0 {
                    if char == ',' {
                        children.push(Part::new(to_parse[start_index..i].to_owned()));
                        start_index = i + 1;
                    } else if char == '[' {
                        bracket_count += 1;
                    }
                } else if char == '[' {
                    bracket_count += 1;
                } else if char == ']' {
                    bracket_count -= 1;
                }
            }
            if start_index < to_parse.len() {
                children.push(Part::new(to_parse[start_index..].to_owned()));
            }

            Self {
                value: -1,
                children,
            }
        }
    }

    fn is_list(&self) -> bool {
        self.value == -1
    }
}

pub fn run() {
    println!("Day 13 of 2022");

    let timer_input_start = Instant::now();

    let file = File::open("./input/day_13").expect("no such file");
    let buf = BufReader::new(file);
    let input: Vec<(Part, Part)> = buf
        .lines()
        .map(|l| l.unwrap())
        .collect::<Vec<String>>()
        .chunks(3)
        .map(|chunk| (Part::new(chunk[0].clone()), Part::new(chunk[1].clone())))
        .collect();
    // for (p1, p2) in &input {
    //     println!("Part1: {}", p1);
    //     println!("Part2: {}", p2);
    // }

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

fn part_1(input: &[(Part, Part)]) -> usize {
    let mut sum = 0;
    for (i, (p1, p2)) in input.iter().enumerate() {
        if p1 <= p2 {
            sum += i + 1;
        }
    }
    sum
}

fn part_2(input: &[(Part, Part)]) -> usize {
    let p2 = Part::new("[[2]]".to_owned());
    let p6 = Part::new("[[6]]".to_owned());
    let mut sorted: Vec<&Part> = input.iter().flat_map(|(p1, p2)| [p1, p2]).collect();
    sorted.sort();
    sorted
        .iter()
        .enumerate()
        .fold(1_usize, |product, (index, part)| {
            if part == &&p2 || part == &&p6 {
                return product * (index + 1);
            }
            product
        })
}
