use std::collections::VecDeque;
use std::fs::File;
use std::io::Read;
use std::path::Path;

fn main() {
    println!("Hello, world!");
    println!("part_one: {}", part_one("input1.txt"));
    println!("part_one: {}", part_one("input2.txt"));
}

fn part_one(filepath: &str) -> usize {
    let content = get_content(filepath.to_string());
    let content = content.split("\n\n");
    let mut sum = 0;
    for (index, set) in content.into_iter().enumerate() {
        let mut set = set.split('\n');
        let top = set.next().unwrap();
        let bottom = set.next().unwrap();
        let top = convert(top);
        let bottom = convert(bottom);
        let result = compare(top, bottom);
        sum += match result {
            None => 0,
            Some(false) => 0,
            Some(true) => index + 1,
        }
    }
    sum
}

fn convert(input: &str) -> Packet {
    let mut s = String::new();
    let mut stack: Vec<Packet> = Vec::new();
    let mut current_vec: Vec<Packet> = Vec::new();

    let input = input
        .trim()
        .strip_prefix('[')
        .unwrap()
        .strip_suffix(']')
        .unwrap();
    for c in input.chars() {
        match c {
            '[' => {
                stack.push(Packet::Vec(current_vec));
                current_vec = Vec::new();
                s = String::new()
            }
            ']' => {
                if !s.is_empty() {
                    current_vec.push(Packet::Integer(s.parse().unwrap()));
                }
                s = String::new();
                match stack.pop().unwrap() {
                    Packet::Vec(mut vec) => {
                        vec.push(Packet::Vec(current_vec));
                        current_vec = vec;
                    }
                    _ => panic!(),
                }
            }
            ',' => {
                if !s.is_empty() {
                    current_vec.push(Packet::Integer(s.parse().unwrap()));
                }
                s = String::new();
            }
            _ => {
                s.push(c);
            }
        }
    }
    if !s.is_empty() {
        //hack around the fact that we removed the outer []
        current_vec.push(Packet::Integer(s.parse().unwrap()));
    }
    Packet::Vec(current_vec)
}

fn compare(left: Packet, right: Packet) -> Option<bool> {
    match left {
        Packet::Vec(left) => match right {
            Packet::Vec(right) => compare_vec(left, right),
            Packet::Integer(right) => compare_vec(left, vec![Packet::Integer(right)]),
            Packet::Empty => Some(false),
        },
        Packet::Integer(left) => match right {
            Packet::Vec(right) => compare_vec(vec![Packet::Integer(left)], right),
            Packet::Integer(right) if right == left => None,
            Packet::Integer(right) => Some(left < right),
            Packet::Empty => Some(false),
        },
        Packet::Empty => match right {
            Packet::Empty => None,
            _ => Some(true),
        },
    }
}

fn compare_vec(left: Vec<Packet>, right: Vec<Packet>) -> Option<bool> {
    let mut left: VecDeque<Packet> = left.into_iter().collect();
    let mut right: VecDeque<Packet> = right.into_iter().collect();
    loop {
        if left.is_empty() {
            return if right.is_empty() { None } else { Some(true) };
        } else if right.is_empty() {
            return Some(false);
        }
        let left = left.pop_front().unwrap();
        let right = right.pop_front().unwrap();
        if let Some(result) = compare(left, right) {
            return Some(result);
        }
    }
}
#[derive(Debug)]
enum Packet {
    Vec(Vec<Packet>),
    Integer(i32),
    Empty,
}

fn get_content(filepath: String) -> String {
    let path = Path::new(&filepath);
    let mut file = File::open(path).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    content
}
