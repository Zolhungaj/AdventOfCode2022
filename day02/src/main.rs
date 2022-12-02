use std::fs::File;
use std::io::Read;
use std::path::Path;

fn main() {
    println!("Part1: {}", part_one("input1.txt"));
    println!("Part1: {}", part_two("input1.txt"));
}

fn part_one(filepath: &str) -> i32 {
    get_inputs(filepath)
        .into_iter()
        .map(|(other_hand, my_hand): (String, String)| (other_hand.into(), my_hand.into()))
        .map(|(other_hand, my_hand): (Hand, Hand)| my_hand.round(other_hand))
        .sum()
}

fn part_two(filepath: &str) -> i32 {
    get_inputs(filepath)
        .into_iter()
        .map(|(other_hand, strategy): (String, String)| (other_hand.into(), strategy))
        .map(|(other_hand, strategy): (Hand, String)| {
            (
                other_hand,
                match strategy.as_str() {
                    "X" => other_hand.wins_against(),
                    "Y" => other_hand.draws_against(),
                    "Z" => other_hand.loses_against(),
                    _ => panic!(),
                },
            )
        })
        .map(|(other_hand, my_hand): (Hand, Hand)| my_hand.round(other_hand))
        .sum()
}

fn get_inputs(filepath: &str) -> Vec<(String, String)> {
    let content = get_content(filepath.to_string());
    let values = content.split('\n');
    values
        .into_iter()
        .filter(|line| !line.is_empty()) //empty lines are ignoreable
        .map(|line| line.split(' ')) //divide into left and right
        .map(|mut split| (split.next().unwrap(), split.next().unwrap())) //turn left and right input into tuple
        .map(|(x, y)| (x.to_string(), y.to_string())) //create owned String to be able to return
        .collect()
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

#[derive(PartialEq, Clone, Copy)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

impl Hand {
    fn round(self, other_hand: Hand) -> i32 {
        self.result(&other_hand).score() + self.intrinsic_value()
    }
    fn result(&self, other_hand: &Hand) -> Result {
        if self.loses_against() == *other_hand {
            Result::Lose
        } else if other_hand.loses_against() == *self {
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
    fn loses_against(&self) -> Hand {
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
    fn draws_against(&self) -> Hand {
        *self
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

impl From<String> for Hand {
    fn from(input: String) -> Self {
        input.as_str().into()
    }
}
