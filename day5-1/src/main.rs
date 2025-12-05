use std::{
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
    vec,
};

fn main() {
    println!("Day 5-1");
    println!("====================");
    let now = std::time::Instant::now();
    let mut timing_per_iteration = std::time::Duration::from_millis(0);
    let file = File::open("inputs-5-1.txt").unwrap();
    let reader = BufReader::new(file);
    let mut result = 0u32;
    let mut i = 0;

    let mut ranges = vec![];
    for line in reader.lines() {
        let iter = Instant::now();
        let line = line.unwrap();
        if line.is_empty() {
            continue;
        }
        if line.contains("-") {
            let range = Range::new(&line);
            ranges.push(range);
            continue;
        }

        let number = line.parse::<u128>().unwrap();
        if ranges.iter().any(|r| r.includes(number)) {
            result += 1;
        }

        timing_per_iteration += iter.elapsed();
        i += 1;
    }
    println!("Password: {}", result);
    let end = Instant::now();
    println!("Time: {:?}", end.duration_since(now));
    println!(
        "Iteration count: {} Timing per iteration: {:?}",
        i,
        timing_per_iteration / i
    );
}

pub struct Range {
    start_num: u128,
    end_num: u128,
}

impl Range {
    pub fn new(range: &str) -> Self {
        let parts: Vec<&str> = range.split("-").collect();
        let start_num = parts[0].parse::<u128>().unwrap();
        let end_num = parts[1].parse::<u128>().unwrap();
        Range { start_num, end_num }
    }

    pub fn includes(&self, value: u128) -> bool {
        value >= self.start_num && value <= self.end_num
    }
}
