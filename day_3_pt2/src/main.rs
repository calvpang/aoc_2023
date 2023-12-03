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

// Implement PartialEq for Symbol struct to compare Symbols
impl PartialEq for Symbol {
    fn eq(&self, other: &Self) -> bool {
        self.symbol == other.symbol && self.row == other.row && self.col == other.col
    }
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

// Finding the Gears and their positions in the grid
fn find_gears (input: Vec<String>) -> Vec<Symbol>{
    let mut symbols = Vec::new();
    let re = Regex::new(r"\*").unwrap();

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

// Finding Numbers which are adjacent to Gears
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

// Finding Number Pairs which are adjacent to the same Gear
fn find_gear_pairs(adjacent: Vec<(Number, Symbol)>) -> Vec<Vec<Number>> {
    let mut pairs = Vec::new();

    for (num1, sym1) in &adjacent {
        for (num2, sym2) in &adjacent {

            // If the numbers are the same whilst looping, skip
            if num1 == num2 {
                continue;
            }

            // If numbers are next to the same gear, add the Number Pair to pairs
            if sym1 == sym2 {
                // Ordering the pairs so that the smaller number is always first
                // This makes it easier to remove duplicates later
                let (smaller_num, larger_num) = if num1.number < num2.number {
                    (num1.clone(), num2.clone())
                } else {
                    (num2.clone(), num1.clone())
                };

                // Pushing the ordered Number Pair to pairs
                let temp_vec = vec![smaller_num, larger_num];
                pairs.push(temp_vec);
            }
            }
    }

    // // Debugging!
    // let mut count = 0;
    // for pair in &pairs.clone() {
    //     println!("Gear Ratio: {} * {} = {}", pair[0].number, pair[1].number, pair[0].number * pair[1].number);
    //     count += 1;
    // }
    // println!("Number of gear pairs: {}", count);

    pairs
}

// Removing duplicate Number Pairs from pairs
fn remove_duplicates(pairs: Vec<Vec<Number>>) -> Vec<Vec<Number>> {
    let mut unique_pairs = Vec::new();
    let mut seen = Vec::new();

    for pair in pairs {
        if !seen.contains(&pair) {
            unique_pairs.push(pair.clone());
            seen.push(pair.clone());
        }
    }

    // // Debugging!
    // let mut count = 0;
    // for pair in &unique.clone() {
    //     println!("Gear Ratio: {} * {} = {}", pair[0].number, pair[1].number, pair[0].number * pair[1].number);   
    //     count += 1;
    // }
    // println!("Number of gear pairs: {}", count);
    unique_pairs
}

// Summing the unique Number Pairs
fn sum_gear_ratio(unique_pairs: Vec<Vec<Number>>) -> i32 {
    let mut sum = 0;

    for pair in unique_pairs {
        sum += pair[0].number * pair[1].number;
    }

    sum
}

fn main() {
    println!("Reading input...");
    let input = read_input();

    println!("\nFinding numbers...");
    let numbers = find_numbers(input.clone());

    println!("\nFinding symbols...");
    let symbols = find_gears(input.clone());

    println!("\nFinding adjacent numbers...");
    let adjacent = find_adjacent(numbers.clone(), symbols.clone());

    // // Debugging!
    // for (num, sym) in &adjacent {
    //     println!("Number {} at (Row: {}, Start Col: {}, End Col: {}) is adjacent to symbol {} at (Row: {}, Column: {})", num.number, num.row, num.col_start, num.col_end, sym.symbol, sym.row, sym.col);
    // }

    println!("\nFinding gear pairs...");
    let gear_pairs = find_gear_pairs(adjacent.clone());
    
    // // Debugging!
    // // for num in &unique {
    // //     println!("Gear Ratios {}", num);
    // // }
    
    println!("\nRemoving duplicates...");
    let unique = remove_duplicates(gear_pairs.clone());

    println!("\nCalculating result...");
    let sum = sum_gear_ratio(unique.clone());
    println!("\nSum of Gear Ratios: {}", sum);
}