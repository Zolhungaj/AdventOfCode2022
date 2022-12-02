use std::fs::File;
use std::io::Read;
use std::path::Path;

fn main() {
    println!("Part1: {}", part_one("input1.txt"));
    println!("Part1: {}", part_two("input1.txt"));
}

fn part_one(filepath: &str) -> i32 {
    let content = get_content(filepath.to_string());
    let values = content.split('\n');
    let mut sum = 0;
    for line in values {
        if line.is_empty() {
            continue;
        }
        let mut hands: Vec<&str> = line.split(' ').into_iter().collect();
        let my_hand: Hand = hands.pop().unwrap().into();
        let other_hand: Hand = hands.pop().unwrap().into();
        sum += my_hand.round(&other_hand);
    }
    sum
}

fn part_two(filepath: &str) -> i32 {
    let content = get_content(filepath.to_string());
    let values = content.split('\n');
    let mut sum = 0;
    for line in values {
        if line.is_empty() {
            continue;
        }
        let mut hands: Vec<&str> = line.split(' ').into_iter().collect();
        let strategy = hands.pop().unwrap();
        let other_hand: Hand = hands.pop().unwrap().into();
        let my_hand = match strategy {
            "X" => other_hand.wins_against(),
            "Y" => other_hand.clone(),
            "Z" => other_hand.defeated_by(),
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

#[derive(PartialEq, Clone)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

impl Hand {
    fn round(&self, other_hand: &Hand) -> i32 {
        self.result(other_hand).score() + self.intrinsic_value()
    }
    fn result(&self, other_hand: &Hand) -> Result {
        if self.defeated_by() == *other_hand {
            Result::Lose
        } else if other_hand.defeated_by() == *self {
            Result::Win
        } else {
            Result::Draw
        }
    }
    fn intrinsic_value(&self) -> i32 {
        match self {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3,
        }
    }
    fn defeated_by(&self) -> Hand {
        match self {
            Hand::Rock => Hand::Paper,
            Hand::Paper => Hand::Scissors,
            Hand::Scissors => Hand::Rock,
        }
    }
    fn wins_against(&self) -> Hand {
        match self {
            Hand::Rock => Hand::Scissors,
            Hand::Paper => Hand::Rock,
            Hand::Scissors => Hand::Paper,
        }
    }
}

impl From<&str> for Hand {
    fn from(input: &str) -> Self {
        match input {
            "A" => Hand::Rock,
            "B" => Hand::Paper,
            "C" => Hand::Scissors,
            "X" => Hand::Rock,
            "Y" => Hand::Paper,
            "Z" => Hand::Scissors,
            _ => panic!(),
        }
    }
}
