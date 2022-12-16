use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::io::Read;

fn main() {
    // println!("part_one: {}", part_one("input2.txt"));
    // println!("part_one: {}", part_one("input1.txt"));
    println!("part_two: {}", part_two("input2.txt"));
    // println!("part_two: {}", part_two("input1.txt"));
}

fn part_one(filepath: &str) -> i32 {
    let valve_map = create_valves(filepath);
    println!("{:#?}", valve_map);
    traverse("AA", &valve_map, HashSet::new(), 30, false)
}

fn part_two(filepath: &str) -> i32 {
    let valve_map = create_valves(filepath);
    println!("{:#?}", valve_map);
    let target = valve_map.get("AA").unwrap().accessible_nodes.len();
    traverse_pair("AA", "AA", 0, 0, &valve_map, HashSet::new(), 26, target)
}

fn traverse(
    current_valve: &str,
    valve_map: &HashMap<String, Valve>,
    mut visited: HashSet<String>,
    time_left: i32,
    is_self: bool,
) -> i32 {
    if time_left <= 0 {
        return 0;
    }
    let valve = valve_map.get(current_valve).unwrap();
    let mut max = 0;
    if !is_self && valve.flow_rate > 0 {
        let score = valve.flow_rate * (time_left - 1)
            + traverse(
                current_valve,
                valve_map,
                visited.clone(),
                time_left - 1,
                true,
            );
        max = max.max(score)
    }
    let mut index = 0;
    visited.insert(current_valve.to_string());
    while index < valve.accessible_nodes.len() {
        let neighbour_name = valve.accessible_nodes.get(index).unwrap();
        let neighbour_distance = valve.distances.get(index).unwrap();
        let time_left_after_move = time_left - neighbour_distance;
        if !visited.contains(neighbour_name) && time_left_after_move >= 0 {
            max = max.max(traverse(
                neighbour_name,
                valve_map,
                visited.clone(),
                time_left_after_move,
                false,
            ));
        }
        index += 1
    }
    max
}

fn traverse_pair(
    me_value: &str,
    elephant_value: &str,
    me_state: i32,
    elephant_state: i32,
    valve_map: &HashMap<String, Valve>,
    visited: HashSet<String>,
    time_left: i32,
    target: usize,
) -> i32 {
    let mut do_not_visit = visited.clone();
    do_not_visit.insert(me_value.to_string());
    do_not_visit.insert(elephant_value.to_string());
    if time_left <= 0 {
        0
    } else if me_state == 0 {
        let mut visited = visited.clone();
        visited.insert(me_value.to_string());
        if do_not_visit.len() >= target {
            return traverse_pair(
                me_value,
                elephant_value,
                -1,
                elephant_state,
                valve_map,
                visited,
                time_left,
                target,
            ); //disable me since nothing more can be done
        }
        let mut max = 0;
        let valve = valve_map.get(me_value).unwrap();
        let mut index = 0;
        while index < valve.accessible_nodes.len() {
            let neighbour_name = valve.accessible_nodes.get(index).unwrap();
            let neighbour_distance = valve.distances.get(index).unwrap();
            if !do_not_visit.contains(neighbour_name) {
                let result = traverse_pair(
                    neighbour_name,
                    elephant_value,
                    *neighbour_distance + 1, //once we arrive we turn on the power
                    elephant_state,
                    valve_map,
                    visited.clone(),
                    time_left,
                    target,
                );
                max = max.max(result);
            }
            index += 1
        }
        max
    } else if elephant_state == 0 {
        let mut visited = visited.clone();
        visited.insert(elephant_value.to_string());
        if do_not_visit.len() >= target {
            return traverse_pair(
                me_value, "", me_state, -1, valve_map, visited, time_left, target,
            ); //disable elephant since nothing more can be done
        }
        let mut max = 0;
        let valve = valve_map.get(elephant_value).unwrap();
        let mut index = 0;
        while index < valve.accessible_nodes.len() {
            let neighbour_name = valve.accessible_nodes.get(index).unwrap();
            let neighbour_distance = valve.distances.get(index).unwrap();
            if !do_not_visit.contains(neighbour_name) {
                let result = traverse_pair(
                    me_value,
                    neighbour_name,
                    me_state,
                    *neighbour_distance + 1, //once we arrive we turn on the power
                    valve_map,
                    visited.clone(),
                    time_left,
                    target,
                );
                max = max.max(result);
            }
            index += 1
        }
        max
    } else {
        visited
            .iter()
            .map(|key| valve_map.get(key).unwrap().flow_rate)
            .sum::<i32>()
            + traverse_pair(
                me_value,
                elephant_value,
                me_state - 1,
                elephant_state - 1,
                valve_map,
                visited,
                time_left - 1,
                target,
            )
    }
}

