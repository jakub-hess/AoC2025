use std::{
    cmp::Reverse,
    collections::BinaryHeap,
    fmt::Debug,
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
    vec,
};

fn main() {
    println!("Day 8");
    println!("====================");
    let now = Instant::now();
    let file_read = Instant::now();
    let file = File::open("inputs-8.txt").unwrap();
    let reader = BufReader::new(file);

    let mut boxes = vec![];
    for line in reader.lines() {
        let line = line.unwrap();
        let b = Box::new(line);
        boxes.push(b);
    }
    let file_read_duration = file_read.elapsed();
    println!("File read time: {:?}", file_read_duration);

    let mut circuits = vec![];
    let op_start = Instant::now();
    let max_iterations = 1_000;
    let mut iteration_count = 0;

    let mut box_pair_heap = BinaryHeap::with_capacity(boxes.len() * boxes.len());
    let mut result = 0;
    let result_p2;
    //println!("Last line: {:?}", lines.last().unwrap());
    for i in 0..boxes.len() {
        for j in 1 + i..boxes.len() {
            let pair = BoxPair::new(boxes[i], boxes[j]);
            box_pair_heap.push(Reverse(pair));
            //println!("Box pair: left: {:?}, right: {:?}, distance: {}", pair.left, pair.right, pair.distance);
        }
    }

    loop {
        let box_pair = box_pair_heap.pop().unwrap().0;
        if circuits.len() == 0 {
            let box_circuit = BoxCircuit::new(box_pair);
            circuits.push(box_circuit);
            iteration_count += 1;
            continue;
        }
        let mut found_circuit = false;
        let mut found_circuits: Vec<BoxCircuit> = vec![];
        for circuit in circuits.iter_mut() {
            if circuit.contains_pair(&box_pair) {
                found_circuit = true;
                iteration_count += 1;
                break;
            }
            if circuit.contains_box(&box_pair) {
                circuit.insert_pair(box_pair);
                found_circuit = true;
                iteration_count += 1;
                found_circuits.push(circuit.clone());
            }
        }

        if !found_circuit {
            let box_circuit = BoxCircuit::new(box_pair);
            circuits.push(box_circuit);
            iteration_count += 1;
        }
        if found_circuits.len() == 2 {
            let mut base_circuit = found_circuits.first().unwrap().clone();
            for c in found_circuits.iter().skip(1) {
                base_circuit = base_circuit.join(c);
            }
            circuits.retain(|c| !found_circuits.contains(&c));
            circuits.push(base_circuit.clone());
            iteration_count -= 1;
        }
        if iteration_count == max_iterations {
            circuits.sort_by_key(|circuit| Reverse(circuit.size()));
            let longest_curcuit = circuits
                .iter()
                .take(3)
                .map(|c| c.size())
                .collect::<Vec<usize>>();
            println!("Longest circuits sizes: {:?}", longest_curcuit);
            result = longest_curcuit.iter().product::<usize>();
            println!("P1 Operation time: {:?}", op_start.elapsed());
        }
        circuits.sort_by_key(|circuit| Reverse(circuit.size()));
        if box_pair_heap.is_empty() || circuits.first().unwrap().size() == boxes.len() {
            result_p2 = box_pair.left.x * box_pair.right.x;
            break;
        }
    }

    let op_duration = op_start.elapsed();
    println!("P2 Operation time: {:?}", op_duration);
    println!("P2 needed {} iterations", iteration_count);
    println!("Result P1: {:?}", result);
    println!("Result P2: {:?}", result_p2);

    println!("Time: {:?}", now.elapsed());
}

#[derive(Clone, Copy)]
pub enum Position {
    Empty,
    Start,
    Splitter(u128),
    Beam(u128),
}

#[derive(Debug, Clone, Copy)]
pub struct Box {
    pub x: usize,
    pub y: usize,
    pub z: usize,
}

impl Box {
    pub fn new(position: String) -> Self {
        let coords = position.split(',').collect::<Vec<&str>>();
        let x = coords[0].parse::<usize>().unwrap();
        let y = coords[1].parse::<usize>().unwrap();
        let z = coords[2].parse::<usize>().unwrap();
        Box { x, y, z }
    }
}

impl PartialEq for Box {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl Eq for Box {}

#[derive(Debug, Clone, Copy)]
pub struct BoxPair {
    pub left: Box,
    pub right: Box,
    distance: i64,
}

impl BoxPair {
    pub fn new(left: Box, right: Box) -> Self {
        let distance = ((left.x - right.x) * (left.x - right.x)
            + (left.y - right.y) * (left.y - right.y)
            + (left.z - right.z) * (left.z - right.z)) as i64;
        BoxPair {
            left,
            right,
            distance: distance,
        }
    }
}

impl Ord for BoxPair {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.distance.cmp(&other.distance)
    }
}

impl PartialOrd for BoxPair {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for BoxPair {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}

impl Eq for BoxPair {}

#[derive(Clone)]
pub struct BoxCircuit {
    pub id: uuid::Uuid,
    pub boxes: Vec<Box>,
}

impl BoxCircuit {
    pub fn new(pair: BoxPair) -> Self {
        BoxCircuit {
            id: uuid::Uuid::new_v4(),
            boxes: vec![pair.left, pair.right],
        }
    }

    pub fn insert_pair(&mut self, box_pair: BoxPair) {
        if self.boxes.contains(&box_pair.left) == false {
            self.boxes.push(box_pair.left);
        }
        if self.boxes.contains(&box_pair.right) == false {
            self.boxes.push(box_pair.right);
        }
    }

    pub fn contains_box(&self, b: &BoxPair) -> bool {
        self.boxes
            .iter()
            .find(|x| **x == b.left || **x == b.right)
            .is_some()
    }

    pub fn contains_pair(&self, b: &BoxPair) -> bool {
        self.boxes.iter().find(|x| **x == b.left).is_some()
            && self.boxes.iter().find(|x| **x == b.right).is_some()
    }

    pub fn size(&self) -> usize {
        self.boxes.len()
    }

    pub fn join(&self, other: &BoxCircuit) -> BoxCircuit {
        let mut new_circuit = self.clone();
        for b in other.boxes.iter() {
            if new_circuit.boxes.contains(b) == false {
                new_circuit.boxes.push(*b);
            }
        }
        new_circuit
    }
}

impl PartialEq for BoxCircuit {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for BoxCircuit {}
