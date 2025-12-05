use std::{
    collections::HashSet,
    fmt::Debug,
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
    vec,
};

fn main() {
    println!("Day 5-2");
    println!("====================");
    let now = std::time::Instant::now();
    let mut timing_per_iteration = std::time::Duration::from_millis(0);
    let file = File::open("inputs-5-2.txt").unwrap();
    let reader = BufReader::new(file);
    let mut result = 0u128;

    let mut ranges = vec![];
    for line in reader.lines() {
        let line = line.unwrap();
        if line.is_empty() {
            break;
        }
        if line.contains("-") {
            let range = Range::new(&line);
            ranges.push(range);
            continue;
        }
    }
    ranges.sort_by_key(|x| x.start_num);
    println!("Ranges count: {:?}", ranges.len());
    let mut marked_for_removal = HashSet::new();
    let mut iteration_count = 0;
    let mut final_ranges = vec![];

    let now_merge = Instant::now();
    for i in 0..ranges.len() {
        iteration_count += 1;
        let timing_now = Instant::now();
        if marked_for_removal.contains(&i) {
            continue;
        }
        let mut range = ranges[i].clone();

        for j in (i + 1)..ranges.len() {
            if marked_for_removal.contains(&j) {
                continue;
            }
            if range.includes_range(&ranges[j]) {
                marked_for_removal.insert(j);
            } else if range.overlaps(&ranges[j]) {
                marked_for_removal.insert(j);
                marked_for_removal.insert(i);
                range = range.merge(&ranges[j]);
            } else {
                break;
            }
        }
        final_ranges.push(range);
        timing_per_iteration += timing_now.elapsed();
    }
    let merge_time = now_merge.elapsed();

    let now_calc = Instant::now();
    for range in final_ranges.iter() {
        result += range.end_num - range.start_num + 1;
    }
    let calc_time = now_calc.elapsed();

    let end = Instant::now();
    println!("Result: {}", result);
    println!("Time: {:?}", end.duration_since(now));
    println!(
        "Iteration count: {} Timing per iteration: {:?} \nMerge duration {:?} result calc time: {:?}",
        iteration_count,
        timing_per_iteration / iteration_count,
        merge_time,
        calc_time
    );
}

#[derive(Clone)]
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

    pub fn includes_range(&self, other: &Range) -> bool {
        other.start_num >= self.start_num && other.end_num <= self.end_num
    }

    pub fn overlaps(&self, other: &Range) -> bool {
        !(other.end_num < self.start_num || other.start_num > self.end_num)
    }

    pub fn merge(&self, other: &Range) -> Range {
        let start_num = std::cmp::min(self.start_num, other.start_num);
        let end_num = std::cmp::max(self.end_num, other.end_num);
        Range { start_num, end_num }
    }
}

impl PartialEq for Range {
    fn eq(&self, other: &Self) -> bool {
        self.start_num == other.start_num && self.end_num == other.end_num
    }
}
impl Eq for Range {}

impl PartialOrd for Range {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.start_num.cmp(&other.start_num))
    }
}

impl Ord for Range {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.start_num.cmp(&other.start_num)
    }
}

impl Debug for Range {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}", self.start_num, self.end_num)
    }
}
