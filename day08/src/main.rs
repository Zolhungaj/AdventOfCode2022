use std::fs::File;
use std::io::Read;
use std::path::Path;

fn main() {
    println!("part_one: {}", part_one("input.txt"));
    println!("part_one: {}", part_two("input.txt"));
}

fn part_one(filepath: &str) -> i32 {
    let content = get_content(filepath.to_string());
    0
}

fn part_two(filepath: &str) -> i32 {
    0
}

fn get_content(filepath: String) -> String {
    let path = Path::new(&filepath);
    let mut file = File::open(path).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    content
}
