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

// Check if the number of cubes is less than the max number of cubes
fn check_cubes(input: Vec<Vec<Vec<i32>>>) {
    let red_max: i32 = 12;
    let green_max: i32 = 13;
    let blue_max: i32 = 14;
    let mut game_id_sum = 0;

    for game in input {
        // Debugging!
        // println!("Game ID: {}", game[0][0]);
        // println!("Red: \n");
        // print_items(game[1].clone());
        // println!("Green: \n");
        // print_items(game[2].clone());
        // println!("Blue: \n");
        // print_items(game[3].clone());
        // println!("");

        // Create a Boolean for each color
        let mut red_bool = true;
        let mut green_bool = true;
        let mut blue_bool = true;

        // Check that values inside red_count are less than red_max
        for red in game[1].clone() {
            if red > red_max {
                red_bool = false;
            }
        }

        // Check that values inside green_count are less than green_max
        for green in game[2].clone() {
            if green > green_max {
                green_bool = false;
            }
        }

        // Check that values inside blue_count are less than blue_max
        for blue in game[3].clone() {
            if blue > blue_max {
                blue_bool = false;
            }
        }

        // If all are true, add the game_id to the game_id_sum
        if red_bool && green_bool && blue_bool {
            game_id_sum += game[0][0];
        }
    }
    println!("Game ID Sum: {}", game_id_sum);

}

fn main() {
    println!("Reading input.txt...");
    let input = read_input();

    println!("\nGetting Game ID...");
    let game_info = extract_game_info(input);

    println!("\nChecking Cube Limit...");
    check_cubes(game_info);
}
