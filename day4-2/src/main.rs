use std::{
    fs::File,
    io::{BufRead, BufReader},
    process::Command,
    thread::sleep,
    time::Instant,
};
fn main() {
    let visualize = false;
    println!("Day 4-2");
    println!("====================");
    println!("Naive solution:");
    let now = std::time::Instant::now();

    let file = File::open("inputs-4-2.txt").unwrap();
    let reader = BufReader::new(file);

    let file_load = Instant::now();
    let mut lines = reader
        .lines()
        .filter_map(Result::ok)
        .map(|x| x.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    println!("File load time: {:?}", file_load.elapsed());

    let (result, timing_per_iteration, iteration_count) = naive_solution(&mut lines, visualize);

    println!("Password: {}", result);
    let end = Instant::now();
    println!("Time: {:?}", end.duration_since(now));
    println!(
        "Iteration count: {} Timing per iteration: {:?}",
        iteration_count,
        timing_per_iteration / iteration_count
    );
    println!("====================");
    println!("Flood solution:");
    let now = std::time::Instant::now();

    let file = File::open("inputs-4-2.txt").unwrap();
    let reader = BufReader::new(file);

    let file_load = Instant::now();
    let mut lines = reader
        .lines()
        .filter_map(Result::ok)
        .map(|x| x.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    println!("File load time: {:?}", file_load.elapsed());

    let (result, timing_per_iteration, iteration_count) = flood_solution(&mut lines, visualize);

    println!("Password: {}", result);
    let end = Instant::now();
    println!("Time: {:?}", end.duration_since(now));
    println!(
        "Iteration count: {} Timing per iteration: {:?}",
        iteration_count,
        timing_per_iteration / iteration_count
    );
}

pub fn flood_solution(
    input: &mut Vec<Vec<char>>,
    visualize: bool,
) -> (u32, std::time::Duration, u32) {
    let mut result = 0u32;
    let mut timing_per_iteration = std::time::Duration::from_millis(0);
    let mut iteration_count = 0;
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
    let lines_count = input.len();
    let line_length = input[0].len();
    let mut looked_positions = 0u64;
    let (tx, rx) = std::sync::mpsc::channel();

    for i in 0..lines_count {
        let now = Instant::now();
        for j in 0..line_length {
            looked_positions += 1;
            if input[i][j] == '.' {
                continue;
            }
            if check_occupied_position(
                input,
                i,
                j,
                &look_positions,
                lines_count,
                line_length,
                &tx,
                visualize,
            ) {
                result += 1;
            }
            while let Ok((ny, nx)) = rx.try_recv() {
                looked_positions += 1;
                if input[ny][nx] == '.' {
                    continue;
                }
                if check_occupied_position(
                    input,
                    ny,
                    nx,
                    &look_positions,
                    lines_count,
                    line_length,
                    &tx,
                    visualize,
                ) {
                    result += 1;
                }
            }
        }
        timing_per_iteration += now.elapsed();
        iteration_count += 1;
    }
    println!("Looked positions: {}", looked_positions);
    (result, timing_per_iteration, iteration_count)
}

pub fn check_occupied_position(
    input: &mut Vec<Vec<char>>,
    y: usize,
    x: usize,
    look_positions: &[(isize, isize)],
    lines_count: usize,
    line_length: usize,
    tx: &std::sync::mpsc::Sender<(usize, usize)>,
    visualize: bool,
) -> bool {
    let mut occupied_neighbors = vec![];
    for (dy, dx) in look_positions {
        let ny = y as isize + dy;
        let nx = x as isize + dx;
        if ny < 0 || ny >= lines_count as isize || nx < 0 || nx >= line_length as isize {
            continue;
        }
        if input[ny as usize][nx as usize] == '@' {
            occupied_neighbors.push((ny as usize, nx as usize));
        }
        if occupied_neighbors.len() >= 4 {
            break;
        }
    }
    if occupied_neighbors.len() < 4 {
        input[y][x] = '.';
        for (ny, nx) in occupied_neighbors {
            tx.send((ny, nx)).unwrap();
        }
        if visualize {
            let full_string = input
                .iter()
                .map(|line| line.iter().collect::<String>())
                .collect::<Vec<String>>()
                .join("\n");
            sleep(std::time::Duration::from_millis(10));
            Command::new("clear").status().unwrap();
            Command::new("clear").status().unwrap();
            Command::new("clear").status().unwrap();
            println!("{}\n", full_string);
        }
        true
    } else {
        false
    }
}

pub fn naive_solution(
    input: &mut Vec<Vec<char>>,
    visualize: bool,
) -> (u32, std::time::Duration, u32) {
    let mut result = 0u32;
    let mut timing_per_iteration = std::time::Duration::from_millis(0);

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
    let lines_count = input.len();
    let mut iteration_count = 0;
    let mut looked_positions = 0u64;
    loop {
        let mut removed_count = 0;
        // clear console
        for (i, line) in input.clone().iter().enumerate() {
            let iter = Instant::now();
            //println!("{:?}", chars);
            for (j, c) in line.iter().enumerate() {
                //println!("Checking position ({}, {})", i, j);
                looked_positions += 1;
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
                    if input[ny as usize][nx as usize] == '@' {
                        occupied_count += 1;
                    }
                    if occupied_count >= 4 {
                        break;
                    }
                }
                if occupied_count < 4 {
                    input[i][j] = '.';
                    result += 1;
                    removed_count += 1;
                    if visualize {
                        let full_string = input
                            .iter()
                            .map(|line| line.iter().collect::<String>())
                            .collect::<Vec<String>>()
                            .join("\n");
                        sleep(std::time::Duration::from_millis(10));
                        Command::new("clear").status().unwrap();
                        Command::new("clear").status().unwrap();
                        Command::new("clear").status().unwrap();
                        println!("{}\n", full_string);
                    }
                }
                //println!("  Occupied count: {}", occupied_count);
            }
            timing_per_iteration += iter.elapsed();
            iteration_count += 1;
        }
        //println!("Removed this iteration: {}", removed_count);
        if removed_count == 0 {
            break;
        }
    }

    println!("Looked positions: {}", looked_positions);
    (result, timing_per_iteration, iteration_count)
}
