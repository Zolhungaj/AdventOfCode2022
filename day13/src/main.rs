use std::collections::VecDeque;
use std::fs::File;
use std::io::Read;
use std::path::Path;

fn main() {
    println!("Hello, world!");
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
