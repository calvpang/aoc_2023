use std::fs::File;
use std::io::{BufRead, BufReader};

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

// Extracting Numbers from Input for each Line
fn parse_input(input: Vec<String>) -> Vec<Vec<i64>> {
    let mut numbers = Vec::new();

    for line in input {
        let line_numbers: Vec<i64> = line.split(' ')
            .map(|num| num.parse::<i64>().unwrap())
            .collect();
        numbers.push(line_numbers);
    }
    numbers
}

// Iteratively calculate deltas between numbers until there is a delta vector of 0's
fn calculate_deltas(mut numbers: Vec<i64>) -> Vec<Vec<i64>> {
    let mut all_deltas = Vec::new();

    loop {
        let mut deltas = Vec::new();

        // Calculating the deltas between numbers
        for window in numbers.windows(2) {
            let delta = window[1] - window[0];
            deltas.push(delta);
        }

        all_deltas.push(deltas.clone());

        // Checking if the Deltas Vector is all 0's
        if deltas.iter().all(|&x| x == 0) {
            break;
        }

        // Overwriting the Numbers Vectors with the Deltas Vector to continue the loop
        numbers = deltas;
    }

    all_deltas
}

// Extrapolate the Numbers
fn extrapolate_numbers(numbers: Vec<i64>) -> i64{
    // Calculating the Deltas
    let mut deltas = calculate_deltas(numbers.clone());

    // Reversing the vector of Delta Vectors
    deltas.reverse();

    // Handling the Case where there is not a 0's delta vector in deltas
    if deltas[0].is_empty() {
        deltas[0].push(0);
    }

    // Pushing the Original Numbers to the end of the Vector
    deltas.push(numbers);

    // Debugging!
    // println!("{:?}", deltas);

    // Extrapolating the Numbers
    let mut extrapolated_numbers = Vec::new();
    let mut delta_value = 0;
    for mut delta in deltas {
        delta_value += delta[delta.len() -1];
        delta.push(delta_value);
        extrapolated_numbers.push(delta_value);

        // Debugging!
        // println!("{:?}", delta);
    }

    // Returning the Extrapolated Number for the Original Input
    extrapolated_numbers[extrapolated_numbers.len() - 1]
}

fn main() {
    println!("Reading Input...");
    let input = read_input("src/input.txt".to_string());

    println!("\nParsing Input...");
    let numbers = parse_input(input);

    println!("\nExtrapolating the Numbers...");
    let mut summing_vector = Vec::new();
    for numbers in numbers {
        let extrapolated_numbers = extrapolate_numbers(numbers);
        summing_vector.push(extrapolated_numbers);

        // // Debugging!
        // println!("{:?}", extrapolated_numbers);
    }
    
    // Summing the Extrapolated Numbers
    println!("\nSumming the Extrapolated Numbers...");
    let sum: i64 = summing_vector.iter().sum();
    println!("Sum: {}", sum);
}

// // Attempts:
// // 1: 1897034315 (Too High)
// // 2: 1713265569 (Too Low)
// // 3: 1884768153 (Correct)