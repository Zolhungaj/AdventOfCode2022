use std::fs::File;
use std::io::Read;
use std::path::Path;

fn main() {
    println!("Hello, world!");
    println!("Part one: {}", part_one("input1.txt"));
    println!("Part two: {}", part_two("input1.txt"));
}

fn part_one(filepath: &str) -> i32 {
    let lines = get_lines(filepath);
    let split: Vec<((i32, i32), (i32, i32))> = lines
        .iter()
        .map(|line| line.split(','))
        .map(|mut split| (split.next().unwrap(), split.next().unwrap()))
        .map(|(left, right): (&str, &str)| (left.split('-'), right.split('-')))
        .map(|(mut left, mut right)| {
            (
                (left.next().unwrap(), left.next().unwrap()),
                (right.next().unwrap(), right.next().unwrap()),
            )
        })
        .map(|((left_low, left_high), (right_low, right_high))| {
            (
                (left_low.parse().unwrap(), left_high.parse().unwrap()),
                (right_low.parse().unwrap(), right_high.parse().unwrap()),
            )
        })
        .collect();
    let mut count = 0;
    for ((left_low, left_high), (right_low, right_high)) in split {
        if (left_low <= right_low && left_high >= right_high)
            || (right_low <= left_low && right_high >= left_high)
        {
            count += 1;
        }
    }
    count
}

fn part_two(filepath: &str) -> i32 {
    0
}

fn get_lines(filepath: &str) -> Vec<String> {
    get_content(filepath.to_string())
        .lines()
        .map(|x| x.to_string())
        .collect()
}

fn get_content(filepath: String) -> String {
    let path = Path::new(&filepath);
    let mut file = File::open(path).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    content
}
