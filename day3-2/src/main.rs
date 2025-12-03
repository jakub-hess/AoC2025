use std::{
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
    vec,
};

fn main() {
    let now = std::time::Instant::now();
    let file = File::open("inputs-3-2.txt").unwrap();
    let reader = BufReader::new(file);
    let mut result = 0u128;
    for battery in reader.lines() {
        let battery = battery.unwrap();
        let chars: Vec<(usize, char)> = battery.chars().enumerate().collect();
        let mut res = vec![];
        let mut lower_bound = 0;
        let mut upper_bound = battery.len() - 12;
        for _ in 0..12 {
            let highest = chars[lower_bound..=upper_bound]
                .iter()
                .max_by_key(|x| x.1)
                .map(|x| x.1)
                .unwrap();
            let index = chars[lower_bound..=upper_bound]
                .iter()
                .find(|x| x.1 == highest)
                .unwrap()
                .0;
            res.push((index, highest.clone()));
            lower_bound = index + 1;
            upper_bound = upper_bound + 1;
        }
        let mut res = res.iter().collect::<Vec<_>>();
        res.sort_by(|a, b| a.0.cmp(&b.0));
        println!("{:?}", res);
        let joltage = res
            .iter()
            .map(|x| x.1)
            .collect::<String>()
            .parse::<u128>()
            .unwrap();
        result += joltage;
    }
    println!("Password: {}", result);
    let end = Instant::now();
    println!("Time: {:?}", end.duration_since(now));
}
