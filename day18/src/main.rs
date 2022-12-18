use std::collections::HashSet;
use std::io::Read;

fn main() {
    println!("part_one: {}", part_one("input2.txt"));
    println!("part_one: {}", part_one("input1.txt"));
    // println!("part_two: {}", part_two("input2.txt"));
    // println!("part_two: {}", part_two("input1.txt"));
}

fn part_one(filepath: &str) -> i32 {
    let cubes = extract_content(filepath);
    let mut count = 0;
    let directions = vec![
        (-1, 0, 0),
        (1, 0, 0),
        (0, -1, 0),
        (0, 1, 0),
        (0, 0, -1),
        (0, 0, 1),
    ];
    for cube in &cubes {
        for direction in &directions {
            let other_cube = (
                cube.0 + direction.0,
                cube.1 + direction.1,
                cube.2 + direction.2,
            );
            if !cubes.contains(&other_cube) {
                count += 1;
            }
        }
    }
    count
}

fn extract_content(filepath: &str) -> HashSet<(i32, i32, i32)> {
    let mut hash_set = HashSet::new();
    for line in get_content(filepath.to_string()).lines() {
        let mut coords = line.split(',');
        let cube: (i32, i32, i32) = (
            coords.next().unwrap().parse().unwrap(),
            coords.next().unwrap().parse().unwrap(),
            coords.next().unwrap().parse().unwrap(),
        );
        hash_set.insert(cube);
    }
    hash_set
}

fn get_content(filepath: String) -> String {
    let path = std::path::Path::new(&filepath);
    let mut file = std::fs::File::open(path).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    content
}
