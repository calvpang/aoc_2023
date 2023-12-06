use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;

// Read input.txt and return it as a vector of strings
fn read_input() -> Vec<String> {
    let mut input = Vec::new();
    let file = File::open("src/input.txt").expect("File not found");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        input.push(line.unwrap());
    }
    input
}

#[derive(Clone, Debug)]
struct Race {
    time: i32,
    distance: i32
}

// Parsing Input into Vector of Race
fn get_races(input: Vec<String>) -> Vec<Race>{
    let mut races = Vec::new();

    // Extracting Digits from String
    let re = Regex::new(r"\d+").unwrap();

    // Extracting the Lines Containing the Times and Distances
    let times = re.find_iter(&input[0]).map(|mat| mat.as_str().parse().unwrap()).collect::<Vec<i32>>();
    let distances = re.find_iter(&input[1]).map(|mat| mat.as_str().parse().unwrap()).collect::<Vec<i32>>();

    for i in 0..times.len() {
        let race = Race {
            time: times[i],
            distance: distances[i]
        };
        races.push(race)
    }
    races
}

// Calculating possible Win Conditions
fn possible_wins(race: Race) -> i32{
    let time = race.time;
    let distance = race.distance;

    let mut wins = Vec::new();

    for i in 0..time {
        let distance_travelled = i * (&time-i);
        
        // Debugging!
        // println!("Time: {}, Distance: {}", i, distance_travelled);
        
        if distance_travelled > distance {
            wins.push(true);
        }
    }
    println!("Wins: {:?}", wins.len());
    wins.len() as i32
}

// Finding all possible win
fn calculate_result(races: Vec<Race>) -> i32 {
    let mut result = 1;

    for race in races {
        result *= possible_wins(race);
    }
    result
} 

fn main() {
    println!("Reading File...");
    let input = read_input();

    println!("\nExtracting Races...");
    let races = get_races(input);

    println!("\nCalculating Result...");
    let result = calculate_result(races);
    println!("Result: {}", result);
}
