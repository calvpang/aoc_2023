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

// Hand struct to store the cards and the bid
struct Hand {
    card_1: usize,
    card_2: usize,
    card_3: usize,
    card_4: usize,
    card_5: usize,
    bid: usize,
    strength: usize
}

fn convert_card_to_number(c: char) -> usize {
    match c {
      'A' => 14,
      'K' => 13,
      'Q' => 12,
      'J' => 11,
      'T' => 10,
      '9' => 9,
      '8' => 8,
      '7' => 7,
      '6' => 6,
      '5' => 5,
      '4' => 4,
      '3' => 3,
      '2' => 2,
      _ => unreachable!()
    }
  }

// Parse the input into a vector of Hands
fn parse_input(input: Vec<String>) -> Vec<Hand>{
    let mut hands = Vec::new();

    for line in input {
        let split_line: Vec<_> = line.split_whitespace().collect();

        let strength = calculate_strength(split_line[0].to_string());

        let hand = Hand {
            card_1: convert_card_to_number(split_line[0].chars().nth(0).unwrap()),
            card_2: convert_card_to_number(split_line[0].chars().nth(1).unwrap()),
            card_3: convert_card_to_number(split_line[0].chars().nth(2).unwrap()),
            card_4: convert_card_to_number(split_line[0].chars().nth(3).unwrap()),
            card_5: convert_card_to_number(split_line[0].chars().nth(4).unwrap()),
            bid: split_line[1].parse::<usize>().unwrap(),
            strength: strength
        };
        // Debugging!
        println!("Hand: {}", split_line[0]);
        println!("Strength: {}", strength);
        println!("Bid: {}", split_line[1]);
        println!();
        hands.push(hand);
    }
    hands
}

// Find Strength of a Hand
fn calculate_strength(input: String) -> usize {
    let mut card_counts = std::collections::HashMap::new();
    let mut strength= 0;

    for card in input.chars() {
        let count = card_counts.entry(card).or_insert(0);
        *count += 1;
    }

    // Check for Five of a Kind
    if card_counts.values().any(|&count| count == 5) {
        strength = 7;
    }
    
    // Check for Four of a Kind
    else if card_counts.values().any(|&count| count == 4) {
        strength = 6;
    }
    
    // Check for Full House
    else if card_counts.values().any(|&count| count == 3) && card_counts.values().any(|&count| count == 2) {
        strength = 5;
    }

    // Check for Three of a Kind where the remaining cards are exactly 1
    else if card_counts.values().any(|&count| count == 3) && card_counts.values().filter(|&count| *count == 1).count() == 2 {
        strength = 4;
    }

    // Check for Two Pair
    else if card_counts.values().filter(|&count| *count == 2).count() == 2 {
        strength = 3;
    }

    // Check for One Pair 
    else if card_counts.values().filter(|&count| *count == 2).count() == 1 {
        strength = 2;
    }

    // Check for High Card
    else {
        strength = 1;
    }
    strength
}

// Sort Hands by Strength and Card Order
fn sort_hands(hands: Vec<Hand>) -> Vec<Hand> {
    let mut sorted_hands = hands;
    sorted_hands.sort_by(|a, b| {
        let strength_cmp = b.strength.cmp(&a.strength);
        if strength_cmp != std::cmp::Ordering::Equal {
            return strength_cmp;
        }

        let card_1_cmp = b.card_1.cmp(&a.card_1);
        if card_1_cmp != std::cmp::Ordering::Equal {
            return card_1_cmp;
        }

        let card_2_cmp = b.card_2.cmp(&a.card_2);
        if card_2_cmp != std::cmp::Ordering::Equal {
            return card_2_cmp;
        }

        let card_3_cmp = b.card_3.cmp(&a.card_3);
        if card_3_cmp != std::cmp::Ordering::Equal {
            return card_3_cmp;
        }

        let card_4_cmp = b.card_4.cmp(&a.card_4);
        if card_4_cmp != std::cmp::Ordering::Equal {
            return card_4_cmp;
        }

        let card_5_cmp = b.card_5.cmp(&a.card_5);
        if card_5_cmp != std::cmp::Ordering::Equal {
            return card_5_cmp;
        }

        std::cmp::Ordering::Equal
    });

    // Reverse the order of the hands
    sorted_hands.reverse();
    sorted_hands
}

fn calculate_winnings(hands: Vec<Hand>) -> i32 {
    let mut rank = 1;
    let mut winnings = 0;
    for hand in hands {
        winnings += hand.bid as i32 * rank;

        // Debugging!
        println!("Hand: {}{}{}{}{}", hand.card_1, hand.card_2, hand.card_3, hand.card_4, hand.card_5);
        println!("Strength: {}", hand.strength);
        println!("Bid: {}", hand.bid);
        println!("Rank: {}", rank);
        println!();

        rank += 1;
    }
    winnings
}

fn main() {
    println!("Reading Input...");
    let input = read_input();

    println!("\nParsing Input...");
    let hands = parse_input(input);

    println!("\nSorting Hands...");
    let sorted_hands = sort_hands(hands);

    println!("\nCalculating Winnings...");
    let winnings = calculate_winnings(sorted_hands);
    println!("Winnings: {}", winnings);
    }
