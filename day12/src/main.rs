use std::{
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
    vec,
};

fn main() {
    println!("Day 12");
    println!("====================");
    let now = Instant::now();
    let file_read = Instant::now();
    let file = File::open("inputs-12.txt").unwrap();
    let reader = BufReader::new(file);

    let mut presents = vec![];
    let mut parse_state = ParseState::Start;
    let mut current_present = vec![];
    let mut areas = vec![];
    for line in reader.lines() {
        let line = line.unwrap();
        let line_split = line.split(":").collect::<Vec<_>>();
        match parse_state {
            ParseState::Start => {
                if line_split[1].is_empty() {
                    parse_state = ParseState::Present
                }
            }
            ParseState::Present => {
                if line_split[0].is_empty() {
                    parse_state = ParseState::None;
                    presents.push(current_present);
                    current_present = vec![];
                    continue;
                }
                current_present.push(line_split[0].chars().collect::<Vec<_>>());
            }
            ParseState::None => {
                if line_split[1].is_empty() {
                    parse_state = ParseState::Present
                } else {
                    parse_state = ParseState::Area;
                    let area = Area::new(line_split[0], line_split[1]);
                    areas.push(area);
                }
            }
            ParseState::Area => {
                let area = Area::new(line_split[0], line_split[1]);
                areas.push(area);
            }
        }
    }
    let file_read_duration = file_read.elapsed();
    println!("File read time: {:?}", file_read_duration);

    let mut field_counts = vec![];

    //println!("Machines: {:?}", machines);

    let op_start = Instant::now();
    for (_, present) in presents.iter().enumerate() {
        //println!("Present {}:", i);
        let mut field_count = 0;
        for row in present {
            field_count += row.iter().filter(|&&c| c == '#').count();
            //println!("{}", row_str);
        }
        field_counts.push(field_count);
    }

    let mut possible_areas = 0;
    for area in areas.iter() {
        //println!("Area: {:?}, Required Presents: {:?}", area.area, area.required_presents);
        let mut total_field_count = 0;
        for (i, present_count) in area.required_presents.iter().enumerate() {
            if let Some(&count) = field_counts.get(i) {
                total_field_count += count * present_count;
            }
        }
        if total_field_count <= area.get_area() {
            possible_areas += 1;
            //println!("Area {:?} can hold all required presents.", area.area);
        }
        // println!(
        //     "Area {:?} with area of {} requires presents {:?} with total field count: {}",
        //     area.area,
        //     area.get_area(),
        //     area.required_presents,
        //     total_field_count
        // );
    }
    //println!("Field counts per present: {:?}", field_counts);
    let op_duration = op_start.elapsed();
    println!("P1 time: {:?}", op_duration);
    println!("Result P1: {:?}", possible_areas);

    println!("Time: {:?}", now.elapsed());
}

pub enum ParseState {
    Start,
    Present,
    None,
    Area,
}

pub struct Area {
    pub area: (usize, usize),
    pub required_presents: Vec<usize>,
}

impl Area {
    pub fn new(area: &str, required_presents: &str) -> Self {
        let area_split = area.split('x').collect::<Vec<&str>>();
        let area = (
            area_split[0].parse::<usize>().unwrap(),
            area_split[1].parse::<usize>().unwrap(),
        );
        let required_presents = required_presents
            .trim()
            .split(' ')
            .map(|s| s.trim().parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        Area {
            area,
            required_presents,
        }
    }

    pub fn get_area(&self) -> usize {
        self.area.0 * self.area.1
    }
}
