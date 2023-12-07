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

// Struct to store the information about a Hand
struct Hand {
    cards: Vec<usize>,
    bid: usize,
    strength: usize
}

// Converting Cards to Values
fn convert_card_to_number(c: char, p2: bool) -> usize {
    match c {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => if p2 { 0 } else { 11 },
        'T' => 10,
        '2'..='9' => c.to_digit(10).unwrap() as usize,
        _ => unreachable!()
    }
}

// Parse the input into a vector of Hand structs
fn parse_input(input: Vec<String>) -> Vec<Hand>{
    let mut hands = Vec::new();

    for line in input {
        let split_line: Vec<_> = line.split_whitespace().collect();

        let strength = calculate_strength(split_line[0].to_string());

        let hand = Hand {
            cards: split_line[0].chars().map(|c| convert_card_to_number(c, true)).collect(),
            bid: split_line[1].parse::<usize>().unwrap(),
            strength: strength
        };
        // // Debugging!
        // println!("Cards: {:?}", hand.cards);
        // println!("Bid: {}", hand.bid);
        // println!("Strength: {}", hand.strength);
        hands.push(hand);
    }
    hands
}

// Determining Strength of a Hand
fn calculate_strength(input: String) -> usize {
    let mut card_counts = std::collections::HashMap::new();
    let mut strength = 0;

    // Counting the number of each card in the Hand
    for card in input.chars() {
        let count = card_counts.entry(card).or_insert(0);
        *count += 1;
    }

    // Finding Joker's and adding their count to the highest card in hand
    if card_counts.contains_key(&'J') {
        let joker_count = *card_counts.get(&'J').unwrap();

        // Checking that there are less than 5 Jokers
        // Five Joker's would be a Five of a Kind with Jokers (Strength 7)
        if joker_count < 5 {
            // Finding the card with the highest count that is not the Joker
            let highest_card = *card_counts.iter()
                .filter(|(&card, _)| card != 'J')
                .max_by_key(|(_, &val)| val)
                .unwrap()
                .0;

            // Removing the Joker from the HashMap and adding its count to the highest card
            card_counts.remove(&'J');
            *card_counts.entry(highest_card).or_insert(0) += joker_count;
        } 
    }

    // Check for Five of a Kind
    if card_counts.values().any(|&count| count == 5) {
        strength += 7;
    }
    
    // Check for Four of a Kind
    else if card_counts.values().any(|&count| count == 4) {
        strength += 6;
    }
    
    // Check for Full House
    else if card_counts.values().any(|&count| count == 3) && card_counts.values().any(|&count| count == 2) {
        strength += 5;
    }

    // Check for Three of a Kind where the remaining cards are exactly 1
    else if card_counts.values().any(|&count| count == 3) && card_counts.values().filter(|&count| *count == 1).count() == 2 {
        strength += 4;
    }

    // Check for Two Pair
    else if card_counts.values().filter(|&count| *count == 2).count() == 2 {
        strength += 3;
    }

    // Check for One Pair 
    else if card_counts.values().filter(|&count| *count == 2).count() == 1 {
        strength += 2;
    }

    // Check for High Card
    else {
        strength += 1;
    }
    strength
}

// Sort Hands by Strength and Card Order
fn sort_hands(hands: Vec<Hand>) -> Vec<Hand> {
    let mut sorted_hands = hands;
    sorted_hands.sort_by(|a, b| {
        // Sort by Strength where the Highest Strength is First
        let strength_cmp = b.strength.cmp(&a.strength);
        if strength_cmp != std::cmp::Ordering::Equal {
            return strength_cmp;
        }

        // If the Strength Fields are Equal, compare the Cards One by One
        // Sort by First Card
        let card_1_cmp = b.cards[0].cmp(&a.cards[0]);
        if card_1_cmp != std::cmp::Ordering::Equal {
            return card_1_cmp;
        }

        // Sort by Second Card
        let card_2_cmp = b.cards[1].cmp(&a.cards[1]);
        if card_2_cmp != std::cmp::Ordering::Equal {
            return card_2_cmp;
        }

        // Sort by Third Card
        let card_3_cmp = b.cards[2].cmp(&a.cards[2]);
        if card_3_cmp != std::cmp::Ordering::Equal {
            return card_3_cmp;
        }
        
        // Sort by Fourth Card
        let card_4_cmp = b.cards[3].cmp(&a.cards[3]);
        if card_4_cmp != std::cmp::Ordering::Equal {
            return card_4_cmp;
        }

        // Sort by Fifth Card
        let card_5_cmp = b.cards[4].cmp(&a.cards[4]);
        if card_5_cmp != std::cmp::Ordering::Equal {
            return card_5_cmp;
        }

        // If all else fails, return Equal
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
        println!("Hand: {}{}{}{}{}", hand.cards[0], hand.cards[1], hand.cards[2], hand.cards[3], hand.cards[4]);
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
    let input = read_input("src/input.txt".to_string());

    println!("\nParsing Input...");
    let hands = parse_input(input);

    println!("\nSorting Hands...");
    let sorted_hands = sort_hands(hands);

    println!("\nCalculating Winnings...");
    let winnings = calculate_winnings(sorted_hands);
    println!("Winnings: {}", winnings);
    }
