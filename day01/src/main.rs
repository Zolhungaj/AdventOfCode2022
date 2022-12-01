extern crate core;

use std::fs::File;
use std::io::Read;
use std::path::Path;

fn main() {
    let result1 = part1();
    println!("Result 1: {result1}");
    let result2 = part2();
    println!("Result 2: {result2}");
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

fn part2() -> u32 {
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

fn get_content(input: String) -> String {
    let path = Path::new(&input);
    let mut file = File::open(path).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    content
}
