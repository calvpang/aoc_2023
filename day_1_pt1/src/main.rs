use std::fs::File;
use std::io::{BufRead, BufReader};

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

// Removing all non-numeric characters each line
fn numbers_only(input: Vec<String>) -> Vec<String> {
    let mut output = Vec::new();
    for line in input {
        let mut cleaned_line = String::new();
        for c in line.chars() {
            if c.is_numeric() {
                cleaned_line.push(c);
            }
        }
        output.push(cleaned_line.parse::<String>().unwrap());
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

// Print each item in the vector of integers
fn print_input(input: Vec<i32>) {
    for i in input {
        println!("{}", i);
    }
}

fn main() {
    println!("Reading input...");
    let input = read_input();

    println!("\nCleaning input of non-alphanumeric...");
    let numbers_vec = numbers_only(input);

    println!("\nKeeping first and last...");
    let first_last_vec = keep_first_and_last(numbers_vec);

    println!("\nPrinting input...");
    print_input(first_last_vec.clone());

    println!("\nSumming input...");
    let sum = sum_input(first_last_vec);
    println!("Sum: {}", sum);

}