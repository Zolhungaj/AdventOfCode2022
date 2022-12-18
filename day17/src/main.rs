use std::collections::VecDeque;
use std::fmt::{Display, Formatter};
use std::io::Read;

fn main() {
    println!("part_one: {}", part_one("input2.txt"));
    // println!("part_one: {}", part_one("input1.txt"));
    // println!("part_two: {}", part_two("input2.txt"));
    // println!("test: {}", test("input2.txt"));
    // println!("part_two: {}", part_two("input1.txt"));
}

fn test(filepath: &str) -> usize {
    let mut jet_generator = get_jet_generator(filepath);
    let (base, floors, current_position) =
        simulate_shaft_with_floor(40, jet_generator.clone(), vec![vec![Tile::Dead; 7]]);
    println!("!!!!");
    jet_generator.current_position = current_position;
    let (base, floors, _) = simulate_shaft_with_floor(0, jet_generator.clone(), floors);
    0
}

fn part_one(filepath: &str) -> usize {
    simulate_shaft(2022, get_jet_generator(filepath))
}

fn part_two(filepath: &str) -> usize {
    let jet_generator = get_jet_generator(filepath);
    let target: usize = 2022;
    let (height, iterations) = simulate_until_condition(jet_generator.clone());
    let possible_to_skip = target / iterations; //total amount we can actually skip
    let skipped = possible_to_skip * height;
    let remaining_iterations2 = target - (possible_to_skip * iterations); //remaining iterations that have to be simulated
    let excess = simulate_shaft(remaining_iterations2, jet_generator.clone());
    return skipped + excess;
}

fn simulate_shaft(target: usize, mut jet_generator: JetGenerator) -> usize {
    let floor = vec![Tile::Dead; 7];
    let floors = vec![floor];
    simulate_shaft_with_floor(target, jet_generator, floors).0 - 1
}

