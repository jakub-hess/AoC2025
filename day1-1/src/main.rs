use std::{
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

fn main() {
    let now = Instant::now();
    let file = File::open("input-1-1.txt").unwrap();
    let reader = BufReader::new(file);
    let mut lock_state: u8 = 50;
    let mut pwd = 0u16;
    for line in reader.lines() {
        let line = line.unwrap().replace("L", "-").replace("R", "");
        let value: i16 = line.parse().unwrap();
        let current_state: i16 = lock_state as i16 + value;
        lock_state = current_state.rem_euclid(100) as u8;
        pwd += !((lock_state as i16).signum() as u16) & 0x0001;
    }
    println!("Password: {}", pwd);
    println!("Time: {:?}", now.elapsed());
}
