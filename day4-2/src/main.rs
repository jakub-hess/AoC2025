use std::{
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

fn main() {
    let now = std::time::Instant::now();
    let mut timing_per_iteration = std::time::Duration::from_millis(0);
    let file = File::open("inputs-4-2.txt").unwrap();
    let reader = BufReader::new(file);
    let mut result = 0u32;

    let file_load = Instant::now();
    let mut lines = reader
        .lines()
        .filter_map(Result::ok)
        .map(|x| x.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    println!("File load time: {:?}", file_load.elapsed());
    let lines_count = lines.len();

    let look_positions = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    let mut iteration_count = 0;
    loop {
        let mut removed_count = 0;
        for (i, line) in lines.clone().iter().enumerate() {
            let iter = Instant::now();
            //println!("{:?}", chars);
            for (j, c) in line.iter().enumerate() {
                //println!("Checking position ({}, {})", i, j);
                if *c == '.' {
                    continue;
                }
                let mut occupied_count = 0;
                for (dy, dx) in look_positions {
                    //println!("  Looking direction ({}, {})", dy, dx);

                    let ny = i as isize + dy;
                    let nx = j as isize + dx;
                    if ny < 0 || ny >= lines_count as isize || nx < 0 || nx >= line.len() as isize {
                        continue;
                    }
                    if lines[ny as usize][nx as usize] == '@' {
                        occupied_count += 1;
                    }
                    if occupied_count >= 4 {
                        break;
                    }
                }
                if occupied_count < 4 {
                    lines[i][j] = '.';
                    result += 1;
                    removed_count += 1;
                }
                //println!("  Occupied count: {}", occupied_count);
            }
            timing_per_iteration += iter.elapsed();
            iteration_count += 1;
        }
        println!("Removed this iteration: {}", removed_count);
        if removed_count == 0 {
            break;
        }
    }
    println!("Password: {}", result);
    let end = Instant::now();
    println!("Time: {:?}", end.duration_since(now));
    println!(
        "Iteration count: {} Timing per iteration: {:?}",
        iteration_count,
        timing_per_iteration / iteration_count
    );
}
