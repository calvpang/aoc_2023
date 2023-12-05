use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;
// use std::collections::HashMap;


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

struct SectionPositions {
    seed: (usize, usize),
    seed_to_soil: (usize, usize),
    soil_to_fertiliser: (usize, usize),
    fertiliser_to_water: (usize, usize),
    water_to_light: (usize, usize),
    light_to_temperature: (usize, usize),
    temperature_to_humidity: (usize, usize),
    humidity_to_location: (usize, usize),
}

fn find_start_end(input: Vec<String>) -> SectionPositions {
    // Finding the Start of Each Section
    let seed_start = input.iter().position(|line| line.contains("seeds: ")).unwrap();
    let seed_to_soil_start = input.iter().position(|line| line.contains("seed-to-soil map:")).unwrap() + 1;
    let soil_to_fertiliser_start = input.iter().position(|line| line.contains("soil-to-fertilizer map:")).unwrap() + 1;
    let fertiliser_to_water_start = input.iter().position(|line| line.contains("fertilizer-to-water map:")).unwrap() + 1;
    let water_to_light_start = input.iter().position(|line| line.contains("water-to-light map:")).unwrap() + 1;
    let light_to_temperature_start = input.iter().position(|line| line.contains("light-to-temperature map:")).unwrap() + 1;
    let temperature_to_humidity_start = input.iter().position(|line| line.contains("temperature-to-humidity map:")).unwrap() + 1;
    let humidity_to_location_start = input.iter().position(|line| line.contains("humidity-to-location map:")).unwrap() + 1;

    // Finding the End of Each Section
    let seed_end = seed_start;
    let seed_to_soil_end = soil_to_fertiliser_start - 2;
    let soil_to_fertiliser_end = fertiliser_to_water_start - 2;
    let fertiliser_to_water_end = water_to_light_start - 2;
    let water_to_light_end = light_to_temperature_start - 2;
    let light_to_temperature_end = temperature_to_humidity_start - 2;
    let temperature_to_humidity_end = humidity_to_location_start - 2;
    let humidity_to_location_end = input.len();

    SectionPositions {
        seed: (seed_start, seed_end),
        seed_to_soil: (seed_to_soil_start, seed_to_soil_end),
        soil_to_fertiliser: (soil_to_fertiliser_start, soil_to_fertiliser_end),
        fertiliser_to_water: (fertiliser_to_water_start, fertiliser_to_water_end),
        water_to_light: (water_to_light_start, water_to_light_end),
        light_to_temperature: (light_to_temperature_start, light_to_temperature_end),
        temperature_to_humidity: (temperature_to_humidity_start, temperature_to_humidity_end),
        humidity_to_location: (humidity_to_location_start, humidity_to_location_end),
    }
}

struct InputData {
    seeds: Vec<i64>,
    seed_to_soil: Vec<Vec<i64>>,
    soil_to_fertiliser: Vec<Vec<i64>>,
    fertiliser_to_water: Vec<Vec<i64>>,
    water_to_light: Vec<Vec<i64>>,
    light_to_temperature: Vec<Vec<i64>>,
    temperature_to_humidity: Vec<Vec<i64>>,
    humidity_to_location: Vec<Vec<i64>>
}

