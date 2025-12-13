use std::{
    collections::{HashMap, VecDeque},
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
    vec,
};

fn main() {
    println!("Day 10");
    println!("====================");
    let now = Instant::now();
    let file_read = Instant::now();
    let file = File::open("inputs-11.txt").unwrap();
    let reader = BufReader::new(file);

    let mut machines = HashMap::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let line_split = line.split(':').collect::<Vec<&str>>();
        let server_id = line_split[0].to_string();
        let connections = line_split[1]
            .trim()
            .split(' ')
            .map(|s| s.trim().to_string())
            .collect::<Vec<String>>();
        machines.insert(server_id, connections);
    }
    let file_read_duration = file_read.elapsed();
    println!("File read time: {:?}", file_read_duration);

    //println!("Machines: {:?}", machines);

    if machines.contains_key("you") {
        let op_start = Instant::now();
        let mut to_visit = VecDeque::new();
        to_visit.push_back("you".to_string());
        let mut result_p1 = 0;

        while let Some(current) = to_visit.pop_back() {
            // Process each machine
            let connections = machines.get(&current);
            //println!("Visiting machine: {}", current);
            if let Some(connections) = connections {
                for conn in connections {
                    if conn == "out" {
                        //println!("Found exit at machine: {}", current);
                        result_p1 += 1;
                        continue;
                    }
                    to_visit.push_back(conn.clone());
                }
            }
        }

        let op_duration = op_start.elapsed();
        println!("P1 time: {:?}", op_duration);
        println!("Result P1: {:?}", result_p1);
    }

    //println!("{:?}", machines.keys());
    let connections_to_dac = machines
        .iter()
        .filter(|(_k, v)| v.contains(&"fft".to_string()))
        .count();
    println!("Connections to fft: {}", connections_to_dac);
    let connections_to_dac = machines
        .iter()
        .filter(|(_k, v)| v.contains(&"dac".to_string()))
        .count();
    println!("Connections to dac: {}", connections_to_dac);

    if machines.contains_key("svr") {
        let p2_start = Instant::now();
        let mut to_visit: VecDeque<Vec<String>> = VecDeque::new();
        to_visit.push_back(vec!["fft".to_string()]);
        let svr_fft_paths = dfs(
            &machines,
            &"svr".to_string(),
            &"fft".to_string(),
            &mut HashMap::new(),
        );
        let fft_dac_paths = dfs(
            &machines,
            &"fft".to_string(),
            &"dac".to_string(),
            &mut HashMap::new(),
        );

        let dac_out_paths = dfs(
            &machines,
            &"dac".to_string(),
            &"out".to_string(),
            &mut HashMap::new(),
        );
        println!("Paths from svr to fft: {}", svr_fft_paths);
        println!("Paths from fft to dac: {}", fft_dac_paths);
        println!("Paths from dac to out: {}", dac_out_paths);

        let p2_duration = p2_start.elapsed();

        println!("P2 time: {:?}", p2_duration);

        println!(
            "Result P2: {:?}",
            svr_fft_paths * fft_dac_paths * dac_out_paths
        );
    }

    println!("Time: {:?}", now.elapsed());
}

pub fn dfs(
    graph: &HashMap<String, Vec<String>>,
    current: &String,
    target: &String,
    visited: &mut HashMap<String, i64>,
) -> i64 {
    if current == target {
        return 1;
    }
    if let Some(count) = visited.get(current) {
        return *count;
    }

    let mut result = 0;
    for connections in graph.get(current).unwrap_or(&vec![]) {
        result += dfs(graph, connections, target, visited);
    }
    visited.insert(current.clone(), result);
    result
}
