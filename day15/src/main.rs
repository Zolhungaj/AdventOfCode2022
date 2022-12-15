use regex::Regex;
use std::fs::File;
use std::io::Read;
use std::path::Path;

fn main() {
    println!("part_one: {}", part_one("input2.txt", 10));
    println!("part_one: {}", part_one("input1.txt", 2000000));
    println!("part_two: {}", part_two("input2.txt", 20));
    println!("part_two: {}", part_two("input1.txt", 4000000));
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

fn part_two(filepath: &str, search_space_max: isize) -> usize {
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
    let search_space_min = 0;
    for sensor in &sensors {
        //limit search space to circumference of covered area
        let north = (
            sensor.point.x,
            sensor.point.y - sensor.distance as isize - 1,
        );
        let south = (
            sensor.point.x,
            sensor.point.y + sensor.distance as isize + 1,
        );
        let west = (
            sensor.point.x - sensor.distance as isize - 1,
            sensor.point.y,
        );
        let east = (
            sensor.point.x + sensor.distance as isize + 1,
            sensor.point.y,
        );
        for n in 0..sensor.distance {
            let n = n as isize;
            let north = (north.0 + n, north.1 + n);
            let south = (south.0 - n, south.1 - n);
            let east = (east.0 - n, east.1 + n);
            let west = (west.0 + n, west.1 - n);
            'clock: for (x, y) in [north, south, east, west] {
                if x < search_space_min
                    || x > search_space_max
                    || y < search_space_min
                    || y > search_space_max
                {
                    continue;
                }
                let point = Point::new(x, y);
                for sensor in &sensors {
                    if point.distance(&sensor.point) <= sensor.distance {
                        continue 'clock;
                    }
                }
                return x as usize * 4000000 + y as usize;
            }
        }
    }
    0
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
