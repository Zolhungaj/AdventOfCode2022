use std::io::Read;

fn main() {
    println!("part_one: {}", part_one("input2.txt"));
    println!("part_one: {}", part_one("input1.txt"));
}

fn part_one(filepath: &str) {
    let content = get_content(filepath.to_string());
}

fn get_content(filepath: String) -> String {
    let path = std::path::Path::new(&filepath);
    let mut file = std::fs::File::open(path).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    content
}