fn create_valves(filepath: &str) -> HashMap<String, Valve> {
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
                let vec = vec![other_name.to_string()];
                valve_to_neighbour_map.insert(name.to_string(), vec);
            }
        }
        valve_map.insert(name.to_string(), valve);
    }
    for (valve_name, neighbour_names) in valve_to_neighbour_map {
        let valve = valve_map.get_mut(valve_name.as_str()).unwrap();
        for neighbour_name in neighbour_names {
            valve.neighbours.push(neighbour_name.to_string());
            valve.accessible_nodes.push(neighbour_name);
            valve.distances.push(1)
        }
    }
    let valve_names: Vec<String> = valve_map
        .values()
        .map(|valve| valve.name.to_string())
        .collect();
    for valve_name in valve_names {
        let mut index = 0;
        while index
            < valve_map
                .get(valve_name.as_str())
                .unwrap()
                .accessible_nodes
                .len()
        {
            let other_valve_name = valve_map
                .get(valve_name.as_str())
                .unwrap()
                .accessible_nodes
                .get(index)
                .unwrap()
                .to_string();
            let distance = *valve_map
                .get(valve_name.as_str())
                .unwrap()
                .distances
                .get(index)
                .unwrap();
            let other_valve = valve_map.get(other_valve_name.as_str()).unwrap();
            let other_valve_distances = other_valve.distances.clone();
            let other_valve_neighbours = other_valve.accessible_nodes.clone();
            let mut other_index = 0;
            while other_index < other_valve_distances.len() {
                let other_valve_distance = *other_valve_distances.get(other_index).unwrap();
                let other_valve_neighbour =
                    other_valve_neighbours.get(other_index).unwrap().to_string();
                if valve_name != other_valve_neighbour
                    && other_valve_distance == 1
                    && !valve_map
                        .get(valve_name.as_str())
                        .unwrap()
                        .accessible_nodes
                        .contains(&other_valve_neighbour)
                {
                    let valve = valve_map.get_mut(valve_name.as_str()).unwrap();
                    valve.distances.push(distance + 1);
                    valve
                        .accessible_nodes
                        .push(other_valve_neighbour.to_string());
                }
                other_index += 1;
            }
            index += 1;
        }
    }
    //optimization to remove visiting of flowless nodes
    let useless_valves: Vec<String> = valve_map
        .values()
        .filter(|valve| valve.flow_rate == 0)
        .map(|valve| valve.name.to_string())
        .collect();
    for valve in valve_map.values_mut() {
        for useless in &useless_valves {
            let pos = valve.accessible_nodes.iter().position(|x| x == useless);
            if let Some(pos) = pos {
                valve.accessible_nodes.remove(pos);
                valve.distances.remove(pos);
            }
        }
    }
    valve_map
}

#[derive(Debug)]
struct Valve {
    name: String,
    flow_rate: i32,
    neighbours: Vec<String>,
    accessible_nodes: Vec<String>,
    distances: Vec<i32>,
}

impl Valve {
    fn new(name: String, flow_rate: i32) -> Self {
        Valve {
            name,
            flow_rate,
            neighbours: Vec::new(),
            accessible_nodes: Vec::new(),
            distances: Vec::new(),
        }
    }
}

impl PartialEq for Valve {
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
