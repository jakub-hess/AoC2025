use std::{
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
    vec,
};

fn main() {
    println!("Day 6-2");
    println!("====================");
    let now = Instant::now();
    let mut timing_per_iteration = std::time::Duration::from_millis(0);
    let file_read = Instant::now();
    let file = File::open("inputs-6.txt").unwrap();
    let reader = BufReader::new(file);

    let mut lines = vec![];
    for line in reader.lines() {
        let line = line.unwrap().chars().rev().collect::<Vec<char>>();
        lines.push(line);
    }
    let file_read_duration = file_read.elapsed();
    println!("File read time: {:?}", file_read_duration);

    let mut iteration_count = 0;
    let mut operation_line = lines.last().unwrap().iter().enumerate();
    let mut op = (0, ' ');
    let mut final_result = 0u128;
    let op_start = Instant::now();
    let lines_range = 0..lines.len() - 1;

    loop {
        let iter_start = Instant::now();
        iteration_count += 1;
        let end = operation_line.find(|x| *x.1 == '+' || *x.1 == '*');
        if end.is_none() {
            break;
        }
        let end = end.unwrap();
        let range = op.0..=end.0;
        let local_op = end.1;
        //let mut result = None;
        //println!("Operation: {} on range {:?}", local_op, range);

        let mut result = None;
        for i in range.clone() {
            let number = lines[lines_range.clone()]
                .iter()
                .map(|line| line[i])
                .collect::<String>();
            //println!("Segment: {:?}", number);
            let number = Number::new(number);
            match result {
                None => {
                    result = Some(number);
                }
                Some(prev) => {
                    result = Some(match local_op {
                        '+' => prev.add_v1(number),
                        '*' => prev.multiply_v1(number),
                        _ => panic!("Unknown operation"),
                    });
                }
            }
        }
        let result: u128 = result.unwrap().into();
        //println!("Intermediate result: {}", result);
        final_result += result;

        //let result: u128 = result.unwrap().into();
        //println!("Intermediate result: {}", result);
        //final_result += result;

        op = (end.0 + 2, *end.1);
        timing_per_iteration += iter_start.elapsed();
    }
    let op_duration = op_start.elapsed();
    println!("Operation time: {:?}", op_duration);

    println!("Lines count: {:?}", lines.len());

    println!("Result: {}", final_result);
    println!("Time: {:?}", now.elapsed());
    println!(
        "Average time per iteration: {:?}, iterations: {}",
        timing_per_iteration / iteration_count,
        iteration_count
    );
}

pub struct Number {
    digits: u128,
}

impl Number {
    pub fn new(digits: String) -> Self {
        Self {
            digits: digits.trim().parse::<u128>().unwrap(),
        }
    }

    pub fn new_from_u128(digits: u128) -> Self {
        Self { digits }
    }

    pub fn add_v1(&self, other: Number) -> Number {
        let own = self.digits;
        let other = other.digits;
        let sum = own + other;
        Number::new_from_u128(sum)
    }

    pub fn multiply_v1(&self, other: Number) -> Number {
        let own = self.digits;
        let other = other.digits;
        let product = own * other;
        Number::new_from_u128(product)
    }
}

impl Into<u128> for Number {
    fn into(self) -> u128 {
        self.digits
    }
}
