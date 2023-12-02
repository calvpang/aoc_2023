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

// Iterate through the input to extract the Game ID and the number of cubes for each color
fn extract_game_info(input: Vec<String>) -> Vec<Vec<Vec<i32>>>{
    // Create a vector to store extracted game info
    let mut game_info = Vec::new();

    // Iterate through each line
    for line in input {
        // Extract the Game ID
        let game_id = line.split(":").collect::<Vec<&str>>()[0].chars().skip(5).collect::<String>();

        // Extract the Games Rounds
        let games = line.split(":").collect::<Vec<&str>>()[1].split(";").collect::<Vec<&str>>();

        // let colors = vec!["red", "green", "blue"];
        // Create a counter for each color
        let mut red_count = Vec::new();
        let mut green_count = Vec::new();
        let mut blue_count = Vec::new();

        // Iterate through each game and count the number of colors
        for game in games {
            
            // Extract the colors in each game
            let colors_in_game = game.split(",").collect::<Vec<&str>>();
            
            for color in colors_in_game {
                // Extract the number of cubes for each color
                if color.contains("red") {
                    let cube_number = color.chars().filter(|c| c.is_digit(10)).collect::<String>();
                    red_count.push(cube_number.parse::<i32>().unwrap());
                } else if color.contains("green") {
                    let cube_number = color.chars().filter(|c| c.is_digit(10)).collect::<String>();
                    green_count.push(cube_number.parse::<i32>().unwrap());
                } else if color.contains("blue") {
                    let cube_number = color.chars().filter(|c| c.is_digit(10)).collect::<String>();
                    blue_count.push(cube_number.parse::<i32>().unwrap());
                }
            }
        }

        // Debugging!
        // println!("Game ID: {}", game_id.clone());
        // print_items(red_count.clone());
        // print_items(green_count.clone());
        // print_items(blue_count.clone());

        // Store in a vector
        let game_id_vec = vec![game_id.parse::<i32>().unwrap()];
        let round_info = vec![game_id_vec, red_count, green_count, blue_count];
        
        // Push to the game_info vector
        game_info.push(round_info);
    }
    game_info
}

// Function to print a vector
fn print_items<T: std::fmt::Display>(input: Vec<T>) {
    for i in input {
        println!("{}", i);
    }
}

// Determine the minimum number of cubes required for each color in a game
// Multiply those values for each game and sum them together for the answer
fn check_min_cubes_and_power(input: Vec<Vec<Vec<i32>>>) {
    let mut cube_power_sum = 0;

    for game in input {
        let mut red_max: i32 = 0;
        let mut green_max: i32 = 0;
        let mut blue_max: i32 = 0;
        let mut cube_power: i32 = 0;

        // Find the maximum value in the red_count vector
        red_max = *game[1].clone().iter().max().unwrap();

        // Find the maximum value in the green_count vector
        green_max = *game[2].clone().iter().max().unwrap();

        // Find the maximum value in the blue_count vector
        blue_max = *game[3].clone().iter().max().unwrap();

        // Debugging!
        // println!("Game ID: {}", game[0][0]);
        // println!("Red Max: {}", red_max);
        // println!("Green Max: {}", green_max);
        // println!("Blue Max: {}", blue_max);

        // Calculate the power of a set of cubes and sum it
        cube_power = red_max * green_max * blue_max;
        cube_power_sum += cube_power;
    }
    println!("Cube Power Sum: {}", cube_power_sum);
}

fn main() {
    println!("Reading input.txt...");
    let input = read_input();

    println!("\nGetting Game ID...");
    let game_info = extract_game_info(input);

    println!("\nChecking Cube Limit...");
    check_min_cubes_and_power(game_info);
}
