use std::fs::File;
use std::io::Read;
use std::path::Path;

fn main() {
    println!("Hello, world!");
}

fn get_lines(filepath: &str) -> Vec<String> {
    get_content(filepath.to_string())
        .lines()
        .map(|x| x.to_string())
        .collect()
}

fn get_content(filepath: String) -> String {
    let path = Path::new(&filepath);
    let mut file = File::open(path).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    content
}
