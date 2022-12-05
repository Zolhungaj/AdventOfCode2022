use regex::Regex;
use std::fs::File;
use std::io::Read;
use std::path::Path;

fn main() {
    println!("Hello, world!");
    println!("part_one '{}'", part_one("input1.txt"));
    println!("part_one '{}'", part_one("input2.txt"));
    println!("part_two '{}'", part_two("input1.txt"));
    println!("part_two '{}'", part_two("input2.txt"));
}

fn part_one(filepath: &str) -> String {
    let content = get_content(filepath.to_string());
    let mut content = content.split("\n\n");
    let initial_state_drawing = content.next().unwrap();
    let mut initial_state_list = initial_state_drawing.lines().rev();

    //create a generic mapping for the stack numbers (even though the input is just a 1-indexed array)
    let stack_identifiers: Vec<i32> = initial_state_list
        .next()
        .unwrap()
        .strip_prefix(' ') //remove leading space
        .unwrap()
        .split_whitespace()
        .into_iter()
        .map(|x| x.parse().unwrap())
        .collect();
    println!("{:?}", stack_identifiers);
    let count = stack_identifiers.len();
    let mut stacks: Vec<Vec<String>> = Vec::with_capacity(count);
    for _ in 0..count {
        stacks.push(Vec::new());
    }

    for line in initial_state_list {
        let mut current = line;
        for i in 0..count {
            let (left, new_current) = current.split_at(3);
            current = new_current;
            if left != "   " {
                let left = left.replace('[', "").replace(']', "");
                stacks[i].push(left);
            }
            if current.is_empty() {
                break;
            }
            let (_, new_current) = current.split_at(1);
            current = new_current
        }
    }
    println!("{:?}", stacks);
    let instructions = content.next().unwrap();
    let regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    for capture in regex.captures_iter(instructions) {
        let count: i32 = capture[1].parse().unwrap();
        let from: i32 = capture[2].parse().unwrap();
        let to: i32 = capture[3].parse().unwrap();

        //translate to stack position based on identifier
        let from = stack_identifiers.iter().position(|x| *x == from).unwrap();
        let to = stack_identifiers.iter().position(|x| *x == to).unwrap();

        for _ in 0..count {
            let value = stacks[from].pop().unwrap();
            stacks[to].push(value);
        }
    }
    println!("{:?}", stacks);
    stacks
        .into_iter()
        .map(|mut stack| stack.pop().unwrap_or_else(|| " ".to_string()))
        .collect()
}

fn part_two(filepath: &str) -> String {
    let content = get_content(filepath.to_string());
    let mut content = content.split("\n\n");
    let initial_state_drawing = content.next().unwrap();
    let mut initial_state_list = initial_state_drawing.lines().rev();

    //create a generic mapping for the stack numbers (even though the input is just a 1-indexed array)
    let stack_identifiers: Vec<i32> = initial_state_list
        .next()
        .unwrap()
        .strip_prefix(' ') //remove leading space
        .unwrap()
        .split_whitespace()
        .into_iter()
        .map(|x| x.parse().unwrap())
        .collect();
    println!("{:?}", stack_identifiers);
    let count = stack_identifiers.len();
    let mut stacks: Vec<Vec<String>> = Vec::with_capacity(count);
    for _ in 0..count {
        stacks.push(Vec::new());
    }

    for line in initial_state_list {
        let mut current = line;
        for i in 0..count {
            let (left, new_current) = current.split_at(3);
            current = new_current;
            if left != "   " {
                let left = left.replace('[', "").replace(']', "");
                stacks[i].push(left);
            }
            if current.is_empty() {
                break;
            }
            let (_, new_current) = current.split_at(1);
            current = new_current
        }
    }
    println!("{:?}", stacks);
    let instructions = content.next().unwrap();
    let regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    for capture in regex.captures_iter(instructions) {
        let count: usize = capture[1].parse().unwrap();
        let from: i32 = capture[2].parse().unwrap();
        let to: i32 = capture[3].parse().unwrap();

        //translate to stack position based on identifier
        let from = stack_identifiers.iter().position(|x| *x == from).unwrap();
        let to = stack_identifiers.iter().position(|x| *x == to).unwrap();

        let from_length = stacks[from].len();
        let values: Vec<String> = stacks[from]
            .drain((from_length - count)..from_length)
            .collect();
        values.into_iter().for_each(|x| stacks[to].push(x));
    }
    println!("{:?}", stacks);
    stacks
        .into_iter()
        .map(|mut stack| stack.pop().unwrap_or_else(|| " ".to_string()))
        .collect()
}

fn get_content(filepath: String) -> String {
    let path = Path::new(&filepath);
    let mut file = File::open(path).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    content
}
