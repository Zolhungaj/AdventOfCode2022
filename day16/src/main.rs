use regex::Regex;
use std::collections::{HashMap, HashSet, VecDeque};
use std::io::Read;
use std::vec;

fn main() {
    println!("part_one: {}", part_one("input2.txt"));
    // println!("part_one: {}", part_one("input1.txt"));
}

fn part_one(filepath: &str) -> i32 {
    let valve_map = create_valves(filepath);
    println!("{:#?}", valve_map);
    0
}

fn create_valves<'a>(filepath: &str) -> HashMap<String, Valve> {
    let content = get_content(filepath.to_string());
    let regex = Regex::new(r"Valve (?P<valve_name>\w+) has flow rate=(?P<flow_rate>\d+); tunnels? leads? to valves? (?P<tunnels>.*)").unwrap();
    let tunnel_regex = Regex::new(r"\w+").unwrap();
    let mut valve_map = HashMap::new();
    let mut valve_to_neighbour_map: HashMap<String, Vec<String>> = HashMap::new();
    for capture in regex.captures_iter(content.as_str()) {
        let name = capture.name("valve_name").unwrap().as_str();
        let flow_rate = capture.name("flow_rate").unwrap().as_str().parse().unwrap();
        let valve = Valve::new(name.to_string(), flow_rate);
        for capture in tunnel_regex.captures_iter(capture.name("tunnels").unwrap().as_str()) {
            let other_name = capture.get(0).unwrap().as_str();
            if let Some(vec) = valve_to_neighbour_map.get_mut(name) {
                vec.push(other_name.to_string());
            } else {
                let mut vec = Vec::new();
                vec.push(name.to_string());
                valve_to_neighbour_map.insert(name.to_string(), vec);
            }
        }
        valve_map.insert(name.to_string(), valve);
    }
    for (valve_name, neighbour_names) in valve_to_neighbour_map {
        for neighbour_name in neighbour_names {
            let neighbour = valve_map.get(neighbour_name.as_str()).unwrap();
            let valve = valve_map.get_mut(valve_name.as_str()).unwrap();
            valve.neighbours.push(neighbour);
            valve.distances.push(1)
        }
    }
    for valve in valve_map.values_mut() {
        let mut index = 0;
        while index <= valve.neighbours.len() {
            let other_valve = valve.neighbours.get(index).unwrap();
            let distance = valve.distances.get(index).unwrap();
            let other_index = 0;
            while other_index < other_valve.distances.len() {
                let other_valve_distance = other_valve.distances.get(other_index).unwrap();
                let other_valve_neighbour = other_valve.neighbours.get(other_index).unwrap();
                if *other_valve_distance == 1 && !valve.neighbours.contains(other_valve_neighbour) {
                }
            }
        }
    }
    valve_map
}

#[derive(Debug)]
struct Valve<'a> {
    name: String,
    flow_rate: i32,
    neighbours: Vec<&'a Valve<'a>>,
    distances: Vec<i32>,
}

impl<'a> Valve<'a> {
    fn new(name: String, flow_rate: i32) -> Self {
        Valve {
            name,
            flow_rate,
            neighbours: Vec::new(),
            distances: Vec::new(),
        }
    }
}

impl<'a> PartialEq for Valve<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

fn get_content(filepath: String) -> String {
    let path = std::path::Path::new(&filepath);
    let mut file = std::fs::File::open(path).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    content
}
