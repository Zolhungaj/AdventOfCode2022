extern crate core;

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
    let content = get_content(filepath.to_string());
    let mut grid = Grid::new(1000, 300);
    for line in content.lines() {
        let mut points = line.split(" -> ");
        let mut first_point = points.next().unwrap().split(',');
        let first_point: (usize, usize) = (
            first_point.next().unwrap().parse().unwrap(),
            first_point.next().unwrap().parse().unwrap(),
        );
        let mut current_point = first_point;
        for point in points {
            let (x, y) = current_point;
            let mut point = point.split(',');
            let (other_x, other_y): (usize, usize) = (
                point.next().unwrap().parse().unwrap(),
                point.next().unwrap().parse().unwrap(),
            );
            if x != other_x {
                if x < other_x {
                    for x in x..=other_x {
                        print!("{:?}", (x, y));
                        grid.set_point((x, y), Content::Rock);
                    }
                } else {
                    for x in other_x..=x {
                        print!("{:?}", (x, y));
                        grid.set_point((x, y), Content::Rock);
                    }
                }
            } else if y != other_y {
                if y < other_y {
                    for y in y..=other_y {
                        print!("{:?}", (x, y));
                        grid.set_point((x, y), Content::Rock);
                    }
                } else {
                    for y in other_y..=y {
                        print!("{:?}", (x, y));
                        grid.set_point((x, y), Content::Rock);
                    }
                }
            } else {
                grid.set_point((x, y), Content::Rock);
            }
            current_point = (other_x, other_y);
            println!();
        }
    }
    let mut count = 0;
    'outer: loop {
        let start_point = (500, 0);
        if grid.get_point(start_point).unwrap().content == Content::Sand {
            break;
        }
        grid.set_point(start_point, Content::Sand);
        count += 1;
        let mut current_point = start_point;
        loop {
            if let Some(point) = grid.update_point(current_point) {
                if point == current_point {
                    break;
                } else {
                    current_point = point;
                }
            } else {
                break 'outer;
            }
        }
    }
    grid.print();
    count - 1
}

fn part_two(filepath: &str) -> usize {
    let content = get_content(filepath.to_string());
    let mut grid = Grid::new(1000, 300);
    let mut max_y = 0;
    for line in content.lines() {
        let mut points = line.split(" -> ");
        let mut first_point = points.next().unwrap().split(',');
        let first_point: (usize, usize) = (
            first_point.next().unwrap().parse().unwrap(),
            first_point.next().unwrap().parse().unwrap(),
        );
        let mut current_point = first_point;
        max_y = max_y.max(current_point.1);
        for point in points {
            let (x, y) = current_point;
            let mut point = point.split(',');
            let (other_x, other_y): (usize, usize) = (
                point.next().unwrap().parse().unwrap(),
                point.next().unwrap().parse().unwrap(),
            );
            if x != other_x {
                if x < other_x {
                    for x in x..=other_x {
                        print!("{:?}", (x, y));
                        grid.set_point((x, y), Content::Rock);
                    }
                } else {
                    for x in other_x..=x {
                        print!("{:?}", (x, y));
                        grid.set_point((x, y), Content::Rock);
                    }
                }
            } else if y != other_y {
                if y < other_y {
                    for y in y..=other_y {
                        print!("{:?}", (x, y));
                        grid.set_point((x, y), Content::Rock);
                    }
                } else {
                    for y in other_y..=y {
                        print!("{:?}", (x, y));
                        grid.set_point((x, y), Content::Rock);
                    }
                }
            } else {
                grid.set_point((x, y), Content::Rock);
            }
            current_point = (other_x, other_y);
            max_y = max_y.max(current_point.1);
            println!();
        }
    }

    for x in 0..1000 {
        grid.get_point_mut((x, max_y + 2)).unwrap().content = Content::Rock;
    }

    let mut count = 0;
    'outer: loop {
        let start_point = (500, 0);
        if grid.get_point(start_point).unwrap().content == Content::Sand {
            break;
        }
        grid.set_point(start_point, Content::Sand);
        count += 1;
        let mut current_point = start_point;
        loop {
            if let Some(point) = grid.update_point(current_point) {
                if point == current_point {
                    break;
                } else {
                    current_point = point;
                }
            } else {
                break 'outer;
            }
        }
    }
    //grid.print();
    count
}

struct Grid {
    points: Vec<Vec<Point>>,
    x_max: usize,
    y_max: usize,
    x_min: usize,
    y_min: usize,
}

impl Grid {
    fn print(&self) {
        for row in &self.points {
            for point in row {
                print!(
                    "{}",
                    match point.content {
                        Content::Sand => "o",
                        Content::Rock => "#",
                        Content::Air => ".",
                    }
                )
            }
            println!();
        }
    }

    fn new(x_max: usize, y_max: usize) -> Grid {
        let y_min = 0;
        let x_min = 0;

        let mut points = Vec::new();
        for y in 0..y_max {
            let mut row = Vec::new();
            for x in 0..x_max {
                let point = Point {
                    x,
                    y,
                    content: Content::Air,
                };
                row.push(point);
            }
            points.push(row);
        }
        Grid {
            points,
            x_max,
            x_min,
            y_min,
            y_max,
        }
    }

    fn set_point(&mut self, (x, y): (usize, usize), content: Content) {
        self.get_point_mut((x, y)).unwrap().content = content
    }

    fn update_point(&mut self, (x, y): (usize, usize)) -> Option<(usize, usize)> {
        let point = self.get_point((x, y));
        point.as_ref()?;
        let point = point.unwrap();
        match point.content {
            Content::Air | Content::Rock => {}
            Content::Sand => {
                let options = [0, -1, 1];
                let other_y = y + 1;
                for n in options {
                    if x as isize + n < self.x_min as isize
                        || other_y >= self.y_max
                        || x as isize + n >= self.x_max as isize
                        || other_y < self.y_min
                    {
                        //falls into the void
                        self.get_point_mut((x, y)).unwrap().content = Content::Air;
                        return None;
                    } else {
                        let other_x = (x as isize + n) as usize;
                        let other_point = self.get_point((other_x, other_y));
                        if let Some(other_point) = other_point {
                            match other_point.content {
                                Content::Sand | Content::Rock => {}
                                Content::Air => {
                                    self.get_point_mut((x, y)).unwrap().content = Content::Air;
                                    self.get_point_mut((other_x, other_y)).unwrap().content =
                                        Content::Sand;
                                    return Some((other_x, other_y));
                                }
                            }
                        }
                    }
                }
                return Some((x, y));
            }
        }
        None
    }

    fn get_point(&self, (x, y): (usize, usize)) -> Option<&Point> {
        self.points.get(y).and_then(|row| row.get(x))
    }

    fn get_point_mut(&mut self, (x, y): (usize, usize)) -> Option<&mut Point> {
        self.points.get_mut(y).and_then(|row| row.get_mut(x))
    }
}

struct Point {
    x: usize,
    y: usize,
    content: Content,
}

#[derive(PartialEq)]
enum Content {
    Sand,
    Rock,
    Air,
}

fn get_content(filepath: String) -> String {
    let path = Path::new(&filepath);
    let mut file = File::open(path).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    content
}
