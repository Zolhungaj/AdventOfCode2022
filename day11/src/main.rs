extern crate core;

use regex::Regex;
use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::Read;
use std::path::Path;

fn main() {
    println!("Hello, world!");
    println!("part_one: {}", part_one("input1.txt"));
    println!("part_one: {}", part_one("input2.txt"));
    println!("part_two: {}", part_two("input2.txt"));
    println!("part_two: {}", part_two("input2.txt"));
}

fn part_one(filepath: &str) -> i32 {
    let content = get_content(filepath.to_string());
    let regex = Regex::new(
        r"Monkey (?P<monkeyID>\d+):
  Starting items:(?P<startingItems>[\d, ]*)
  Operation: new = old (?P<operation>.) (?:(?P<operandOld>old)|(?P<operandConstant>\d+))
  Test: divisible by (?P<divisor>\d+)
    If true: throw to monkey (?P<targetMonkeyIfTrue>\d+)
    If false: throw to monkey (?P<targetMonkeyIfFalse>\d+)",
    )
    .unwrap();
    let starting_items_regex = Regex::new(r"(\d+)").unwrap();
    let mut monkeys: HashMap<i32, Monkey> = HashMap::new();
    for capture in regex.captures_iter(content.as_str()) {
        println!("{:?}", capture);
        let id: i32 = capture.name("monkeyID").unwrap().as_str().parse().unwrap();
        let starting_items = capture.name("startingItems").unwrap().as_str();
        let starting_items: VecDeque<i32> = starting_items_regex
            .captures_iter(starting_items)
            .into_iter()
            .map(|x| x.get(1).unwrap().as_str().parse().unwrap())
            .collect();
        let operation = match capture.name("operation").unwrap().as_str() {
            "+" => Operation::Add,
            "*" => Operation::Multiply,
            _ => panic!(),
        };
        let argument = if capture.name("operandOld").is_some() {
            Argument::Old
        } else if let Some(argument) = capture.name("operandConstant") {
            Argument::Constant(argument.as_str().parse().unwrap())
        } else {
            panic!();
        };
        let divisor: i32 = capture.name("divisor").unwrap().as_str().parse().unwrap();
        let next_monkey_if_true: i32 = capture
            .name("targetMonkeyIfTrue")
            .unwrap()
            .as_str()
            .parse()
            .unwrap();
        let next_monkey_if_false: i32 = capture
            .name("targetMonkeyIfFalse")
            .unwrap()
            .as_str()
            .parse()
            .unwrap();
        let monkey = Monkey {
            items: starting_items,
            count: 0,
            operation,
            argument,
            divisor,
            next_monkey_if_true,
            next_monkey_if_false,
        };
        monkeys.insert(id, monkey);
    }
    for _ in 0..20 {
        for i in 0..monkeys.len() {
            loop {
                let monkey = monkeys.get_mut(&(i as i32)).unwrap();
                if let Some((next_monkey, item)) = monkey.inspect() {
                    let monkey = monkeys.get_mut(&(next_monkey as i32)).unwrap();
                    monkey.add(item);
                } else {
                    break;
                }
            }
        }
    }
    let mut values: Vec<i32> = monkeys.into_values().map(|monkey| monkey.count).collect();
    values.sort();
    for v in &values {
        println!("{v}");
    }
    values.reverse();

    values.drain(0..=1).product()
}

fn part_two(_filepath: &str) -> i32 {
    0
}

fn get_content(filepath: String) -> String {
    let path = Path::new(&filepath);
    let mut file = File::open(path).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    content
}

struct Monkey {
    items: VecDeque<i32>,
    count: i32,
    argument: Argument,
    operation: Operation,
    divisor: i32,
    next_monkey_if_true: i32,
    next_monkey_if_false: i32,
}

impl Monkey {
    fn add(&mut self, item: i32) {
        self.items.push_back(item);
    }

    fn inspect(&mut self) -> Option<(i32, i32)> {
        if let Some(worry_level) = self.items.pop_front() {
            self.count += 1;
            Some(self.decide(worry_level))
        } else {
            None
        }
    }

    fn decide(&self, worry_level: i32) -> (i32, i32) {
        let argument = match self.argument {
            Argument::Old => worry_level,
            Argument::Constant(val) => val,
        };
        //inspection
        let worry_level = match self.operation {
            Operation::Add => worry_level + argument,
            Operation::Multiply => worry_level * argument,
        };

        //post-inspection
        let worry_level = worry_level as f64 / 3.0;
        let worry_level = worry_level.floor() as i32;
        let next_monkey = if worry_level % self.divisor == 0 {
            self.next_monkey_if_true
        } else {
            self.next_monkey_if_false
        };
        (next_monkey, worry_level)
    }
}

enum Argument {
    Old,
    Constant(i32),
}

enum Operation {
    Add,
    Multiply,
}
