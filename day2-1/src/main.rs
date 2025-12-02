use std::{
    fs::File,
    io::{BufReader, Read},
};

fn main() {
    let now = std::time::Instant::now();
    let file = File::open("input-2-1-test.txt").unwrap();
    let mut reader = BufReader::new(file);
    let mut string_buf = String::new();
    reader.read_to_string(&mut string_buf).unwrap();
    let mut result = 0u64;
    for range in string_buf.split(",") {
        for i in Range::new(range) {
            result += i;
        }
    }
    println!("Password: {}", result);
    let end = std::time::Instant::now();
    println!("Time: {:?}", end.duration_since(now));
}

#[derive(Debug)]
pub struct Range {
    start: String,
    start_num: u64,
    current: String,
    end_num: u64,
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
            current: "".to_string(),
            end_num,
        };
        res
    }
}

impl Iterator for Range {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let mut next_num = 0u64;
        while next_num < self.start_num {
            let start = self.current.is_empty();
            if start {
                self.current = self.start.clone();
            }
            let current_len = self.current.len();
            let len_even = current_len % 2 == 0;
            let next_half = if len_even {
                let len = current_len / 2;
                let mut res = self.current[..len].to_string();
                if !start {
                    res = (res.parse::<u64>().unwrap() + 1).to_string();
                }

                res
            } else {
                let len = current_len / 2;
                ("1".to_string() + String::from_utf8(vec![b'0'; len]).unwrap().as_str()).to_string()
            };

            let next = next_half.clone() + &next_half;

            next_num = next.parse::<u64>().unwrap();
            self.current = next.clone();
            //println!("Next candidate num: {}, ", next_num);
        }

        if next_num > self.end_num {
            None
        } else {
            //println!("Returning num: {}, ", self.current);
            Some(next_num)
        }
    }
}