// Parse the Input into the InputData struct
fn parse_input(input: Vec<String>, positions: SectionPositions) -> InputData {
    let mut seed_to_soil: Vec<Vec<i64>> = Vec::new();
    let mut soil_to_fertiliser: Vec<Vec<i64>> = Vec::new();
    let mut fertiliser_to_water: Vec<Vec<i64>> = Vec::new();
    let mut water_to_light: Vec<Vec<i64>> = Vec::new();
    let mut light_to_temperature: Vec<Vec<i64>> = Vec::new();
    let mut temperature_to_humidity: Vec<Vec<i64>> = Vec::new();
    let mut humidity_to_location: Vec<Vec<i64>> = Vec::new();

    // Regexes
    let re = Regex::new(r"\d+").unwrap();

    // Extracting the Seeds
    let seeds = re.find_iter(&input[positions.seed.0]).map(|mat| mat.as_str().parse().unwrap()).collect::<Vec<i64>>();
    
    // Extracting the Seed to Soil Map
    let seed_to_soil_lines = &input[positions.seed_to_soil.0..positions.seed_to_soil.1];
    for line in seed_to_soil_lines {
        let numbers: Vec<i64> = re.find_iter(line).map(|mat| mat.as_str().parse().unwrap()).collect::<Vec<i64>>();
        seed_to_soil.push(numbers);
    }

    // Extracting the Soil to Fertiliser Map
    let soil_to_fertiliser_lines = &input[positions.soil_to_fertiliser.0..positions.soil_to_fertiliser.1];
    for line in soil_to_fertiliser_lines {
        let numbers: Vec<i64> = re.find_iter(line).map(|mat| mat.as_str().parse().unwrap()).collect::<Vec<i64>>();
        soil_to_fertiliser.push(numbers);
    }

    // Extracting the Fertiliser to Water Map
    let fertiliser_to_water_lines = &input[positions.fertiliser_to_water.0..positions.fertiliser_to_water.1];
    for line in fertiliser_to_water_lines {
        let numbers: Vec<i64> = re.find_iter(line).map(|mat| mat.as_str().parse().unwrap()).collect::<Vec<i64>>();
        fertiliser_to_water.push(numbers);
    }

    // Extracting the Water to Light Map
    let water_to_light_lines = &input[positions.water_to_light.0..positions.water_to_light.1];
    for line in water_to_light_lines {
        let numbers: Vec<i64> = re.find_iter(line).map(|mat| mat.as_str().parse().unwrap()).collect::<Vec<i64>>();
        water_to_light.push(numbers);
    }

    // Extracting the Light to Temperature Map
    let light_to_temperature_lines = &input[positions.light_to_temperature.0..positions.light_to_temperature.1];
    for line in light_to_temperature_lines {
        let numbers: Vec<i64> = re.find_iter(line).map(|mat| mat.as_str().parse().unwrap()).collect::<Vec<i64>>();
        light_to_temperature.push(numbers);
    }

    // Extracting the Temperature to Humidity Map
    let temperature_to_humidity_lines = &input[positions.temperature_to_humidity.0..positions.temperature_to_humidity.1];
    for line in temperature_to_humidity_lines {
        let numbers: Vec<i64> = re.find_iter(line).map(|mat| mat.as_str().parse().unwrap()).collect::<Vec<i64>>();
        temperature_to_humidity.push(numbers);
    }

    // Extracting the Humidity to Location Map
    let humidity_to_location_lines = &input[positions.humidity_to_location.0..positions.humidity_to_location.1];
    for line in humidity_to_location_lines {
        let numbers: Vec<i64> = re.find_iter(line).map(|mat| mat.as_str().parse().unwrap()).collect::<Vec<i64>>();
        humidity_to_location.push(numbers);
    }

    // // Debugging!
    // println!("Seeds: {:?}", seeds);
    // println!("Seed to Soil Map: {:?}", seed_to_soil);
    // println!("Soil to Fertiliser Map: {:?}", soil_to_fertiliser);
    // println!("Fertiliser to Water Map: {:?}", fertiliser_to_water);
    // println!("Water to Light Map: {:?}", water_to_light);
    // println!("Light to Temperature Map: {:?}", light_to_temperature);
    // println!("Temperature to Humidity Map: {:?}", temperature_to_humidity);
    // println!("Humidity to Location Map: {:?}", humidity_to_location);

    // Storing as InputData struct
    let input_data = InputData {
        seeds: seeds,
        seed_to_soil: seed_to_soil,
        soil_to_fertiliser: soil_to_fertiliser,
        fertiliser_to_water: fertiliser_to_water,
        water_to_light: water_to_light,
        light_to_temperature: light_to_temperature,
        temperature_to_humidity: temperature_to_humidity,
        humidity_to_location: humidity_to_location,
    };
    input_data
}

// Check Mappings
fn check_mappings(key: i64, map: Vec<Vec<i64>>) -> i64 {
    let mut found = false;
    let mut value = 0;

    for mapping in map {
        // If the Source exists in the map, calculate it
        if key >= mapping[1] && key <= mapping[1] + mapping[2] {
            let value_increment = key - mapping[1];
            value = mapping[0] + value_increment;
            // println!("Key: {}, Value: {}", key, value);
            found = true;
        }
    }
    // If the Source doesn't exist in the map, return the Source
    if !found {
        // println!("Key: {}, Value: {}", key, key);
        value = key;
    }
    value
}

// Find the Location for a Particular Seed in InputData
fn find_locations(input_data: InputData) -> Vec<i64> {
    let mut locations = Vec::new();

    for seed in input_data.seeds {
        let soil = check_mappings(seed, input_data.seed_to_soil.clone());
        let fertiliser = check_mappings(soil, input_data.soil_to_fertiliser.clone());
        let water = check_mappings(fertiliser, input_data.fertiliser_to_water.clone());
        let light = check_mappings(water, input_data.water_to_light.clone());
        let temperature = check_mappings(light, input_data.light_to_temperature.clone());
        let humidity = check_mappings(temperature, input_data.temperature_to_humidity.clone());
        let location = check_mappings(humidity, input_data.humidity_to_location.clone());
        locations.push(location);
    }
    // Debugging!
    println!("Locations: {:?}", locations);
    locations
}

// Find the Smallest Location
fn get_smallest_location(mut locations: Vec<i64>) -> i64 {
    locations.sort();
    let smallest_location = locations[0];
    println!("Smallest Location: {}", smallest_location);
    smallest_location
}

fn main() {
    println!("Reading Input...");
    let input = read_input();

    println!("\nParsing Input...");
    let positions = find_start_end(input.clone());
    let clean_input = parse_input(input, positions);

    println!("\nFinding Locations from Seed...");
    let locations = find_locations(clean_input);

    println!("\nFinding Smallest Location from Seeds...");
    let _smallest_location = get_smallest_location(locations);
}
