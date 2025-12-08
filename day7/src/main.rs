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
    let rows = lines.len();
    let cols = lines[0].len();
    //let mut splitter_positions = vec![];

    for row in 1..rows {
        for col in 0..cols {
            let current_pos = lines[row][col];
            let above = lines[row - 1][col];
            match (current_pos, above) {
                (Position::Empty, Position::Beam(_)) => {
                    let above = lines[row - 1][col];
                    let new_pos = above.add(&current_pos);
                    lines[row][col] = new_pos;
                }
                (Position::Empty, Position::Start) => {
                    lines[row][col] = Position::Beam(1);
                }
                (Position::Splitter(_), Position::Beam(_)) => {
                    let left = lines[row][col - 1];
                    let right = lines[row][col + 1];
                    lines[row][col - 1] = left.add(&above);
                    lines[row][col + 1] = right.add(&above);
                    final_result += 1;
                }
                (Position::Beam(_), Position::Beam(_)) => {
                    let above = lines[row - 1][col];
                    let new_pos = above.add(&current_pos);
                    lines[row][col] = new_pos;
                }
                _ => {}
            }
        }
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