fn simulate_shaft_with_floor(
    target: usize,
    mut jet_generator: JetGenerator,
    floors: Vec<Vec<Tile>>,
) -> (usize, Vec<Vec<Tile>>, usize) {
    let mut shaft = VecDeque::new();
    let width = 7;
    for floor in floors {
        shaft.push_back(floor);
    }
    for rock in RockGenerator::new(target) {
        for _ in 0..3 {
            shaft.push_front(vec![Tile::Empty; width]); //three empty rows before next rock
        }
        match rock {
            //note that the order here is bottom to top
            Rock::Flat => {
                shaft.push_front(vec![
                    Tile::Empty,
                    Tile::Empty,
                    Tile::Live,
                    Tile::Live,
                    Tile::Live,
                    Tile::Live,
                    Tile::Empty,
                ]);
            }
            Rock::Cross => {
                shaft.push_front(vec![
                    Tile::Empty,
                    Tile::Empty,
                    Tile::Empty,
                    Tile::Live,
                    Tile::Empty,
                    Tile::Empty,
                    Tile::Empty,
                ]);
                shaft.push_front(vec![
                    Tile::Empty,
                    Tile::Empty,
                    Tile::Live,
                    Tile::Live,
                    Tile::Live,
                    Tile::Empty,
                    Tile::Empty,
                ]);
                shaft.push_front(vec![
                    Tile::Empty,
                    Tile::Empty,
                    Tile::Empty,
                    Tile::Live,
                    Tile::Empty,
                    Tile::Empty,
                    Tile::Empty,
                ]);
            }
            Rock::Angle => {
                shaft.push_front(vec![
                    Tile::Empty,
                    Tile::Empty,
                    Tile::Live,
                    Tile::Live,
                    Tile::Live,
                    Tile::Empty,
                    Tile::Empty,
                ]);
                shaft.push_front(vec![
                    Tile::Empty,
                    Tile::Empty,
                    Tile::Empty,
                    Tile::Empty,
                    Tile::Live,
                    Tile::Empty,
                    Tile::Empty,
                ]);
                shaft.push_front(vec![
                    Tile::Empty,
                    Tile::Empty,
                    Tile::Empty,
                    Tile::Empty,
                    Tile::Live,
                    Tile::Empty,
                    Tile::Empty,
                ]);
            }
            Rock::Long => {
                shaft.push_front(vec![
                    Tile::Empty,
                    Tile::Empty,
                    Tile::Live,
                    Tile::Empty,
                    Tile::Empty,
                    Tile::Empty,
                    Tile::Empty,
                ]);
                shaft.push_front(vec![
                    Tile::Empty,
                    Tile::Empty,
                    Tile::Live,
                    Tile::Empty,
                    Tile::Empty,
                    Tile::Empty,
                    Tile::Empty,
                ]);
                shaft.push_front(vec![
                    Tile::Empty,
                    Tile::Empty,
                    Tile::Live,
                    Tile::Empty,
                    Tile::Empty,
                    Tile::Empty,
                    Tile::Empty,
                ]);
                shaft.push_front(vec![
                    Tile::Empty,
                    Tile::Empty,
                    Tile::Live,
                    Tile::Empty,
                    Tile::Empty,
                    Tile::Empty,
                    Tile::Empty,
                ]);
            }
            Rock::Square => {
                shaft.push_front(vec![
                    Tile::Empty,
                    Tile::Empty,
                    Tile::Live,
                    Tile::Live,
                    Tile::Empty,
                    Tile::Empty,
                    Tile::Empty,
                ]);
                shaft.push_front(vec![
                    Tile::Empty,
                    Tile::Empty,
                    Tile::Live,
                    Tile::Live,
                    Tile::Empty,
                    Tile::Empty,
                    Tile::Empty,
                ]);
            }
        }
        //bounding box: top left corner to bottom right corner, to save on space to check, x is to the right, y is down
        //top left corner of bounding box always starts out at (2,0)
        let mut bounding_box = (
            (2, 0),
            match rock {
                Rock::Flat => (5, 0),
                Rock::Cross | Rock::Angle => (4, 2),
                Rock::Long => (2, 3),
                Rock::Square => (3, 1),
            },
        );
        loop {
            let direction = jet_generator.next();
            // println!("{:?}", direction);
            match direction {
                Direction::Left => {
                    if bounding_box.0 .0 != 0 {
                        //top left corner x amplitude
                        let mut can_move = true;
                        for x in bounding_box.0 .0..=bounding_box.1 .0 {
                            for y in bounding_box.0 .1..=bounding_box.1 .1 {
                                let tile = shaft.get(y).unwrap().get(x).unwrap();
                                let other_tile = shaft.get(y).unwrap().get(x - 1).unwrap();
                                if *tile == Tile::Live && *other_tile == Tile::Dead {
                                    can_move = false;
                                }
                            }
                        }
                        if can_move {
                            //move all live tiles one to the left, starting from the left
                            for x in bounding_box.0 .0..=bounding_box.1 .0 {
                                for y in bounding_box.0 .1..=bounding_box.1 .1 {
                                    let tile = shaft.get_mut(y).unwrap().get_mut(x).unwrap();
                                    if *tile == Tile::Live {
                                        *tile = Tile::Empty;
                                        let tile =
                                            shaft.get_mut(y).unwrap().get_mut(x - 1).unwrap();
                                        *tile = Tile::Live;
                                    }
                                }
                            }
                            bounding_box = (
                                (bounding_box.0 .0 - 1, bounding_box.0 .1),
                                (bounding_box.1 .0 - 1, bounding_box.1 .1),
                            );
                        }
                    }
                }
                Direction::Right => {
                    if bounding_box.1 .0 != width - 1 {
                        //bottom right corner x amplitude
                        let mut can_move = true;
                        for x in bounding_box.0 .0..=bounding_box.1 .0 {
                            for y in bounding_box.0 .1..=bounding_box.1 .1 {
                                let tile = shaft.get(y).unwrap().get(x).unwrap();
                                let other_tile = shaft.get(y).unwrap().get(x + 1).unwrap();
                                if *tile == Tile::Live && *other_tile == Tile::Dead {
                                    can_move = false;
                                }
                            }
                        }
                        if can_move {
                            //move all live tiles one to the right, starting from the right
                            for x in (bounding_box.0 .0..=bounding_box.1 .0).rev() {
                                for y in bounding_box.0 .1..=bounding_box.1 .1 {
                                    let tile = shaft.get_mut(y).unwrap().get_mut(x).unwrap();
                                    if *tile == Tile::Live {
                                        *tile = Tile::Empty;
                                        let tile =
                                            shaft.get_mut(y).unwrap().get_mut(x + 1).unwrap();
                                        *tile = Tile::Live;
                                    }
                                }
                            }
                            bounding_box = (
                                (bounding_box.0 .0 + 1, bounding_box.0 .1),
                                (bounding_box.1 .0 + 1, bounding_box.1 .1),
                            );
                        }
                    }
                }
            };
            let mut can_move = true;
            for x in bounding_box.0 .0..=bounding_box.1 .0 {
                for y in bounding_box.0 .1..=bounding_box.1 .1 {
                    let tile = shaft.get(y).unwrap().get(x).unwrap();
                    let other_tile = shaft.get(y + 1).unwrap().get(x).unwrap();
                    if *tile == Tile::Live && *other_tile == Tile::Dead {
                        can_move = false;
                    }
                }
            }
            // for line in &shaft {
            //     for tile in line {
            //         print!("{}", tile);
            //     }
            //     println!();
            // }
            if can_move {
                for x in bounding_box.0 .0..=bounding_box.1 .0 {
                    for y in (bounding_box.0 .1..=bounding_box.1 .1).rev() {
                        let tile = shaft.get_mut(y).unwrap().get_mut(x).unwrap();
                        if *tile == Tile::Live {
                            *tile = Tile::Empty;
                            let tile = shaft.get_mut(y + 1).unwrap().get_mut(x).unwrap();
                            *tile = Tile::Live;
                        }
                    }
                }
                bounding_box = (
                    (bounding_box.0 .0, bounding_box.0 .1 + 1),
                    (bounding_box.1 .0, bounding_box.1 .1 + 1),
                );
            } else {
                for x in bounding_box.0 .0..=bounding_box.1 .0 {
                    for y in bounding_box.0 .1..=bounding_box.1 .1 {
                        let tile = shaft.get_mut(y).unwrap().get_mut(x).unwrap();
                        if *tile == Tile::Live {
                            *tile = Tile::Dead;
                        }
                    }
                }
                break;
            }
        }
        'outer: loop {
            shaft.pop_front();
            for x in shaft.get(0).unwrap() {
                if *x == Tile::Dead {
                    break 'outer;
                }
            }
        }
    }
    for line in &shaft {
        for tile in line {
            print!("{}", tile);
        }
        println!();
    }
    let height = shaft.len();
    let mut floors = Vec::new();
    for _ in 0..20 {
        if !shaft.is_empty() {
            floors.push(shaft.pop_front().unwrap());
        }
    }
    // println!("{:#?}", jet_generator);
    (height, floors, jet_generator.current_position)
}

