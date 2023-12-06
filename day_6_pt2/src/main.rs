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
    time: i64,
    distance: i64
}

// Parsing Input into Vector of Race
fn get_races(input: Vec<String>) -> Race{
    // Extracting Digits from String
    let re = Regex::new(r"\d+").unwrap();

    // Extracting the Lines Containing the Times and Distances
    let time = re.find_iter(&input[0]).map(|mat| mat.as_str()).collect::<Vec<_>>().join("").parse::<i64>().unwrap();
    let distance = re.find_iter(&input[1]).map(|mat| mat.as_str()).collect::<Vec<_>>().join("").parse::<i64>().unwrap();

    // Debugging!
    println!("Times: {}, Distance: {}", time, distance);

    let race = Race {
        time: time,
        distance: distance
    };
    race
}

// Calculating possible Win Conditions
fn possible_wins(race: Race) -> i64{
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
    // Debugging!
    // println!("Wins: {:?}", wins.len());
    wins.len() as i64
}

fn main() {
    println!("Reading File...");
    let input = read_input();

    println!("\nExtracting Race Info...");
    let race = get_races(input);

    println!("\nDetermining Possible Wins...");
    let wins = possible_wins(race);
    println!("Possible ways to beat the record: {}", wins)

}
