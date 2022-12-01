extern crate core;

use std::fs::File;
use std::io::Read;
use std::path::Path;

fn main() {
    let result1 = part1();
    println!("Result 1: {result1}");
}

fn part1() -> u32 {
    let content = get_content("input1.txt".to_string());
    let values = content.split('\n');
    let mut calories: Vec<u32> = Vec::new();
    let mut sum = 0;
    for line in values {
        if line.is_empty() {
            calories.push(sum);
            sum = 0;
            continue;
        }
        sum += line.parse::<u32>().unwrap();
    }
    calories.into_iter().max().unwrap()
}

fn get_content(input: String) -> String {
    let path = Path::new(&input);
    let mut file = File::open(path).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    content
}
