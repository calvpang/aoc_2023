use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;
use std::collections::HashMap;


// Read input.txt and return it as a vector of strings
fn read_input() -> Vec<String> {
    let mut input = Vec::new();
    let file = File::open("src/sample_input.txt").expect("File not found");
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

// Build Seed to Soil Map as a HashMap
fn build_seed_to_soil_map(seed_to_soil: Vec<Vec<i64>>) -> HashMap<i64, i64> {
    let mut seed_to_soil_map = HashMap::new();

    for vector in seed_to_soil {
        for i in 0..vector[2] {
            seed_to_soil_map.insert(vector[1] + i, vector[0] + i);
        }
    }
    // Debugging!
    // println!("Seed to Soil Map: {:?}", seed_to_soil_map);
    seed_to_soil_map
}

// Build Soil to Fertiliser Map as a HashMap
fn build_soil_to_fertiliser_map(soil_to_fertiliser: Vec<Vec<i64>>) -> HashMap<i64, i64> {
    let mut soil_to_fertiliser_map = HashMap::new();

    for vector in soil_to_fertiliser {
        for i in 0..vector[2] {
            soil_to_fertiliser_map.insert(vector[1] + i, vector[0] + i);
        }
    }
    // Debugging!
    // println!("Soil to Fertiliser Map: {:?}", soil_to_fertiliser_map);
    soil_to_fertiliser_map
}

// Build Fertiliser to Water Map as a HashMap
fn build_fertiliser_to_water_map(fertiliser_to_water: Vec<Vec<i64>>) -> HashMap<i64, i64> {
    let mut fertiliser_to_water_map = HashMap::new();

    for vector in fertiliser_to_water {
        for i in 0..vector[2] {
            fertiliser_to_water_map.insert(vector[1] + i, vector[0] + i);
        }
    }
    // Debugging!
    // println!("Fertiliser to Water Map: {:?}", fertiliser_to_water_map);
    fertiliser_to_water_map
}

// Build Water to Light Map as a HashMap
fn build_water_to_light_map(water_to_light: Vec<Vec<i64>>) -> HashMap<i64, i64> {
    let mut water_to_light_map = HashMap::new();

    for vector in water_to_light {
        for i in 0..vector[2] {
            water_to_light_map.insert(vector[1] + i, vector[0] + i);
        }
    }
    // Debugging!
    // println!("Water to Light Map: {:?}", water_to_light_map);
    water_to_light_map
}

// Build Light to Temperature Map as a HashMap
fn build_light_to_temperature_map(light_to_temperature: Vec<Vec<i64>>) -> HashMap<i64, i64> {
    let mut light_to_temperature_map = HashMap::new();

    for vector in light_to_temperature {
        for i in 0..vector[2] {
            light_to_temperature_map.insert(vector[1] + i, vector[0] + i);
        }
    }
    // Debugging!
    // println!("Light to Temperature Map: {:?}", light_to_temperature_map);
    light_to_temperature_map
}

// Build Temperature to Humidity Map as a HashMap
fn build_temperature_to_humidity_map(temperature_to_humidity: Vec<Vec<i64>>) -> HashMap<i64, i64> {
    let mut temperature_to_humidity_map = HashMap::new();

    for vector in temperature_to_humidity {
        for i in 0..vector[2] {
            temperature_to_humidity_map.insert(vector[1] + i, vector[0] + i);
        }
    }
    // Debugging!
    // println!("Temperature to Humidity Map: {:?}", temperature_to_humidity_map);
    temperature_to_humidity_map
}

// Build Humidity to Location Map as a HashMap
fn build_humidity_to_location_map(humidity_to_location: Vec<Vec<i64>>) -> HashMap<i64, i64> {
    let mut humidity_to_location_map = HashMap::new();

    for vector in humidity_to_location {
        for i in 0..vector[2] {
            humidity_to_location_map.insert(vector[1] + i, vector[0] + i);
        }
    }
    // Debugging!
    // println!("Humidity to Location Map: {:?}", humidity_to_location_map);
    humidity_to_location_map
}

// For every Seed in the Seeds vector, try to find the corresponding Location
// If a value is not in the subsequent map, then the input and output are the same
fn calculate_seed_to_location(seed: i64, seed_to_soil_map: HashMap<i64, i64>, soil_to_fertiliser_map: HashMap<i64, i64>, fertiliser_to_water_map: HashMap<i64, i64>, water_to_light_map: HashMap<i64, i64>, light_to_temperature_map: HashMap<i64, i64>, temperature_to_humidity_map: HashMap<i64, i64>, humidity_to_location_map: HashMap<i64, i64>) {
    // Find the Soil Value for a Particular Seed, if the HashMap doesn't contain the Seed, return the Soil Value
    let soil = match seed_to_soil_map.get(&seed) {
        Some(&soil) => soil,
        None => seed,
    };

    // Find the Fertiliser Value for a Particular Soil, if the HashMap doesn't contain the Soil, return the Fertiliser Value
    let fertiliser = match soil_to_fertiliser_map.get(&soil) {
        Some(&fertiliser) => fertiliser,
        None => soil,
    };

    // Find the Water Value for a Particular Fertiliser, if the HashMap doesn't contain the Fertiliser, return the Water Value
    let water = match fertiliser_to_water_map.get(&fertiliser) {
        Some(&water) => water,
        None => fertiliser,
    };

    // Find the Light Value for a Particular Water, if the HashMap doesn't contain the Water, return the Light Value
    let light = match water_to_light_map.get(&water) {
        Some(&light) => light,
        None => water,
    };

    // Find the Temperature Value for a Particular Light, if the HashMap doesn't contain the Light, return the Temperature Value
    let temperature = match light_to_temperature_map.get(&light) {
        Some(&temperature) => temperature,
        None => light,
    };

    // Find the Humidity Value for a Particular Temperature, if the HashMap doesn't contain the Temperature, return the Humidity Value
    let humidity = match temperature_to_humidity_map.get(&temperature) {
        Some(&humidity) => humidity,
        None => temperature,
    };

    // Find the Location Value for a Particular Humidity, if the HashMap doesn't contain the Humidity, return the Location Value
    let location = match humidity_to_location_map.get(&humidity) {
        Some(&location) => location,
        None => humidity,
    };

    // Debugging!
    println!("Seed: {}, Soil: {}, Fertiliser: {}, Water: {}, Light: {}, Temperature: {}, Humidity: {}, Location: {}", seed, soil, fertiliser, water, light, temperature, humidity, location);
    }

fn main() {
    println!("Reading Input...");
    let input = read_input();

    println!("\nParsing Input...");
    let positions = find_start_end(input.clone());
    let clean_input = parse_input(input, positions);

    println!("\nBuilding HashMaps...");
    println!("\nCalculating Seed to Soil...");
    let seed_to_soil_map = build_seed_to_soil_map(clean_input.seed_to_soil);
    println!("\nCalculating Soil to Fertiliser...");
    let soil_to_fertiliser_map = build_soil_to_fertiliser_map(clean_input.soil_to_fertiliser);
    println!("\nCalculating Fertiliser to Water...");
    let fertiliser_to_water_map = build_fertiliser_to_water_map(clean_input.fertiliser_to_water);
    println!("\nCalculating Water to Light...");
    let water_to_light_map = build_water_to_light_map(clean_input.water_to_light);
    println!("\nCalculating Light to Temperature...");
    let light_to_temperature_map = build_light_to_temperature_map(clean_input.light_to_temperature);
    println!("\nCalculating Temperature to Humidity...");
    let temperature_to_humidity_map = build_temperature_to_humidity_map(clean_input.temperature_to_humidity);
    println!("\nCalculating Humidity to Location...");
    let humidity_to_location_map = build_humidity_to_location_map(clean_input.humidity_to_location);

    println!("\nCalculating Seed to Location...");
    for seed in clean_input.seeds {
        calculate_seed_to_location(seed, seed_to_soil_map.clone(), soil_to_fertiliser_map.clone(), fertiliser_to_water_map.clone(), water_to_light_map.clone(), light_to_temperature_map.clone(), temperature_to_humidity_map.clone(), humidity_to_location_map.clone());
    }

}
