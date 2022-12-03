use std::fs::File;
use std::io::Read;
use std::path::Path;

fn main() {
    println!("Hello, world!");
    println!("Part one: {}", part_one("input1.txt"));
    //println!("Part two: {}", part_two("input1.txt"));
}

fn part_one(filepath: &str) -> i32 {
    let content = get_content(filepath.to_string());
    let lines = content.split('\n');
    let mut sum: i32 = 0;
    for line in lines {
        if line.is_empty() {
            continue;
        }
        let mid = line.len() / 2;
        let (left, right) = line.split_at(mid); //only works for ascii
        'outer: for c in left.chars() {
            let item = Item::new(c);
            for c in right.chars() {
                let other_item = Item::new(c);
                if item == other_item {
                    sum += item.priority() as i32;
                    break 'outer;
                }
            }
        }
    }
    sum
}

struct Item {
    value: char,
}

impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        //self.value.to_lowercase().to_string() == other.value.to_lowercase().to_string()
        self.value == other.value
    }
}

impl Item {
    fn priority(&self) -> u8 {
        match self.value as u8 {
            x if (b'A'..=b'Z').contains(&x) => x - b'A' + 27,
            x if (b'a'..=b'z').contains(&x) => x - b'a' + 1,
            _ => panic!(),
        }
    }

    fn new(c: char) -> Self {
        Self { value: c }
    }
}

fn get_content(filepath: String) -> String {
    let path = Path::new(&filepath);
    let mut file = File::open(path).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    content
}
