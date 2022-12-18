use std::collections::HashSet;
use std::io::Read;

const DIRECTIONS: [(i32, i32, i32); 6] = [
    (-1, 0, 0),
    (1, 0, 0),
    (0, -1, 0),
    (0, 1, 0),
    (0, 0, -1),
    (0, 0, 1),
];

fn main() {
    println!("part_one: {}", part_one("input2.txt"));
    println!("part_one: {}", part_one("input1.txt"));
    println!("part_two: {}", part_two("input2.txt"));
    println!("part_two: {}", part_two("input1.txt"));
}

fn part_one(filepath: &str) -> i32 {
    let cubes = extract_content(filepath);
    let mut count = 0;
    for cube in &cubes {
        for direction in DIRECTIONS {
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

fn part_two(filepath: &str) -> i32 {
    let cubes = extract_content(filepath);
    let steam = flood_fill_steam(&cubes);
    let mut count = 0;
    for cube in cubes {
        for direction in DIRECTIONS {
            let other_cube = (
                cube.0 + direction.0,
                cube.1 + direction.1,
                cube.2 + direction.2,
            );
            if steam.contains(&other_cube) {
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

fn flood_fill_steam(cubes: &HashSet<(i32, i32, i32)>) -> HashSet<(i32, i32, i32)> {
    let mut max_x = i32::MIN;
    let mut max_y = i32::MIN;
    let mut max_z = i32::MIN;
    let mut min_x = i32::MAX;
    let mut min_y = i32::MAX;
    let mut min_z = i32::MAX;

    //create a cube that completely encompasses the droplet
    for cube in cubes {
        max_x = max_x.max(cube.0 + 1);
        min_x = min_x.min(cube.0 - 1);
        max_y = max_y.max(cube.1 + 1);
        min_y = min_y.min(cube.1 - 1);
        max_z = max_z.max(cube.2 + 1);
        min_z = min_z.min(cube.2 - 1);
    }
    let max_x = max_x;
    let max_y = max_y;
    let max_z = max_z;
    let min_x = min_x;
    let min_y = min_y;
    let min_z = min_z;

    let mut steam = HashSet::new();
    let mut next = HashSet::new();
    let seed = (min_x, min_y, min_z);
    next.insert(seed);
    steam.insert(seed);

    while !next.is_empty() {
        let mut next_next = HashSet::new();
        for cube in next.drain() {
            for direction in DIRECTIONS {
                let other_cube = (
                    cube.0 + direction.0,
                    cube.1 + direction.1,
                    cube.2 + direction.2,
                );
                if other_cube.0 <= max_x
                    && other_cube.0 >= min_x
                    && other_cube.1 <= max_y
                    && other_cube.1 >= min_y
                    && other_cube.2 <= max_z
                    && other_cube.2 >= min_z
                    && !steam.contains(&other_cube)
                    && !cubes.contains(&other_cube)
                {
                    steam.insert(other_cube);
                    next_next.insert(other_cube);
                }
            }
        }
        for cube in next_next {
            next.insert(cube);
        }
    }
    steam
}

fn get_content(filepath: String) -> String {
    let path = std::path::Path::new(&filepath);
    let mut file = std::fs::File::open(path).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    content
}
