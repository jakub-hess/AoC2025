//^(\d+)\1+$
use fancy_regex::Regex;
use std::{
    fs::File,
    io::{BufReader, Read},
    time::Instant,
};

fn main() {
    let now = std::time::Instant::now();
    let file = File::open("input-2-1-test.txt").unwrap();
    let mut reader = BufReader::new(file);
    let mut string_buf = String::new();
    reader.read_to_string(&mut string_buf).unwrap();
    let mut result = 0u128;
    let re = Regex::new(r"^(\d+)\1+$").unwrap();
    for range in string_buf.split(",") {
        let range_split = range.split("-").collect::<Vec<&str>>();
        let start = range_split[0].parse::<u128>().unwrap();
        let end = range_split[1].parse::<u128>().unwrap();
        for i in start..=end {
            let s = i.to_string();
            if re.is_match(&s).unwrap() {
                result += i;
            }
        }
    }
    println!("Password: {}", result);
    let end = Instant::now();
    println!("Time: {:?}", end.duration_since(now));
}
