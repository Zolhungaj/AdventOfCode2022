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
            let tree = Tree {
                height,
                visible: false,
            };
            row.push(tree);
        }
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
            if tree.height > min {
                min = tree.height;
                tree.visible = true;
            }
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
    grid.into_iter()
        .flat_map(|vec| vec.into_iter())
        .filter(|tree| tree.visible)
        .count()
}

fn part_two(filepath: &str) -> i32 {
    let content = get_content(filepath.to_string());
    let mut grid: Vec<Vec<Tree>> = Vec::new();
    for line in content.lines() {
        let mut row: Vec<Tree> = Vec::new();
        for c in line.chars() {
            let c: String = c.into();
            let height: i32 = c.as_str().parse().unwrap();
            let tree = Tree {
                height,
                visible: false,
            };
            row.push(tree);
        }
        grid.push(row);
    }
    let mut best_scenic_score = -1;

    for (tree_x, row) in grid.iter().enumerate() {
        for (tree_y, tree) in row.iter().enumerate() {
            let height = tree.height;
            let mut scenic_score = 1;
            let mut count = 0;
            for y in tree_y + 1..row.len() {
                count += 1;
                if row.get(y).unwrap().height >= height {
                    break;
                }
            }
            scenic_score *= count;
            count = 0;
            for y in (0..tree_y).rev() {
                count += 1;
                if row.get(y).unwrap().height >= height {
                    break;
                }
            }
            scenic_score *= count;
            count = 0;
            for x in tree_x + 1..grid.len() {
                count += 1;
                if grid.get(x).unwrap().get(tree_y).unwrap().height >= height {
                    break;
                }
            }
            scenic_score *= count;
            count = 0;
            for x in (0..tree_x).rev() {
                count += 1;
                if grid.get(x).unwrap().get(tree_y).unwrap().height >= height {
                    break;
                }
            }
            scenic_score *= count;
            best_scenic_score = best_scenic_score.max(scenic_score);
        }
    }

    best_scenic_score
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
