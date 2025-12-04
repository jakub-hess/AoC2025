//^(\d+)\1+$
use fancy_regex::Regex;
use std::{
    collections::HashSet,
    fs::File,
    io::{BufReader, Read},
    time::Instant,
};

fn main() {
    let now = std::time::Instant::now();
    println!("Day 2-2");
    println!("====================");
    println!("Naive solution:");
    let file = File::open("input-2-1.txt").unwrap();
    let mut reader = BufReader::new(file);
    let mut string_buf = String::new();
    reader.read_to_string(&mut string_buf).unwrap();
    let (result, checked_count, generated_count) = naive_solution(&string_buf);

    println!("Password: {}", result);
    println!("Checked count: {}", checked_count);
    println!("Generated count: {}", generated_count);
    let end = Instant::now();
    println!("Time: {:?}", end.duration_since(now));
    let now = std::time::Instant::now();
    println!("Day 2-2");
    println!("====================");
    println!("Range based solution:");
    let file = File::open("input-2-1.txt").unwrap();
    let mut reader = BufReader::new(file);
    let mut string_buf = String::new();
    reader.read_to_string(&mut string_buf).unwrap();
    let (result, checked_count, generated_count) = range_based_solution(&string_buf);

    println!("Password: {}", result);
    println!("Checked count: {}", checked_count);
    println!("Generated count: {}", generated_count);
    let end = Instant::now();
    println!("Time: {:?}", end.duration_since(now));
}

pub fn naive_solution(input: &str) -> (u128, u64, u64) {
    let mut result = 0u128;
    let re = Regex::new(r"^(\d+)\1+$").unwrap();
    let mut checked_count = 0u64;
    for range in input.split(",") {
        let range_split = range.split("-").collect::<Vec<&str>>();
        let start = range_split[0].parse::<u128>().unwrap();
        let end = range_split[1].parse::<u128>().unwrap();
        for i in start..=end {
            let s = i.to_string();
            checked_count += 1;
            if re.is_match(&s).unwrap() {
                result += i;
            }
        }
    }
    (result, checked_count, 0)
}

pub fn range_based_solution(input: &str) -> (u128, u64, u64) {
    let mut result = 0u128;
    let mut checked_count = 0u64;
    let mut generated_count = 0u64;
    //let input = "111-11111";
    for range in input.split(",") {
        let mut range_iter = Range::new(range);
        while let Some(i) = range_iter.next() {
            result += i as u128;
        }
        checked_count += range_iter.checked.len() as u64;
        generated_count += range_iter.generation_count;
    }
    (result, checked_count, generated_count)
}

#[derive(Debug)]
pub struct Range {
    start: String,
    start_num: u64,
    end_num: u64,
    pub checked: HashSet<u64>,
    substr_len: usize,
    repeat_count: u32,
    current_substr: String,
    pub generation_count: u64,
}

impl Range {
    pub fn new(range: &str) -> Self {
        //println!("Creating range for {}", range);
        let parts: Vec<&str> = range.split("-").collect();
        let start = parts[0];
        let end = parts[1];
        let end_num = end.parse::<u64>().unwrap();
        let res = Range {
            start: start.to_string(),
            start_num: start.parse::<u64>().unwrap(),
            end_num,
            checked: HashSet::new(),
            substr_len: 1,
            repeat_count: start.chars().count() as u32,
            current_substr: "1".to_string(),
            generation_count: 0,
        };
        res
    }

    fn next_candidate(&mut self) -> Option<u64> {
        loop {
            self.generation_count += 1;
            if self.substr_len > self.end_num.to_string().len() / 2 {
                return None;
            }
            let nex = self
                .current_substr
                .repeat(self.repeat_count as usize)
                .parse::<u64>()
                .unwrap();
            self.repeat_count += 1;

            if nex < self.start_num {
                continue;
            }

            if nex > self.end_num {
                self.current_substr = (self.current_substr.parse::<u64>().unwrap() + 1).to_string();
                self.substr_len = self.current_substr.chars().count();
                self.repeat_count = self.start.chars().count() as u32 / (self.substr_len as u32);
                if self.repeat_count == 1 {
                    self.repeat_count += 1;
                }
                // println!(
                //     "Increasing substr to {}, resetting repeat count to {}",
                //     self.current_substr, self.repeat_count
                // );
                continue;
            }
            // if current subst is 1 and only 0s with at least 1 zero

            if self.checked.contains(&nex) {
                // println!("Already checked {}, skipping", nex);
                continue;
            }
            self.checked.insert(nex);
            return Some(nex);
        }
    }
}

impl Iterator for Range {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let mut next_num = 0u64;
        while next_num < self.start_num {
            let next_num_candidate = self.next_candidate();
            if next_num_candidate.is_none() {
                return None;
            }
            next_num = next_num_candidate.unwrap();
        }
        //println!("Next candidate num: {}, ", next_num);
        Some(next_num)
    }
}
