//^(\d+)\1+$
use std::{
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

fn main() {
    let now = std::time::Instant::now();
    let mut timing_per_iteration = std::time::Duration::from_millis(0);
    let mut iteration_count = 0;
    let file = File::open("inputs-3-1.txt").unwrap();
    let reader = BufReader::new(file);
    let mut result = 0u32;
    for battery in reader.lines() {
        let iter = Instant::now();
        let battery = battery.unwrap();
        let chars: Vec<char> = battery.chars().collect();
        let highest = chars[..chars.len() - 1].iter().max().unwrap();
        let index = chars.iter().enumerate().find(|x| x.1 == highest).unwrap().0;
        let second_highest = chars[index + 1..].iter().max().unwrap();
        let joltage = format!("{}{}", highest, second_highest)
            .parse::<u32>()
            .unwrap();
        result += joltage;
        timing_per_iteration += iter.elapsed();
        iteration_count += 1;
    }
    println!("Password: {}", result);
    let end = Instant::now();
    println!("Time: {:?}", end.duration_since(now));
    println!(
        "Timing per iteration: {:?}",
        timing_per_iteration / iteration_count
    );
}
