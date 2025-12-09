use std::{
    collections::BinaryHeap,
    fmt::Debug,
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
    vec,
};

fn main() {
    println!("Day 9");
    println!("====================");
    let now = Instant::now();
    let file_read = Instant::now();
    let file = File::open("inputs-9.txt").unwrap();
    let reader = BufReader::new(file);

    let mut points = vec![];
    for line in reader.lines() {
        let line = line.unwrap();
        let b = Point::new(line);
        points.push(b);
    }
    let file_read_duration = file_read.elapsed();
    println!("File read time: {:?}", file_read_duration);

    let op_start = Instant::now();

    let mut box_pair_heap = BinaryHeap::with_capacity(points.len() * points.len());

    for i in 0..points.len() {
        for j in 1 + i..points.len() {
            let pair = Rectangle::new(points[i], points[j]);
            box_pair_heap.push(pair);
        }
    }

    let mut result = box_pair_heap.pop().unwrap();
    let result_p1 = result.area;
    println!("P1 Time: {:?}", op_start.elapsed());
    let p2_start = Instant::now();
    let polygon = Polygon::new(points.clone());
    loop {
        let mut points_inside = 0;
        // Check all four corners are inside the polygon
        // This is a cheap computation compared to edge intersection checks
        for point in result.get_all_four_corners().iter() {
            if !polygon.point_inside(*point) {
                if let Some(next_pair) = box_pair_heap.pop() {
                    result = next_pair;
                    break;
                } else {
                    break;
                }
            }
            points_inside += 1;
        }
        if points_inside == 4 {
            // Now check if any of the polygon edges intersect with the rectangle edges
            if polygon.edge_intersects_rectangle(&result) {
                if let Some(next_pair) = box_pair_heap.pop() {
                    result = next_pair;
                    continue;
                } else {
                    break;
                }
            }
            break;
        }
    }

    let op_duration = p2_start.elapsed();
    println!("P2 time: {:?}", op_duration);
    println!("Result P1: {:?}", result_p1);
    println!("Result P2: {:?}", result.area);

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
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    pub fn new(position: String) -> Self {
        let coords = position.split(',').collect::<Vec<&str>>();
        let x = coords[0].parse::<usize>().unwrap();
        let y = coords[1].parse::<usize>().unwrap();

        Point { x, y }
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for Point {}

#[derive(Debug, Clone, Copy)]
pub struct Rectangle {
    pub a: Point,
    pub b: Point,
    pub area: i64,
}

impl Rectangle {
    pub fn new(a: Point, b: Point) -> Self {
        let width = (a.x as i64 - b.x as i64).abs();
        let height = (a.y as i64 - b.y as i64).abs();
        let area = (width + 1) * (height + 1);
        Rectangle { a, b, area: area }
    }

    pub fn point_inside(&self, p: Point) -> bool {
        let min_x = self.a.x.min(self.b.x);
        let max_x = self.a.x.max(self.b.x);
        let min_y = self.a.y.min(self.b.y);
        let max_y = self.a.y.max(self.b.y);

        println!(
            "Checking point {:?} inside rectangle a: {:?}, b: {:?}, min_x: {}, max_x: {}, min_y: {}, max_y: {}",
            p, self.a, self.b, min_x, max_x, min_y, max_y
        );
        p.x > min_x && p.x < max_x && p.y > min_y && p.y < max_y
    }

    pub fn get_all_four_corners(&self) -> Vec<Point> {
        let min_x = self.a.x.min(self.b.x);
        let max_x = self.a.x.max(self.b.x);
        let min_y = self.a.y.min(self.b.y);
        let max_y = self.a.y.max(self.b.y);

        vec![
            Point { x: min_x, y: min_y },
            Point { x: min_x, y: max_y },
            Point { x: max_x, y: min_y },
            Point { x: max_x, y: max_y },
        ]
    }
}

impl Ord for Rectangle {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.area.cmp(&other.area)
    }
}

impl PartialOrd for Rectangle {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Rectangle {
    fn eq(&self, other: &Self) -> bool {
        self.area == other.area
    }
}

impl Eq for Rectangle {}

#[derive(Debug, Clone)]
pub struct Polygon {
    pub corners: Vec<Point>,
}

impl Polygon {
    pub fn new(corners: Vec<Point>) -> Self {
        Polygon { corners }
    }

    pub fn point_inside(&self, p: Point) -> bool {
        // Implement point-in-polygon algorithm (e.g., ray-casting)
        // Also returns true if point is on the boundary
        let mut inside = false;
        let n = self.corners.len();
        let mut j = n - 1;
        for i in 0..n {
            let xi = self.corners[i].x as i64;
            let yi = self.corners[i].y as i64;
            let xj = self.corners[j].x as i64;
            let yj = self.corners[j].y as i64;
            let px = p.x as i64;
            let py = p.y as i64;

            // Check if point is on this edge
            if Self::point_on_segment(px, py, xi, yi, xj, yj) {
                return true;
            }

            if ((yi > py) != (yj > py)) && (px < (xj - xi) * (py - yi) / (yj - yi) + xi) {
                inside = !inside;
            }
            j = i;
        }
        inside
    }

    /// Check if point (px, py) lies on the line segment from (x1, y1) to (x2, y2)
    fn point_on_segment(px: i64, py: i64, x1: i64, y1: i64, x2: i64, y2: i64) -> bool {
        // Check if point is within the bounding box of the segment
        let min_x = x1.min(x2);
        let max_x = x1.max(x2);
        let min_y = y1.min(y2);
        let max_y = y1.max(y2);

        if px < min_x || px > max_x || py < min_y || py > max_y {
            return false;
        }

        // Check if point is collinear with the segment using cross product
        // (px - x1) * (y2 - y1) == (py - y1) * (x2 - x1)
        let cross = (px - x1) * (y2 - y1) - (py - y1) * (x2 - x1);
        cross == 0
    }

    fn edge_intersects_rectangle(&self, rect: &Rectangle) -> bool {
        // Check if any of the rectangle's corners are inside the polygon
        let n = self.corners.len();
        let mut j = n - 1;
        for i in 0..n {
            let xi = self.corners[i].x as i64;
            let yi = self.corners[i].y as i64;
            let xj = self.corners[j].x as i64;
            let yj = self.corners[j].y as i64;
            let corners = rect.get_all_four_corners();
            let mut k = corners.len() - 1;
            for l in 0..corners.len() {
                let xk = corners[l].x as f64;
                let yk = corners[l].y as f64;
                let xl = corners[k].x as f64;
                let yl = corners[k].y as f64;
                if line_line_intersection(
                    xi as f64, yi as f64, xj as f64, yj as f64, xk, yk, xl, yl,
                ) {
                    return true;
                }
                k = l;
            }
            j = i;
        }
        return false;
    }
}

fn line_line_intersection(
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
    x3: f64,
    y3: f64,
    x4: f64,
    y4: f64,
) -> bool {
    let u_a = ((x4 - x3) * (y1 - y3) - (y4 - y3) * (x1 - x3))
        / ((y4 - y3) * (x2 - x1) - (x4 - x3) * (y2 - y1));
    let u_b = ((x2 - x1) * (y1 - y3) - (y2 - y1) * (x1 - x3))
        / ((y4 - y3) * (x2 - x1) - (x4 - x3) * (y2 - y1));

    if u_a > 0.0 && u_a < 1.0 && u_b > 0.0 && u_b < 1.0 {
        return true;
    }
    false
}
