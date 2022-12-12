use std::fs::File;
use std::io::Read;
use std::path::Path;

fn main() {
    println!("Hello, world!");
    println!("part_one: {}", part_one("input1.txt"));
    println!("part_one: {}", part_one("input2.txt"));
    println!("part_two: {}", part_two("input1.txt"));
    println!("part_two: {}", part_two("input2.txt"));
}

fn part_one(filepath: &str) -> i32 {
    0
}

fn part_two(_filepath: &str) -> i32 {
    0
}

fn get_content(filepath: String) -> String {
    let path = Path::new(&filepath);
    let mut file = File::open(path).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    content
}
