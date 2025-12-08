use std::{
    collections::VecDeque,
    fmt::Debug,
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
    vec,
};

fn main() {
    println!("Day 6-2");
    println!("====================");
    let now = Instant::now();
    let file_read = Instant::now();
    let file = File::open("inputs-7.txt").unwrap();
    let reader = BufReader::new(file);

    let mut lines = vec![];
    let mut position_stack = VecDeque::with_capacity(128);
    for (index, line) in reader.lines().enumerate() {
        let line = line
            .unwrap()
            .chars()
            .enumerate()
            .map(|(j, c)| {
                if c == '.' {
                    Position::Empty
                } else if c == 'S' {
                    position_stack.push_back((index, j));
                    Position::Start
                } else if c == '^' {
                    Position::Splitter(0)
                } else {
                    panic!("Unknown character {} at line {}, index {}", c, index, j);
                }
            })
            .collect::<Vec<Position>>();
        lines.push(line);
    }
    let file_read_duration = file_read.elapsed();
    println!("File read time: {:?}", file_read_duration);
    println!("starting positions: {:?}", position_stack);

    let mut final_result = 0u128;
    let op_start = Instant::now();
    let mut iteration_count = 0;
    let mut new_positions = vec![];
    //let mut splitter_positions = vec![];

    while let Some((row, col)) = position_stack.pop_front() {
        //println!("Processing position: {:?}", (row, col));
        // if row >= lines.len() || col >= lines[row].len() {
        //     println!("Out of bounds position at ({}, {})", row, col);
        //     continue;
        // }
        iteration_count += 1;

        match lines[row][col] {
            Position::Start => {
                println!("At start position at ({}, {})", row, col);
                if row + 1 < lines.len() {
                    new_positions.push((row + 1, col)); // continue down
                }
            }
            Position::Empty => {
                println!("At empty position at ({}, {})", row, col);
                lines[row][col] = lines[row][col].add(&lines[row - 1][col]);
                if row + 1 < lines.len() {
                    new_positions.push((row + 1, col)); // continue down
                }
            }
            Position::Splitter(_) => {
                println!("At splitter position at ({}, {})", row, col);
                if row + 1 < lines.len() {
                    position_stack.push_back((row, col - 1)); // split left
                    lines[row][col - 1] = lines[row][col - 1].add(&lines[row - 1][col]);
                    position_stack.push_back((row, col + 1)); // split right
                    lines[row][col + 1] = lines[row][col + 1].add(&lines[row - 1][col]);
                }
                final_result += 1;
            }
            Position::Beam(_) => {
                println!("At beam position at ({}, {})", row, col);
                if row + 1 < lines.len()
                    && new_positions
                        .iter()
                        .find(|(new_row, new_col)| *new_row == row + 1 && *new_col == col)
                        .is_none()
                {
                    new_positions.push((row + 1, col)); // continue down
                }
            }
            _ => {
                //println!("At other position at ({}, {})", row, col);
            }
        }
        //print_grid(&lines);

        if position_stack.is_empty() {
            position_stack.extend(new_positions.drain(..));
        }
        // if iteration_count == 200 {
        //     break;
        // }
    }

    let result_p2 = lines
        .last()
        .unwrap()
        .iter()
        .filter_map(|pos| {
            if let Position::Beam(strength) = pos {
                Some(*strength as u128)
            } else {
                None
            }
        })
        .sum::<u128>();
    let op_duration = op_start.elapsed();
    println!("Operation time: {:?}", op_duration);
    //println!("Last line: {:?}", lines.last().unwrap());

    println!("Result P1: {}", final_result);
    println!("Result P2: {}", result_p2);
    println!("Time: {:?}", now.elapsed());
}

fn print_grid(grid: &Vec<Vec<Position>>) {
    for row in grid {
        for pos in row {
            print!("{:?} ", pos);
        }
        println!();
    }
}

#[derive(Clone, Copy)]
pub enum Position {
    Empty,
    Start,
    Splitter(u128),
    Beam(u128),
}

impl Position {
    pub fn add(&self, other: &Position) -> Position {
        match (self, other) {
            (Position::Beam(s1), Position::Beam(s2)) => Position::Beam(*s1 + *s2),
            (Position::Empty, Position::Beam(s)) => Position::Beam(*s),
            (Position::Beam(s), Position::Empty) => Position::Beam(*s),
            (Position::Empty, Position::Start) => Position::Beam(1),
            (Position::Empty, Position::Splitter(s)) => Position::Beam(*s),
            (Position::Splitter(s), Position::Empty) => Position::Beam(*s),
            (Position::Beam(s1), Position::Splitter(s2)) => Position::Beam(*s1 + *s2),
            (Position::Splitter(s1), Position::Beam(s2)) => Position::Beam(*s1 + *s2),
            _ => Position::Empty,
        }
    }
}

impl Debug for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Position::Empty => write!(f, "."),
            Position::Start => write!(f, "S"),
            Position::Splitter(_) => write!(f, "^"),
            Position::Beam(strength) => write!(f, "{:X}", strength),
        }
    }
}
