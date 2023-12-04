use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;
use std::collections::HashSet;

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

// Struct to hold Card ID, Winning Numbers and Numbers
#[derive(Clone)]
struct Card {
    id: i32,
    winning_numbers: Vec<i32>,
    numbers: Vec<i32>,
}

// Function to print a vector
fn print_items<T: std::fmt::Display>(input: Vec<T>) {
    for i in input {
        println!("{}", i);
    }
}

// Extract the Card ID, Winning Numbers and Numbers from the input
fn extract_card_info(input: Vec<String>) -> Vec<Card>{
    let mut card_info = Vec::new();

    // Regex to extract digits
    let re = Regex::new(r"\d+").unwrap();

    // Iterate through each line
    for line in input {
        // Extract the Card ID
        let substring_1 = line.split(":").collect::<Vec<&str>>()[0];
        let card_id = re.find_iter(substring_1).map(|mat| mat.as_str().parse().unwrap()).collect::<Vec<i32>>()[0];

        // Extract the Winning Numbers
        let substring_2 = line.split(":").collect::<Vec<&str>>()[1].split("|").collect::<Vec<&str>>()[0];
        let winning_numbers: Vec<i32> = re.find_iter(substring_2).map(|mat| mat.as_str().parse().unwrap()).collect::<Vec<i32>>();
        
        // Extract the Numbers
        let substring_3 = line.split(":").collect::<Vec<&str>>()[1].split("|").collect::<Vec<&str>>()[1];
        let numbers: Vec<i32> = re.find_iter(substring_3).map(|mat| mat.as_str().parse().unwrap()).collect::<Vec<i32>>();

        // // Debugging!
        // println!("Card ID: {}", card_id.clone());
        // println!("\nWinning Numbers: ");
        // print_items(winning_numbers.clone());
        // println!("\nNumbers: ");
        // print_items(numbers.clone());

        // Create a new Card
        let card = Card {
            id: card_id.clone(),
            winning_numbers: winning_numbers.clone(),
            numbers: numbers.clone(),
        };

        // Add the Card to the card_info vector
        card_info.push(card);
    }
    card_info
}

// Iterating through the card info and checking if the numbers match the winning numbers using a HashSet
fn check_winning_cards(card_info: Vec<Card>) {
    let mut score = 0;

    for card in card_info {
        // Convert the winning_numbers and numbers vectors to HashSets
        let winning_numbers: HashSet<_> = card.winning_numbers.iter().collect();
        let numbers: HashSet<_> = card.numbers.iter().collect();

        // Find the intersection of the two HashSets
        let common_numbers: HashSet<_> = winning_numbers.intersection(&numbers).collect();

        // Count the number of common numbers
        let common_numbers_count = common_numbers.len();

        // Calculate the score
        // 0 common numbers = 0 points
        // 1 common number = 1 point
        // 2 common numbers = 2 points
        // 3 common numbers = 4 points etc.
        let mut card_score = 0;
        if common_numbers_count == 0 {
            card_score = 0;
        }
        else if common_numbers_count == 1 {
            card_score = 1;
        }
        else {
            // 2^0 = 1
            card_score += 2_i32.pow((common_numbers_count-1) as u32);
        }
        
        // Debugging!
        println!("Card ID: {}", card.id);
        println!("Winning Numbers: {:?}", card.winning_numbers);
        println!("Numbers: {:?}", card.numbers);
        println!("Common Numbers: {:?}", common_numbers);
        println!("Common Numbers Count: {}", common_numbers_count);
        println!("Card Score: {}", card_score.clone());
        println!();

        // Add the card score to the total score
        score += card_score;
    }
    println!("Total Score: {}", score);
}


fn main() {
    println!("Reading input...");
    let input = read_input();

    println!("\nExtracting card info...");
    let card_info = extract_card_info(input);

    println!("\nChecking winning cards...");
    check_winning_cards(card_info);
}
