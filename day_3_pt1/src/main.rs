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

// Struct to hold Numbers and their positions in the grid
#[derive(Clone)]
struct Number {
    number: i32,
    row: i32,
    col_start: i32,
    col_end: i32,
}

// Implement PartialEq for Number struct to compare Numbers
impl PartialEq for Number {
    fn eq(&self, other: &Self) -> bool {
        self.number == other.number && self.row == other.row && self.col_start == other.col_start && self.col_end == other.col_end
    }
}

// Struct to hold Symbols and their positions in the grid
#[derive(Clone)]
struct Symbol {
    symbol: char,
    row: i32,
    col: i32,
}

// Finding the Numbers and their positions in the grid
fn find_numbers (input: Vec<String>) -> Vec<Number> {
    let mut numbers = Vec::new();
    let re = Regex::new(r"\d+").unwrap();

    for (i, line) in input.iter().enumerate() {
        for (_j, number) in re.find_iter(line).enumerate() {
            let num = Number {
                number: number.as_str().parse::<i32>().unwrap(),
                row: i as i32,
                col_start: number.start() as i32,
                col_end: number.end() as i32 - 1,
            };
            // // Debugging!
            // println!("Number: {}, Row: {}, Col Start: {}, Col End: {}", num.number, num.row, num.col_start, num.col_end);
            numbers.push(num);
        }
    }
    numbers

}

// Finding the Symbols and their positions in the grid
fn find_symbols (input: Vec<String>) -> Vec<Symbol>{
    let mut symbols = Vec::new();
    let re = Regex::new(r"[^\d\.]+").unwrap();

    for (i, line) in input.iter().enumerate() {
        for (_j, symbol) in re.find_iter(line).enumerate() {
            let sym = Symbol {
                symbol: symbol.as_str().chars().next().unwrap(),
                row: i as i32,
                col: symbol.start() as i32,
            };
            // // Debugging!
            // println!("Symbol: {}, Row: {}, Col: {}", sym.symbol, sym.row, sym.col);
            symbols.push(sym);
        }
    }
    symbols
}

// Finding the adjacent Numbers and Symbols
fn find_adjacent(numbers: Vec<Number>, symbols: Vec<Symbol>) -> Vec<(Number, Symbol)> {
    let mut adjacent = Vec::new();

    for num in &numbers {
        for sym in &symbols {
            for col in num.col_start..=num.col_end {
                if (num.row - sym.row).abs() <= 1 && (col - sym.col).abs() <= 1 {
                    adjacent.push((num.clone(), sym.clone()));
                    break;
                }
            }
        }
    }
    adjacent
}

// Function to find unique Number structs in adjacent
fn find_unique(adjacent: Vec<(Number, Symbol)>) -> Vec<Number> {
    let mut unique = Vec::new();
    let mut seen = Vec::new();

    for (num, _sym) in adjacent {
        if !seen.contains(&num) {
            unique.push(num.clone());
            seen.push(num.clone());
        }
    }
    unique
}

// Sum the number for each Number struct in unique
fn sum_unique(unique: Vec<Number>) -> i32 {
    let mut sum = 0;
    for num in &unique {
        sum += num.number;
    }
    sum
}

fn main() {
    println!("Reading input...");
    let input = read_input();

    println!("\nFinding numbers...");
    let numbers = find_numbers(input.clone());

    println!("\nFinding symbols...");
    let symbols = find_symbols(input.clone());

    println!("\nFinding adjacent numbers...");
    let adjacent = find_adjacent(numbers.clone(), symbols.clone());

    // // Debugging!
    // for (num, sym) in &adjacent {
    //     println!("Number {} at (Row: {}, Start Col: {}, End Col: {}) is adjacent to symbol {} at (Row: {}, Column: {})", num.number, num.row, num.col_start, num.col_end, sym.symbol, sym.row, sym.col);
    // }

    println!("\nFinding unique numbers...");
    let unique = find_unique(adjacent.clone());

    // // Debugging!
    // for num in &unique {
    //     println!("Unique Number {} at (Row: {}, Start Col: {}, End Col: {})", num.number, num.row, num.col_start, num.col_end);
    // }

    println!("\nSumming unique numbers...");
    let sum = sum_unique(unique.clone());
    println!("\nSum of unique numbers: {}", sum);
}