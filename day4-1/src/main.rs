use std::{
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
    vec,
};

fn main() {
    let now = std::time::Instant::now();
    let mut timing_per_iteration = std::time::Duration::from_millis(0);
    let file = File::open("inputs-4-1.txt").unwrap();
    let reader = BufReader::new(file);
    let mut result = 0u32;
    let mut lines_peek = reader.lines().peekable();
    let mut i = 0;
    let look_positions = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1)];
    let future_position = [(1, -1), (1, 0), (1, 1)];
    let mut lines = vec![];
    while let Some(line) = lines_peek.next() {
        let iter = Instant::now();
        let line = line.unwrap();
        let chars: Vec<char> = line.chars().collect();
        //println!("{:?}", chars);
        lines.push(chars.clone());
        for (j, c) in chars.iter().enumerate() {
            //println!("Checking position ({}, {})", i, j);
            if *c == '.' {
                continue;
            }
            let mut occupied_count = 0;
            for (dy, dx) in look_positions {
                //println!("  Looking direction ({}, {})", dy, dx);

                let ny = i as isize + dy;
                let nx = j as isize + dx;
                if ny < 0 || nx < 0 || nx >= chars.len() as isize {
                    continue;
                }
                if lines[ny as usize][nx as usize] == '@' {
                    occupied_count += 1;
                }
                if occupied_count >= 4 {
                    break;
                }
            }
            let peeked_line = lines_peek.peek();
            if let Some(Ok(peeked_line)) = peeked_line
                && occupied_count < 4
            {
                let peeked_chars: Vec<char> = peeked_line.chars().collect();
                #[allow(unused_variables)]
                for (dy, dx) in future_position {
                    //println!("  Looking direction ({}, {})", dy, dx);
                    let nx = j as isize + dx;
                    if nx < 0 || nx >= chars.len() as isize {
                        continue;
                    }
                    if peeked_chars[nx as usize] == '@' {
                        occupied_count += 1;
                    }
                    if occupied_count >= 4 {
                        break;
                    }
                }
            }
            if occupied_count < 4 {
                result += 1;
            }
            //println!("  Occupied count: {}", occupied_count);
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
