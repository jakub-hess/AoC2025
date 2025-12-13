use std::{
    collections::VecDeque,
    fmt::Debug,
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
    vec,
};

use uuid::Uuid;

fn main() {
    println!("Day 10");
    println!("====================");
    let now = Instant::now();
    let file_read = Instant::now();
    let file = File::open("inputs-10.txt").unwrap();
    let reader = BufReader::new(file);
    let mut machines = VecDeque::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let b = Maschine::new(line);
        machines.push_back(b);
    }
    let file_read_duration = file_read.elapsed();
    println!("File read time: {:?}", file_read_duration);

    //println!("Maschines loaded: {:?}", machines);
    let op_start = Instant::now();
    for machine in machines.iter().skip(195) {
        println!("{:?}", machine);

        println!("Joltage matrix:");
        //print_matrix(&light_matrix);
        let mut joltage_matrix = machine.construct_joltage_matrix();
        solve(&mut joltage_matrix);
        write_equations(&joltage_matrix);
    }
    let op_duration = op_start.elapsed();
    println!("P1 time: {:?}", op_duration);
    println!("Result P1: {:?}", 0);
    let p2_start = Instant::now();

    let p2_duration = p2_start.elapsed();
    //println!("Machine count final: {:?}", machines_on.len());

    println!("P2 time: {:?}", p2_duration);

    println!(
        "Result P2: {:?}",
        machines.iter().map(|x| x.joltage_steps).sum::<i32>()
    );

    println!("Time: {:?}", now.elapsed());
}

#[derive(Debug, Clone)]
pub struct Maschine {
    pub id: Uuid,
    // Machine fields here
    required_lights: Vec<bool>,
    light_state: Vec<bool>,
    buttons: Vec<Vec<i8>>,
    required_joltage: Vec<i32>,
    current_joltage: Vec<i32>,
    pub buttons_pressed: i32,
    pub joltage_steps: i32,
}

impl Maschine {
    pub fn new(line: String) -> Self {
        let start = line.find('[').unwrap();
        let end = line.find(']').unwrap();
        let lights_str = &line[start + 1..end];
        let required_lights = lights_str.chars().map(|c| c == '#').collect::<Vec<bool>>();
        let light_state = vec![false; required_lights.len()];
        let mut buttons_str = &line[end + 1..];
        let mut buttons = vec![];
        while let Some(open_idx) = buttons_str.find('(') {
            let close_idx = buttons_str[open_idx..].find(')').unwrap() + open_idx;
            let button_str = &buttons_str[open_idx + 1..close_idx];
            let button = button_str
                .split(',')
                .map(|s| s.trim().parse::<i8>().unwrap())
                .collect::<Vec<i8>>();
            buttons.push(button);
            if close_idx + 1 >= buttons_str.len() {
                break;
            }
            buttons_str = &buttons_str[close_idx + 1..];
        }
        let start = line.find('{').unwrap();
        let end = line.find('}').unwrap();
        let required_joltage = line[start + 1..end]
            .split(',')
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        let current_joltage = vec![0; required_joltage.len()];
        Self {
            required_lights,
            light_state,
            buttons,
            required_joltage,
            current_joltage,
            buttons_pressed: 0,
            joltage_steps: 0,
            id: Uuid::new_v4(),
        }
    }

    pub fn push_button_lights(&mut self, button_index: usize) {
        if button_index >= self.buttons.len() {
            return;
        }
        let button = &self.buttons[button_index];
        for i in button.iter() {
            self.light_state[*i as usize] = !self.light_state[*i as usize];
        }
        self.buttons_pressed += 1;
    }

    pub fn push_button_joltage(&mut self, button_index: usize) {
        if button_index >= self.buttons.len() {
            return;
        }
        let button = &self.buttons[button_index];
        for i in button.iter() {
            self.current_joltage[*i as usize] = self.current_joltage[*i as usize] + 1;
        }
        self.joltage_steps += 1;
    }

    pub fn is_on(&self) -> bool {
        self.light_state == self.required_lights
    }

    pub fn machine_state_string(&self) -> String {
        let lights_str = self
            .light_state
            .iter()
            .map(|&b| if b { '#' } else { '.' })
            .collect::<String>();
        return lights_str;
    }

    pub fn joltage_machine_state_string(&self) -> String {
        let joltage_str = self
            .current_joltage
            .iter()
            .map(|&b| b.to_string())
            .collect::<Vec<String>>()
            .join(",");
        return joltage_str;
    }

    pub fn correct_joltage(&self) -> bool {
        self.current_joltage == self.required_joltage
    }

    fn construct_button_matrix(&self) -> Vec<Vec<f32>> {
        let mut matrix = vec![vec![0.0; self.buttons.len()]; self.required_lights.len()];
        for (j, button) in self.buttons.iter().enumerate() {
            for &light_index in button.iter() {
                matrix[light_index as usize][j] = 1.0;
            }
        }
        matrix
    }

