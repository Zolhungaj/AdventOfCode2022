use std::fs::File;
use std::io::Read;
use std::path::Path;

fn main() {
    println!("Hello, world!");
    println!("part_one: {}", part_one("input2.txt"));
    println!("part_one: {}", part_one("input1.txt"));
    println!("part_two: {}", part_two("input2.txt"));
    println!("part_two: {}", part_two("input1.txt"));
}

fn part_one(filepath: &str) -> usize {
    let (tiles, (start_x, start_y)) = get_tiles(filepath);
    solve(tiles, start_x, start_y)
}

fn part_two(filepath: &str) -> usize {
    let (tiles, (_, _)) = get_tiles(filepath);
    let mut start_points = Vec::new();
    for (y, row) in tiles.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            if tile.height == 0 {
                start_points.push((x, y));
            }
        }
    }
    let mut min = usize::MAX;
    for (x, y) in start_points {
        let mut tiles_clone = Vec::new();
        tiles.clone_into(&mut tiles_clone);
        min = min.min(solve(tiles_clone, x, y));
    }
    min
}

fn solve(mut tiles: Vec<Vec<Tile>>, start_x: usize, start_y: usize) -> usize {
    let start_tile = tiles.get_mut(start_y).unwrap().get_mut(start_x).unwrap();
    start_tile.min_here = 0;
    let x_max = tiles.get(0).unwrap().len();
    let y_max = tiles.len();
    let mut visited = Vec::new();
    let mut current_x = start_x;
    let mut current_y = start_y;
    let mut next_move = 0; //0: up, 1: right, 2: down, 3: left, 4: step back
    let mut min = usize::MAX;
    'outer: loop {
        let current_tile = tiles.get(current_y).unwrap().get(current_x).unwrap();
        if current_tile.end {
            min = current_tile.min_here;
            ((current_x, current_y), next_move) = visited.pop().unwrap();
        }
        let current_tile = tiles.get(current_y).unwrap().get(current_x).unwrap();
        let range_start = next_move;
        for n in range_start..=4 {
            next_move = n;
            let (next_x, next_y) = match next_move {
                0 => {
                    if current_y == 0 {
                        continue;
                    }
                    (current_x, current_y - 1)
                }
                1 => {
                    if current_x == x_max - 1 {
                        continue;
                    }
                    (current_x + 1, current_y)
                }
                2 => {
                    if current_y == y_max - 1 {
                        continue;
                    }
                    (current_x, current_y + 1)
                }
                3 => {
                    if current_x == 0 {
                        continue;
                    }
                    (current_x - 1, current_y)
                }
                4 => {
                    break;
                }
                _ => panic!(),
            };
            let other_tile = tiles.get(next_y).unwrap().get(next_x).unwrap();
            if (other_tile.height == current_tile.height
                || other_tile.height == current_tile.height + 1
                || other_tile.height == 16 && current_tile.height == 18) //input data has no places where a drop is necessary, except 's' to 'q'
                && other_tile.min_here > current_tile.min_here + 1
            {
                visited.push(((current_x, current_y), next_move + 1));
                current_x = next_x;
                current_y = next_y;
                next_move = 0;
                let new_min_here = current_tile.min_here + 1;
                let other_tile = tiles.get_mut(next_y).unwrap().get_mut(next_x).unwrap();
                other_tile.min_here = new_min_here;
                continue 'outer;
            }
        }

        while next_move == 4 {
            if visited.is_empty() {
                break 'outer;
            }
            ((current_x, current_y), next_move) = visited.pop().unwrap();
        }
    }
    min
}

fn get_tiles(filepath: &str) -> (Vec<Vec<Tile>>, (usize, usize)) {
    let content = get_content(filepath.to_string());
    let mut tile_map: Vec<Vec<Tile>> = Vec::new();
    let mut start = (usize::MAX, usize::MAX);
    for (y, line) in content.lines().enumerate() {
        let mut tile_line = Vec::new();
        for (x, c) in line.chars().enumerate() {
            let tile = match c {
                alpha if ('a'..='z').contains(&alpha) => Tile {
                    height: alpha as i32 - b'a' as i32,
                    end: false,
                    min_here: usize::MAX,
                },
                'S' => {
                    start = (x, y);
                    Tile {
                        height: 0,
                        end: false,
                        min_here: usize::MAX,
                    }
                }
                'E' => Tile {
                    height: b'z' as i32 - b'a' as i32,
                    end: true,
                    min_here: usize::MAX,
                },
                _ => panic!(),
            };
            tile_line.push(tile);
        }
        tile_map.push(tile_line);
    }
    (tile_map, start)
}

#[derive(Clone)]
struct Tile {
    height: i32,
    end: bool,
    min_here: usize,
}

fn get_content(filepath: String) -> String {
    let path = Path::new(&filepath);
    let mut file = File::open(path).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    content
}
