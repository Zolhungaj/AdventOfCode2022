use std::io::Read;

fn main() {
    println!("part_one: {}", part_one("input2.txt"));
    // println!("part_one: {}", part_one("input1.txt"));
}

fn part_one(filepath: &str) -> usize {
    let jet_generator = get_jet_generator(filepath);
    //width: 7
    //spawn: left edge of rock two units from left edge of chamber
    //       bottom edge: three above top rock
    0
}

fn get_jet_generator(filepath: &str) -> JetGenerator {
    let content = get_content(filepath.to_string());
    let line = content.lines().next().unwrap();
    let mut directions = Vec::new();
    for c in line.chars() {
        directions.push(match c {
            '>' => Direction::Right,
            '<' => Direction::Left,
            _ => panic!(),
        })
    }
    JetGenerator::new(directions)
}

struct JetGenerator {
    directions: Vec<Direction>,
    current_position: usize,
    length: usize,
}

impl JetGenerator {
    fn new(directions: Vec<Direction>) -> Self {
        if directions.is_empty() {
            panic!();
        }
        Self {
            length: directions.len(),
            directions,
            current_position: 0,
        }
    }
    fn next(&mut self) -> Direction {
        let direction = self
            .directions
            .get(self.current_position)
            .unwrap()
            .to_owned();
        self.current_position += 1;
        self.current_position %= self.length;
        direction
    }
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

struct RockGenerator {
    current_rock: Rock,
    count: usize,
    target: usize,
}

impl RockGenerator {
    fn new(target: usize) -> Self {
        Self {
            current_rock: Rock::Flat,
            count: 0,
            target,
        }
    }
}

impl Iterator for RockGenerator {
    type Item = Rock;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count == self.target {
            None
        } else {
            self.current_rock = match self.current_rock {
                Rock::Flat => Rock::Cross,
                Rock::Cross => Rock::Angle,
                Rock::Angle => Rock::Long,
                Rock::Long => Rock::Square,
                Rock::Square => Rock::Flat,
            };
            Some(self.current_rock)
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Rock {
    Flat,
    Cross,
    Angle,
    Long,
    Square,
}

fn get_content(filepath: String) -> String {
    let path = std::path::Path::new(&filepath);
    let mut file = std::fs::File::open(path).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    content
}
