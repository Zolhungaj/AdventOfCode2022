use regex::Regex;
use std::fs::File;
use std::io::Read;
use std::path::Path;

fn main() {
    println!("part_one: {}", part_one("input2.txt", 10));
    println!("part_one: {}", part_one("input1.txt", 2000000));
}

fn part_one(filepath: &str, y: isize) -> usize {
    let content = get_content(filepath.to_string());
    let mut sensors = Vec::new();
    let regex = Regex::new(r"Sensor at x=(?P<sensor_x>-?\d+), y=(?P<sensor_y>-?\d+): closest beacon is at x=(?P<beacon_x>-?\d+), y=(?P<beacon_y>-?\d+)").unwrap();
    for line in content.lines() {
        let capture = regex.captures(line).unwrap();
        let sensor_point = Point::new(
            capture.name("sensor_x").unwrap().as_str().parse().unwrap(),
            capture.name("sensor_y").unwrap().as_str().parse().unwrap(),
        );
        let beacon_point = Point::new(
            capture.name("beacon_x").unwrap().as_str().parse().unwrap(),
            capture.name("beacon_y").unwrap().as_str().parse().unwrap(),
        );
        sensors.push(Sensor::new(sensor_point, beacon_point));
    }
    let mut min_x = isize::MAX;
    let mut max_x = isize::MIN;
    for sensor in &sensors {
        min_x = min_x.min(sensor.point.x - sensor.distance as isize);
        max_x = max_x.max(sensor.point.x + sensor.distance as isize);
    }
    let mut count = 0;
    'outer: for x in min_x..=max_x {
        let point = Point::new(x, y);
        let mut in_range = false;
        for sensor in &sensors {
            if sensor.beacon == point {
                continue 'outer; // is beacon and does therefore not count
            } else if point.distance(&sensor.point) <= sensor.distance {
                in_range = true;
            }
        }
        if in_range {
            count += 1
        }
    }
    count
}

struct Sensor {
    point: Point,
    beacon: Point,
    distance: usize,
}

impl Sensor {
    fn new(point: Point, beacon: Point) -> Self {
        let distance = point.distance(&beacon);
        Self {
            point,
            beacon,
            distance,
        }
    }
}

#[derive(PartialEq)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    fn distance(&self, other_point: &Self) -> usize {
        self.x.abs_diff(other_point.x) + self.y.abs_diff(other_point.y)
    }
}

fn get_content(filepath: String) -> String {
    let path = Path::new(&filepath);
    let mut file = File::open(path).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    content
}
