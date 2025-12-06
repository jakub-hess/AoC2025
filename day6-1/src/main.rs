use std::{
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
    vec,
};

fn main() {
    println!("Day 6-1");
    println!("====================");
    let now = Instant::now();
    let mut timing_per_iteration = std::time::Duration::from_millis(0);
    let file_read = Instant::now();
    let file = File::open("inputs-6.txt").unwrap();
    let reader = BufReader::new(file);

    let mut lines = vec![];
    for line in reader.lines() {
        let line = line.unwrap();
        lines.push(line);
    }
    let file_read_duration = file_read.elapsed();
    println!("File read time: {:?}", file_read_duration);

    let mut iteration_count = 0;
    let mut operation_line = lines.last().unwrap().chars().enumerate();
    let mut op = operation_line.find(|x| x.1 == '+' || x.1 == '*').unwrap();
    let mut stop_condition = false;
    let mut final_result = 0u128;
    let op_start = Instant::now();

    loop {
        let iter_start = Instant::now();
        iteration_count += 1;
        let mut end = operation_line.find(|x| x.1 == '+' || x.1 == '*');
        let mut remove_trailing = 1;
        if end.is_none() {
            end = Some((lines.last().unwrap().len(), ' '));
            stop_condition = true;
            remove_trailing = 0;
        }
        let end = end.unwrap();
        let range = op.0..end.0 - remove_trailing;
        let local_op = op.1;
        let mut result = None;
        for line in &lines[..lines.len() - 1] {
            let segment = &line[range.clone()];
            let number = Number::new(segment.to_string());
            match result {
                None => {
                    result = Some(number);
                }
                Some(acc) => {
                    result = Some(match local_op {
                        '+' => acc.add_v1(number),
                        '*' => acc.multiply_v1(&number),
                        _ => panic!("Unknown operation"),
                    });
                }
            }
            //println!("Segment: {:?}", segment);
        }

        let result: u128 = result.unwrap().into();
        //println!("Intermediate result: {}", result);
        final_result += result;

        op = end;
        if stop_condition {
            break;
        }
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

    pub fn multiply_v1(&self, other: &Number) -> Number {
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
