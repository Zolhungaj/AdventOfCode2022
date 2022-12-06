use std::collections::VecDeque;
use std::fs::File;
use std::io::Read;
use std::path::Path;

fn main() {
    println!("Hello, world!");
    println!("part_one: {}", part_one("input1.txt"));
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
            let mut last = '\0';
            for c in &buffer {
                if last == '\0' {
                    last = *c;
                }
                println!("{c}, {last}");
                if *c != last {
                    continue 'outer;
                }
            }
            return position;
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
