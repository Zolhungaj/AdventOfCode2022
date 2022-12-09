use std::collections::HashSet;
use std::fs::File;
use std::io::Read;
use std::path::Path;

fn main() {
    println!("part_one: {}", part_one("input1.txt"));
}

fn part_one(filepath: &str) -> usize {
    let instructions = extract_instructions(filepath);
    let mut head = (0, 0);
    let mut tail = head;
    let mut set: HashSet<(i32, i32)> = HashSet::new();
    set.insert(tail);
    for instruction in instructions {
        let idenity_vector = match instruction.direction {
            Direction::Up => (0, 1),
            Direction::Down => (0, -1),
            Direction::Right => (1, 0),
            Direction::Left => (-1, 0),
        };
        let direction_vector = (
            idenity_vector.0 * instruction.magnitude,
            idenity_vector.1 * instruction.magnitude,
        );
        head = (head.0 + direction_vector.0, head.1 + direction_vector.1);

        while not_touching(head, tail) {
            let distance = distance(head, tail);
            let identity_vector = get_identity_vector(distance);
            tail = (tail.0 + identity_vector.0, tail.1 + identity_vector.1);
            set.insert(tail);
        }
    }
    set.len()
}

fn extract_instructions(filepath: &str) -> Vec<Instruction> {
    let content = get_content(filepath.to_string());
    let mut instructions: Vec<Instruction> = Vec::new();
    for line in content.lines() {
        let mut split = line.split(' ');
        let direction = split.next().unwrap();
        let direction = match direction {
            "D" => Direction::Down,
            "U" => Direction::Up,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => panic!(),
        };
        let magnitude: i32 = split.next().unwrap().parse().unwrap();
        let instruction = Instruction {
            magnitude,
            direction,
        };
        instructions.push(instruction);
    }
    instructions
}

fn not_touching(head: (i32, i32), tail: (i32, i32)) -> bool {
    let distance = distance(head, tail);
    distance.0.abs() > 1 || distance.1.abs() > 1
}

fn distance(head: (i32, i32), tail: (i32, i32)) -> (i32, i32) {
    (head.0 - tail.0, head.1 - tail.1)
}

fn get_identity_vector(distance: (i32, i32)) -> (i32, i32) {
    let first = if distance.0 != 0 {
        distance.0 / distance.0.abs()
    } else {
        0
    };
    let second = if distance.1 != 0 {
        distance.1 / distance.1.abs()
    } else {
        0
    };
    (first, second)
}

#[derive(Debug, Copy, Clone)]
struct Instruction {
    magnitude: i32,
    direction: Direction,
}
#[derive(Debug, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn get_content(filepath: String) -> String {
    let path = Path::new(&filepath);
    let mut file = File::open(path).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    content
}
