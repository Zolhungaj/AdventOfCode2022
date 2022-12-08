use std::fs::File;
use std::io::Read;
use std::path::Path;

fn main() {
    println!("part_one: {}", part_one("input1.txt"));
    println!("part_two: {}", part_two("input1.txt"));
}

fn part_one(filepath: &str) -> usize {
    let content = get_content(filepath.to_string());
    let mut grid: Vec<Vec<Tree>> = Vec::new();
    for line in content.lines() {
        let mut row: Vec<Tree> = Vec::new();
        for c in line.chars() {
            let c: String = c.into();
            let height: i32 = c.as_str().parse().unwrap();
            print!("{height}");
            let tree = Tree {
                height,
                visible: false,
            };
            row.push(tree);
        }
        println!();
        grid.push(row);
    }
    for vec in grid.iter_mut() {
        let mut min = -1;
        for tree in vec.iter_mut() {
            if tree.height > min {
                min = tree.height;
                tree.visible = true;
            }
        }
        min = -1;
        for tree in vec.iter_mut().rev() {
            if tree.height > min {
                min = tree.height;
                tree.visible = true;
            }
        }
    }
    for x in 0..grid.get(0).unwrap().len() {
        let mut min = -1;
        for vec in &mut grid {
            let mut tree = vec.get_mut(x).unwrap();
            print!("{} {} ->", tree.height, tree.visible);
            if tree.height > min {
                min = tree.height;
                tree.visible = true;
            }
            println!("{}, min={}", tree.visible, min);
        }
        min = -1;
        for vec in grid.iter_mut().rev() {
            let mut tree = vec.get_mut(x).unwrap();
            if tree.height > min {
                min = tree.height;
                tree.visible = true;
            }
        }
    }

    for vec in &grid {
        for tree in vec {
            let c = if tree.visible { "1" } else { "0" };
            print!("{c}");
        }
        println!()
    }

    for vec in &grid {
        for tree in vec {
            let c = tree.height;
            print!("{c}");
        }
        println!()
    }

    grid.into_iter()
        .flat_map(|vec| vec.into_iter())
        .filter(|tree| tree.visible)
        .count()
}

fn part_two(_filepath: &str) -> i32 {
    0
}

struct Tree {
    height: i32,
    visible: bool,
}

fn get_content(filepath: String) -> String {
    let path = Path::new(&filepath);
    let mut file = File::open(path).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    content
}
