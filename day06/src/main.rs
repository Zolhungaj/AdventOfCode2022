use std::collections::VecDeque;
use std::fs::File;
use std::io::Read;
use std::path::Path;

fn main() {
    println!("Hello, world!");
    assert_eq!(5, part_one("input2.txt"));
    assert_eq!(6, part_one("input3.txt"));
    assert_eq!(10, part_one("input4.txt"));
    assert_eq!(11, part_one("input5.txt"));
    assert_eq!(7, part_one("input6.txt"));
    println!("part_one: {}", part_one("input1.txt"));

    assert_eq!(23, part_two("input2.txt"));
    assert_eq!(23, part_two("input3.txt"));
    assert_eq!(29, part_two("input4.txt"));
    assert_eq!(26, part_two("input5.txt"));
    assert_eq!(19, part_two("input6.txt"));
    println!("part_two: {}", part_two("input1.txt"));
}

fn part_one(filepath: &str) -> usize {
    let content = get_content(filepath.to_string());
    let mut buffer: VecDeque<char> = VecDeque::new();
    'outer: for (position, current) in content.split("").enumerate() {
        println!("{}, {}", position, current);
        if current.is_empty() {
            continue;
        }
        if buffer.len() >= 4 {
            buffer.pop_front();
        }
        buffer.push_back(current.chars().next().unwrap());
        println!("{:?}, {}", buffer, buffer.len());
        if buffer.len() == 4 {
            let mut second_buffer: Vec<char> = Vec::new();
            let mut success = true;
            while !buffer.is_empty() {
                let c = buffer.pop_front().unwrap();
                if second_buffer.contains(&c) {
                    success = false;
                }
                second_buffer.push(c);
            }
            if success {
                return position;
            }
            for c in second_buffer {
                buffer.push_back(c);
            }
        }
    }
    0
}

fn part_two(filepath: &str) -> usize {
    let content = get_content(filepath.to_string());
    let mut buffer: VecDeque<char> = VecDeque::new();
    'outer: for (position, current) in content.split("").enumerate() {
        println!("{}, {}", position, current);
        if current.is_empty() {
            continue;
        }
        if buffer.len() >= 14 {
            buffer.pop_front();
        }
        buffer.push_back(current.chars().next().unwrap());
        println!("{:?}, {}", buffer, buffer.len());
        if buffer.len() == 14 {
            let mut second_buffer: Vec<char> = Vec::new();
            let mut success = true;
            while !buffer.is_empty() {
                let c = buffer.pop_front().unwrap();
                if second_buffer.contains(&c) {
                    success = false;
                }
                second_buffer.push(c);
            }
            if success {
                return position;
            }
            for c in second_buffer {
                buffer.push_back(c);
            }
        }
    }
    0
}

fn get_content(filepath: String) -> String {
    let path = Path::new(&filepath);
    let mut file = File::open(path).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    content
}