fn simulate_until_condition(mut jet_generator: JetGenerator) -> (usize, usize) {
    let mut shaft = VecDeque::new();
    let width = 7;
    shaft.push_back(vec![Tile::Dead; 7]);
    let mut count = 0;
    for rock in RockGenerator::new(1000000) {
        count += 1;
        for _ in 0..3 {
            shaft.push_front(vec![Tile::Empty; width]); //three empty rows before next rock
        }
        match rock {
            //note that the order here is bottom to top
            Rock::Flat => {
                shaft.push_front(vec![
                    Tile::Empty,
                    Tile::Empty,
                    Tile::Live,
                    Tile::Live,
                    Tile::Live,
                    Tile::Live,
                    Tile::Empty,
                ]);
            }
            Rock::Cross => {
                shaft.push_front(vec![
                    Tile::Empty,
                    Tile::Empty,
                    Tile::Empty,
                    Tile::Live,
                    Tile::Empty,
                    Tile::Empty,
                    Tile::Empty,
                ]);
                shaft.push_front(vec![
                    Tile::Empty,
                    Tile::Empty,
                    Tile::Live,
                    Tile::Live,
                    Tile::Live,
                    Tile::Empty,
                    Tile::Empty,
                ]);
                shaft.push_front(vec![
                    Tile::Empty,
                    Tile::Empty,
                    Tile::Empty,
                    Tile::Live,
                    Tile::Empty,
                    Tile::Empty,
                    Tile::Empty,
                ]);
            }
            Rock::Angle => {
                shaft.push_front(vec![
                    Tile::Empty,
                    Tile::Empty,
                    Tile::Live,
                    Tile::Live,
                    Tile::Live,
                    Tile::Empty,
                    Tile::Empty,
                ]);
                shaft.push_front(vec![
                    Tile::Empty,
                    Tile::Empty,
                    Tile::Empty,
                    Tile::Empty,
                    Tile::Live,
                    Tile::Empty,
                    Tile::Empty,
                ]);
                shaft.push_front(vec![
                    Tile::Empty,
                    Tile::Empty,
                    Tile::Empty,
                    Tile::Empty,
                    Tile::Live,
                    Tile::Empty,
                    Tile::Empty,
                ]);
            }
            Rock::Long => {
                shaft.push_front(vec![
                    Tile::Empty,
                    Tile::Empty,
                    Tile::Live,
                    Tile::Empty,
                    Tile::Empty,
                    Tile::Empty,
                    Tile::Empty,
                ]);
                shaft.push_front(vec![
                    Tile::Empty,
                    Tile::Empty,
                    Tile::Live,
                    Tile::Empty,
                    Tile::Empty,
                    Tile::Empty,
                    Tile::Empty,
                ]);
                shaft.push_front(vec![
                    Tile::Empty,
                    Tile::Empty,
                    Tile::Live,
                    Tile::Empty,
                    Tile::Empty,
                    Tile::Empty,
                    Tile::Empty,
                ]);
                shaft.push_front(vec![
                    Tile::Empty,
                    Tile::Empty,
                    Tile::Live,
                    Tile::Empty,
                    Tile::Empty,
                    Tile::Empty,
                    Tile::Empty,
                ]);
            }
            Rock::Square => {
                shaft.push_front(vec![
                    Tile::Empty,
                    Tile::Empty,
                    Tile::Live,
                    Tile::Live,
                    Tile::Empty,
                    Tile::Empty,
                    Tile::Empty,
                ]);
                shaft.push_front(vec![
                    Tile::Empty,
                    Tile::Empty,
                    Tile::Live,
                    Tile::Live,
                    Tile::Empty,
                    Tile::Empty,
                    Tile::Empty,
                ]);
            }
        }
        //bounding box: top left corner to bottom right corner, to save on space to check, x is to the right, y is down
        //top left corner of bounding box always starts out at (2,0)
        let mut bounding_box = (
            (2, 0),
            match rock {
                Rock::Flat => (5, 0),
                Rock::Cross | Rock::Angle => (4, 2),
                Rock::Long => (2, 3),
                Rock::Square => (3, 1),
            },
        );
        loop {
            let direction = jet_generator.next();
            // println!("{:?}", direction);
            match direction {
                Direction::Left => {
                    if bounding_box.0 .0 != 0 {
                        //top left corner x amplitude
                        let mut can_move = true;
                        for x in bounding_box.0 .0..=bounding_box.1 .0 {
                            for y in bounding_box.0 .1..=bounding_box.1 .1 {
                                let tile = shaft.get(y).unwrap().get(x).unwrap();
                                let other_tile = shaft.get(y).unwrap().get(x - 1).unwrap();
                                if *tile == Tile::Live && *other_tile == Tile::Dead {
                                    can_move = false;
                                }
                            }
                        }
                        if can_move {
                            //move all live tiles one to the left, starting from the left
                            for x in bounding_box.0 .0..=bounding_box.1 .0 {
                                for y in bounding_box.0 .1..=bounding_box.1 .1 {
                                    let tile = shaft.get_mut(y).unwrap().get_mut(x).unwrap();
                                    if *tile == Tile::Live {
                                        *tile = Tile::Empty;
                                        let tile =
                                            shaft.get_mut(y).unwrap().get_mut(x - 1).unwrap();
                                        *tile = Tile::Live;
                                    }
                                }
                            }
                            bounding_box = (
                                (bounding_box.0 .0 - 1, bounding_box.0 .1),
                                (bounding_box.1 .0 - 1, bounding_box.1 .1),
                            );
                        }
                    }
                }
                Direction::Right => {
                    if bounding_box.1 .0 != width - 1 {
                        //bottom right corner x amplitude
                        let mut can_move = true;
                        for x in bounding_box.0 .0..=bounding_box.1 .0 {
                            for y in bounding_box.0 .1..=bounding_box.1 .1 {
                                let tile = shaft.get(y).unwrap().get(x).unwrap();
                                let other_tile = shaft.get(y).unwrap().get(x + 1).unwrap();
                                if *tile == Tile::Live && *other_tile == Tile::Dead {
                                    can_move = false;
                                }
                            }
                        }
                        if can_move {
                            //move all live tiles one to the right, starting from the right
                            for x in (bounding_box.0 .0..=bounding_box.1 .0).rev() {
                                for y in bounding_box.0 .1..=bounding_box.1 .1 {
                                    let tile = shaft.get_mut(y).unwrap().get_mut(x).unwrap();
                                    if *tile == Tile::Live {
                                        *tile = Tile::Empty;
                                        let tile =
                                            shaft.get_mut(y).unwrap().get_mut(x + 1).unwrap();
                                        *tile = Tile::Live;
                                    }
                                }
                            }
                            bounding_box = (
                                (bounding_box.0 .0 + 1, bounding_box.0 .1),
                                (bounding_box.1 .0 + 1, bounding_box.1 .1),
                            );
                        }
                    }
                }
            };
            let mut can_move = true;
            for x in bounding_box.0 .0..=bounding_box.1 .0 {
                for y in bounding_box.0 .1..=bounding_box.1 .1 {
                    let tile = shaft.get(y).unwrap().get(x).unwrap();
                    let other_tile = shaft.get(y + 1).unwrap().get(x).unwrap();
                    if *tile == Tile::Live && *other_tile == Tile::Dead {
                        can_move = false;
                    }
                }
            }
            // for line in &shaft {
            //     for tile in line {
            //         print!("{}", tile);
            //     }
            //     println!();
            // }
            if can_move {
                for x in bounding_box.0 .0..=bounding_box.1 .0 {
                    for y in (bounding_box.0 .1..=bounding_box.1 .1).rev() {
                        let tile = shaft.get_mut(y).unwrap().get_mut(x).unwrap();
                        if *tile == Tile::Live {
                            *tile = Tile::Empty;
                            let tile = shaft.get_mut(y + 1).unwrap().get_mut(x).unwrap();
                            *tile = Tile::Live;
                        }
                    }
                }
                bounding_box = (
                    (bounding_box.0 .0, bounding_box.0 .1 + 1),
                    (bounding_box.1 .0, bounding_box.1 .1 + 1),
                );
            } else {
                for x in bounding_box.0 .0..=bounding_box.1 .0 {
                    for y in bounding_box.0 .1..=bounding_box.1 .1 {
                        let tile = shaft.get_mut(y).unwrap().get_mut(x).unwrap();
                        if *tile == Tile::Live {
                            *tile = Tile::Dead;
                        }
                    }
                }
                break;
            }
        }
        'outer: loop {
            shaft.pop_front();
            for x in shaft.get(0).unwrap() {
                if *x == Tile::Dead {
                    break 'outer;
                }
            }
        }
        if jet_generator.current_position == 0 && rock == Rock::Square {
            let last_line = shaft.get(0).unwrap().to_owned();
            for tile in &last_line {
                print!("{}", tile);
            }
            println!();
            let safety_floor = vec![Tile::Dead; 7];
            let floors = vec![last_line, safety_floor];
            let (height, _, _) = simulate_shaft_with_floor(1, jet_generator.clone(), floors);
            if height == 3 {
                //next item is line
                //line is guaranteed to land as if floor is whole
                //jet_generator is at default position
                return (shaft.len(), count);
            }
        }
    }
    panic!();
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
#[derive(Debug, Clone)]
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
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Tile {
    Live,
    Dead,
    Empty,
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let symbol = match self {
            Tile::Live => "@",
            Tile::Dead => "#",
            Tile::Empty => ".",
        };
        write!(f, "{}", symbol)
    }
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
            let current = Some(self.current_rock);
            self.current_rock = match self.current_rock {
                Rock::Flat => Rock::Cross,
                Rock::Cross => Rock::Angle,
                Rock::Angle => Rock::Long,
                Rock::Long => Rock::Square,
                Rock::Square => Rock::Flat,
            };
            self.count += 1;
            current
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
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
