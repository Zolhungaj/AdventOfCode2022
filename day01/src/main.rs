extern crate core;

use std::fs::File;
use std::io::Read;
use std::path::Path;

fn main() {
    let result1 = part1("input1.txt");
    println!("Result 1: {result1}");
    let result2 = part2("input1.txt");
    println!("Result 2: {result2}");
}

fn part1(filepath: &str) -> u32 {
    let calories = get_calories(filepath);
    calories.into_iter().max().unwrap()
}

fn part2(filepath: &str) -> u32 {
    let calories = get_calories(filepath);
    let mut top3 = vec![0, 0, 0];
    for mut sum in calories {
        for value in top3.iter_mut() {
            if *value < sum {
                std::mem::swap(&mut (*value), &mut sum);
            }
        }
    }

    top3.into_iter().sum()
}

fn get_calories(filepath: &str) -> Vec<u32> {
    let content = get_content(filepath.to_string());
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
    calories
}

fn get_content(input: String) -> String {
    let path = Path::new(&input);
    let mut file = File::open(path).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    content
}