    pub fn construct_light_matrix(&self) -> Vec<Vec<f32>> {
        let mut matrix = self.construct_button_matrix();
        for i in 0..matrix.len() {
            matrix[i].push(if self.required_lights[i] { 1.0 } else { 0.0 });
        }
        matrix
    }

    pub fn construct_joltage_matrix(&self) -> Vec<Vec<f32>> {
        let mut matrix = self.construct_button_matrix();
        for i in 0..matrix.len() {
            matrix[i].push(self.required_joltage[i] as f32);
        }
        matrix
    }

    // Additional methods for Maschine
}

pub fn print_matrix(matrix: &Vec<Vec<f32>>) {
    for row in matrix.iter() {
        let row_str = row
            .iter()
            .map(|&b| b.to_string())
            .collect::<Vec<String>>()
            .join("\t");
        println!("{}", row_str);
    }
}

pub fn solve(matrix: &mut Vec<Vec<f32>>) {
    println!("Initial matrix:");
    print_matrix(matrix);
    get_echeleon_form(matrix);
    back_substitution(matrix);
    flip_signs(matrix);
    normalize_matrix(matrix);
    println!("Final matrix:");
    print_matrix(matrix);
}

fn normalize_matrix(matrix: &mut Vec<Vec<f32>>) {
    for row in 0..matrix.len() {
        let leading_coeff_index = matrix[row]
            .iter()
            .position(|&x| x != 0.0)
            .unwrap_or(matrix[row].len() - 1);
        let leading_coeff = matrix[row][leading_coeff_index];
        if leading_coeff != 0.0 {
            for col in 0..matrix[row].len() {
                matrix[row][col] /= leading_coeff;
            }
        }
    }
}

fn flip_signs(matrix: &mut Vec<Vec<f32>>) {
    for row in 0..matrix[0].len().min(matrix.len()) {
        if matrix[row][row] < 0.0 {
            //println!("Flipping signs in row {}", row);
            for col in 0..matrix[row].len() {
                matrix[row][col] = -matrix[row][col];
            }
            continue;
        }
    }
}

fn back_substitution(matrix: &mut Vec<Vec<f32>>) {
    //Implementation of back substitution to solve for variables

    for row in (1..matrix.len()).rev() {
        let mut col = None;
        for column in 0..matrix[0].len() - 1 {
            if matrix[row][column] != 0.0 {
                // println!("Found leading 1 at row {}, column {}", row, column);
                col = Some(column);
                break;
            }
        }
        if col.is_none() {
            continue;
        }
        let col = col.unwrap();
        // println!("Back substituting for row {}, column {}", row, col);
        for next_row in 0..row {
            let constant = matrix[next_row][col] / matrix[row][col];
            for col in 0..matrix[0].len() {
                matrix[next_row][col] -= matrix[row][col] * constant;
            }
        }
        // print_matrix(matrix);
    }
}

fn get_echeleon_form(matrix: &mut Vec<Vec<f32>>) {
    /*Implementation of converting matrix to echelon form*/
    let mut row = 0;
    for column in 0..matrix[0].len() - 1 {
        let pivot_row = find_pivot_column(matrix, row, column);
        // print!("Pivot row for column {}: {:?}\n", column, pivot_row);
        let _ = if let Some(pivot_row) = pivot_row {
            matrix.swap(row, pivot_row);

            pivot_row
        } else {
            // print_matrix(matrix);
            continue;
        };
        for next_row in row + 1..matrix.len() {
            // println!("Eliminating row {} using pivot row {}", next_row, row);
            if matrix[next_row][column] != 0.0 {
                let constant = matrix[next_row][column] / matrix[row][column];
                for col in 0..matrix[0].len() {
                    matrix[next_row][col] -= matrix[row][col] * constant;
                }
            }
        }
        row += 1;
        if row >= matrix.len() {
            break;
        }
        // println!("Matrix after processing column {}:", column);
        // print_matrix(matrix);
    }
}

fn find_pivot_column(matrix: &Vec<Vec<f32>>, row: usize, col: usize) -> Option<usize> {
    for next_row in row..matrix.len() {
        // println!(
        //     "Checking row {}, col {}: {}",
        //     next_row, col, matrix[next_row][col]
        // );
        if matrix[next_row][col] != 0.0 {
            return Some(next_row);
        }
    }
    None
}

fn write_equations(matrix: &Vec<Vec<f32>>) {
    let mut equation = String::new();
    for row in 0..matrix.len() {
        let pivot_col = if let Some(pos) = matrix[row].iter().position(|&x| x != 0.0) {
            pos
        } else {
            continue;
        };

        equation.push_str(format!("x{} = ", pivot_col + 1).as_str());

        equation.push_str(format!("{} ", matrix[row].last().unwrap()).as_str());

        for col in pivot_col + 1..matrix[row].len() - 1 {
            let coeff = -matrix[row][col];
            if coeff != 0.0 {
                if coeff > 0.0 {
                    equation.push_str("+ ");
                } else {
                    equation.push_str("- ");
                }
                equation.push_str(format!("{}x{} ", coeff.abs(), col + 1).as_str());
            }
        }

        equation.push_str("\n");
    }

    println!("{}", equation)
}
