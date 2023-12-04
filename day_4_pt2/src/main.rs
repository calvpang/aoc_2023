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

// Extract the Card ID, Winning Numbers and Numbers from the Puzzle Input
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

// Helper function to calculate the number of scratch cards won by a particular scratch card
fn find_number_of_cards_won(card: &Card) -> usize {
    // Convert the winning_numbers and numbers vectors to HashSets
    let winning_numbers: HashSet<_> = card.winning_numbers.iter().collect();
    let numbers: HashSet<_> = card.numbers.iter().collect();

    // Find the intersection of the two HashSets
    let common_numbers: HashSet<_> = winning_numbers.intersection(&numbers).collect();

    // Count the number of common numbers
    common_numbers.len()
}

// Calculating the total number of scratch cards won by the "original" winning cards
fn total_number_of_cards(card_info: Vec<Card>) -> usize {
    let mut total_winning_cards = 0;

    // Iterate through each card in card_info
    for card in &card_info {
        let new_cards_won = _total_number_of_cards(&card, &card_info);
        total_winning_cards += new_cards_won;
    }
    total_winning_cards
}

// Helper function to calculate the total number of additional scratch cards won by a particular scratch card (including itself)
fn _total_number_of_cards(card: &Card, card_info: &[Card]) -> usize {
    let mut new_cards_won = Vec::new();

    // Add the initial card to the new_cards_won vector as it is also included in the count
    new_cards_won.push(card.clone());

    let mut i = 0;

    // Keep iterating through new_cards_won until no new cards are added
    while i < new_cards_won.len() {
        // Checking the current card in new_cards_won
        let current_card = new_cards_won[i].clone();

        // Find the number of cards won by the current card
        let number_of_cards_won = find_number_of_cards_won(&current_card);

        // For each card won by the current card
        for j in 1..=number_of_cards_won {
            // Calculate the new card to be added to new_cards_won by incrementing the current card id
            //e.g. if the current card id is 1, and 2 cards were won. Then add card id 2 and 3 respectively
            let new_card_id = current_card.id + j as i32;

            // Check that the new_card_id is not out of bounds
            if new_card_id <= card_info.len() as i32 {
                // Add the new card to new_cards_won
                new_cards_won.push(card_info[(new_card_id - 1) as usize].clone());
            }
        }
        i += 1;
    }

    // Debugging!
    println!("\nCard ID: {}", card.id);
    println!("Number of Cards Added: {}", new_cards_won.len());

    // Returning the number of cards won
    new_cards_won.len()
}

fn main() {
    println!("Reading input...");
    let input = read_input();

    println!("\nExtracting card info...");
    let card_info = extract_card_info(input);

    println!("\nCalculating Total Number of Cards...");
    let total_card_count = total_number_of_cards(card_info);
    println!("\nTotal Number of Cards Cards: {}", total_card_count);
}
