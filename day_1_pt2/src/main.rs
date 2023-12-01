use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;
use regex::Regex;
use itertools::Itertools;

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

// Generate permutations of potential overlaps
fn find_overlaps() {
    let words = vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let mut overlaps: HashSet<String> = HashSet::new();

    let permutations = words.iter().permutations(words.len());

    for perm in permutations {
        for i in 0..perm.len() - 1 {
            let current_word = perm[i];
            let next_word = perm[i + 1];

            if current_word.chars().last() == next_word.chars().next() {
                let overlap = format!("{}{}", current_word, &next_word[1..]);
                overlaps.insert(overlap);
            }
        }
    }

    for overlap in overlaps {
        println!("{}", overlap);
}
}

// Iterate over each line and use regex to get digits and words
fn convert_to_numbers(input: Vec<String>) -> Vec<String>{
    let re = Regex::new(r"(\d|nineight|eightwo|twone|oneight|eighthree|sevenine|fiveight|threeight|one|two|three|four|five|six|seven|eight|nine)").unwrap();
    let mut output = Vec::new();

    for line in input {
        let mut matched_line = Vec::new();

        for cap in re.captures_iter(&line) {
            // if cap is a number, push it to the output vector
            if cap[0].parse::<i32>().is_ok() {
                matched_line.push(cap[0].parse::<String>().unwrap());
            }
            // if cap is a word, convert it to a number
            else {
                match &cap[0] {
                    "one" => matched_line.push("1".to_string()),
                    "two" => matched_line.push("2".to_string()),
                    "three" => matched_line.push("3".to_string()),
                    "four" => matched_line.push("4".to_string()),
                    "five" => matched_line.push("5".to_string()),
                    "six" => matched_line.push("6".to_string()),
                    "seven" => matched_line.push("7".to_string()),
                    "eight" => matched_line.push("8".to_string()),
                    "nine" => matched_line.push("9".to_string()),
                    "nineight" => matched_line.extend(["9", "8"].iter().cloned().map(String::from)),
                    "eightwo" => matched_line.extend(["8", "2"].iter().cloned().map(String::from)),
                    "twone" => matched_line.extend(["2", "1"].iter().cloned().map(String::from)),
                    "oneight" => matched_line.extend(["1", "8"].iter().cloned().map(String::from)),
                    "eighthree" => matched_line.extend(["8", "3"].iter().cloned().map(String::from)),
                    "sevenine" => matched_line.extend(["7", "9"].iter().cloned().map(String::from)),
                    "fiveight" => matched_line.extend(["5", "8"].iter().cloned().map(String::from)),
                    "threeight" => matched_line.extend(["3", "8"].iter().cloned().map(String::from)),
                    _ => println!("Error: {}", &cap[0]),
                }
            }
        }
        // Push the matched line to the output vector
        output.push(matched_line.join(""));
    }
    output
}

fn keep_first_and_last(input: Vec<String>) -> Vec<i32> {
    let mut output: Vec<i32> = Vec::new();
    for line in input {
        // Create a new string to hold the cleaned line
        let mut cleaned_line = String::new();

        // If the line is 1 character long, push it twice as an integer
        if line.len() == 1 {
            cleaned_line.push(line.chars().nth(0).unwrap());
            cleaned_line.push(line.chars().nth(0).unwrap());
        }
        // if the line is 2+ characters long, push the first and last characters as an integer
        else {
            cleaned_line.push(line.chars().nth(0).unwrap());
            cleaned_line.push(line.chars().nth(line.len() - 1).unwrap());
        }

        output.push(cleaned_line.parse::<i32>().unwrap());
    }
    output
}

// Iterate over vector of integers and sum the items
fn sum_input(input: Vec<i32>) -> i32 {
    let mut sum = 0;
    for i in input {
        sum += i;
    }
    sum
}

// Function to print a vector
fn print_items<T: std::fmt::Display>(input: Vec<T>) {
    for i in input {
        println!("{}", i);
    }
}

fn main() {
    println!("\nReading input...");
    let input = read_input();

    println!("\nFinding overlaps...");
    find_overlaps();

    println!("\nMatching and converting to numbers...");
    let numbers_vec = convert_to_numbers(input);

    // println!("\nPrinting...");
    // print_items(numbers_vec.clone());

    println!("\nKeeping first and last...");
    let first_and_last_vec = keep_first_and_last(numbers_vec);

    // println!("\nPrinting...");
    // print_items(first_and_last_vec.clone());

    println!("\nSumming...");
    let output = sum_input(first_and_last_vec);
    println!("Sum: {}", output);
}









