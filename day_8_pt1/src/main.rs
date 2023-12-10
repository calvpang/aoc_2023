use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;
use regex::Regex;

// Read input.txt and return it as a vector of strings
fn read_input(filename: String) -> Vec<String> {
    let mut input = Vec::new();
    let file = File::open(filename).expect("File not found");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        input.push(line.unwrap());
    }
    input
}

fn parse_input(input: Vec<String>) -> (Vec<String>, HashMap<String, (String, String)>){
    // Extract the Route
    let route: Vec<String> = input[0].chars().filter(|&c| !c.is_whitespace()).map(|c| c.to_string()).collect();

    // Extract the Nodes and put them in a HashMap
    let mut nodes: HashMap<String, (String, String)> = HashMap::new();

    for line in input[2..].iter() {
        let re = Regex::new(r"(\w+) = \((\w+), (\w+)\)").unwrap();
        let caps = re.captures(line).unwrap();

        let key = caps[1].to_string();
        let value = (caps[2].to_string(), caps[3].to_string());

        nodes.insert(key, value);
    }
    (route, nodes)
}

fn find_path(route: Vec<String>, nodes: HashMap<String, (String, String)>) {
    // Declaring Start and Destination Nodes
    let start_node = "AAA".to_string();
    let destination_node = "ZZZ".to_string();

    // Finding a Path
    let mut current_node = start_node;
    let mut count = 0;

    // Continue looping through route vector until we find the destination node
    loop {
        for direction in &route {
            // Extract the current_node from the HashMap
            let node = nodes.get(&current_node).unwrap();

            // Update current_node based on direction
            current_node = if direction == "L" {
                count += 1;
                node.0.clone()
            } else if direction == "R" {
                count += 1;
                node.1.clone()
            } else {
                panic!("Invalid direction: {}", direction);
            };

            if current_node == destination_node {
                println!("Found path from AAA to ZZZ!");
                println!("Path length: {}", count);
                return;
            }
        }
    }
}

fn main() {
    println!("Reading Input...");
    let input = read_input("src/input.txt".to_string());

    println!("\nParsing Input...");
    let (route, nodes) = parse_input(input);

    println!("\nFinding Path...");
    find_path(route, nodes);
}
