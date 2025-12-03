use std::{
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

fn main() {
    let now = Instant::now();
    let file = File::open("input-1-2.txt").unwrap();
    let reader = BufReader::new(file);
    let mut lock_state: u8 = 50;
    let mut pwd = 0u16;
    for line in reader.lines() {
        let line = line.unwrap().replace("L", "-").replace("R", "");
        let value: i16 = line.parse().unwrap();
        let current_state: i16 = lock_state as i16 + value;
        // we cross zero
        pwd += ((lock_state as i16).signum() - current_state.signum() as i16).abs() as u16 / 2;
        // we wrap around zero multiple times
        pwd += (current_state / 100).abs() as u16;
        // we land exactly on zero without wrapping, because div on line above already counts wrapping land on 0
        pwd += !(current_state.signum() as u16) & 0x0001;
        lock_state = current_state.rem_euclid(100) as u8;
    }
    println!("Password: {}", pwd);
    println!("Time: {:?}", now.elapsed());
}
