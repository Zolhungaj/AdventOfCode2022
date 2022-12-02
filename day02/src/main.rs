use std::fs::File;
use std::io::Read;
use std::path::Path;

fn main() {
    println!("Hello, world!");
    println!("Part1: {}", part_one("input1.txt"));
}

fn part_one(filepath: &str) -> i32 {
    let content = get_content(filepath.to_string());
    let values = content.split('\n');
    let mut sum = 0;
    for line in values {
        if line.is_empty() {
            continue;
        }
        let hands: Vec<&str> = line.split(' ').into_iter().collect();
        let other_hand = hands.first().unwrap();
        let my_hand = hands.last().unwrap();
        let other_hand = match *other_hand {
            "A" => Hand::Rock,
            "B" => Hand::Paper,
            "C" => Hand::Scissors,
            _ => panic!(),
        };
        let my_hand = match *my_hand {
            "X" => Hand::Rock,
            "Y" => Hand::Paper,
            "Z" => Hand::Scissors,
            _ => panic!(),
        };
        sum += my_hand.round(&other_hand);
    }
    sum
}

fn get_content(input: String) -> String {
    let path = Path::new(&input);
    let mut file = File::open(path).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    content
}

#[derive(PartialEq)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

enum Result {
    Win,
    Draw,
    Lose,
}

impl Result {
    fn score(self) -> i32 {
        match self {
            Result::Lose => 0,
            Result::Draw => 3,
            Result::Win => 6,
        }
    }
}

impl Hand {
    fn round(&self, other_hand: &Hand) -> i32 {
        self.result(other_hand).score() + self.intrinsic_value()
    }
    fn result(&self, other_hand: &Hand) -> Result {
        match (self, other_hand) {
            (Hand::Rock, Hand::Rock) => Result::Draw,
            (Hand::Scissors, Hand::Scissors) => Result::Draw,
            (Hand::Paper, Hand::Paper) => Result::Draw,
            (Hand::Rock, Hand::Scissors) => Result::Win,
            (Hand::Scissors, Hand::Paper) => Result::Win,
            (Hand::Paper, Hand::Rock) => Result::Win,
            (Hand::Paper, Hand::Scissors) => Result::Lose,
            (Hand::Rock, Hand::Paper) => Result::Lose,
            (Hand::Scissors, Hand::Rock) => Result::Lose,
        }
    }
    fn intrinsic_value(&self) -> i32 {
        match self {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3,
        }
    }
}
