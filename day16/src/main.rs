use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::io::Read;

fn main() {
    println!("part_one: {}", part_one("input2.txt"));
    // println!("part_one: {}", part_one("input1.txt"));
}

fn part_one(filepath: &str) -> i32 {
    let content = get_content(filepath.to_string());
    let regex = Regex::new(r"Valve (?P<valve_name>\w+) has flow rate=(?P<flow_rate>\d+); tunnels? leads? to valves? (?P<tunnels>.*)").unwrap();
    let tunnel_regex = Regex::new(r"\w+").unwrap();
    let mut valve_map = HashMap::new();
    for capture in regex.captures_iter(content.as_str()) {
        let name = capture.name("valve_name").unwrap().as_str();
        let flow_rate = capture.name("flow_rate").unwrap().as_str().parse().unwrap();
        let mut valve = Valve::new(name.to_string(), flow_rate);
        for capture in tunnel_regex.captures_iter(capture.name("tunnels").unwrap().as_str()) {
            let name = capture.get(0).unwrap().as_str();
            valve.tunnels.insert(name.to_string());
        }
        valve_map.insert(name.to_string(), valve);
    }
    println!("{:?}", valve_map);
    traverse("AA".to_string(), &valve_map, HashSet::new(), 30)
}

fn traverse(
    current_valve: String,
    valve_map: &HashMap<String, Valve>,
    open_valves: HashSet<String>,
    time_left: i32,
) -> i32 {
    if time_left <= 0 {
        return 0;
    }
    let time_left = time_left - 1;
    let mut max = i32::MIN;
    if !open_valves.contains(current_valve.as_str())
        && valve_map.get(current_valve.as_str()).unwrap().flow_rate > 0
    {
        let mut open_valves = open_valves.clone();
        open_valves.insert(current_valve.as_str().to_string());
        max = max.max(traverse(
            current_valve.as_str().to_string(),
            valve_map,
            open_valves,
            time_left,
        ));
    }
    for valve in &valve_map.get(current_valve.as_str()).unwrap().tunnels {
        max = max.max(traverse(
            valve.to_string(),
            valve_map,
            open_valves.clone(),
            time_left,
        ));
    }
    for valve in open_valves {
        max += valve_map.get(valve.as_str()).unwrap().flow_rate;
    }
    max
}

#[derive(Debug)]
struct Valve {
    name: String,
    flow_rate: i32,
    tunnels: HashSet<String>,
}

impl Valve {
    fn new(name: String, flow_rate: i32) -> Self {
        Valve {
            name,
            flow_rate,
            tunnels: HashSet::new(),
        }
    }
}

fn get_content(filepath: String) -> String {
    let path = std::path::Path::new(&filepath);
    let mut file = std::fs::File::open(path).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    content
}
