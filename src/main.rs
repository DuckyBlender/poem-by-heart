// Welcome to the program which you can learn your favourite poems by heart.
// This program has a few steps
// 1. Read the poem from a file
// 2. Make sure the user reads it at least 3 times (each one identified by pressing enter)
// 3. Seperate the poem by double newlines
// 4. Remove more and more random characters from the poem piece by piece
// When the user is ready, he can choose to only show the first letter of each word
// The user always can choose to see the whole poem again
// If the user is ready, he is asked to type the poem from memory
// If the user is 95% correct, he is congratulated
// If the user is less than 95% correct, he is asked to try again

// The poem is stored in a file called poem.txt

use std::env;
use std::fs;
use std::io;

use rand::Rng;

use device_query::{DeviceQuery, DeviceState, Keycode};

fn remove_input() {
    // Move the cursor up to remove the user newline
    print!("{}[1A", 27 as char);
    // Remove the line
    print!("{}[2K", 27 as char);
    // Move the cursor up to remove the prompt
    print!("{}[1A", 27 as char);
    // Remove the line
    print!("{}[2K", 27 as char);
}

// Replace a percentage of random characters with an underline
fn replace_with_newlines(input: String, percentage: u8) -> String {
    let number_of_characters = input.chars().count();
    let number_of_characters_to_replace =
        (number_of_characters as f32 * (percentage as f32 / 100.0)) as u32;

    let mut modified_input = input.clone();
    let mut rng = rand::thread_rng();
}

fn main() {
    // Set the backtrace environment variable to 1
    env::set_var("RUST_BACKTRACE", "1");

    let device_state = DeviceState::new(); // To see the user input without pressing enter

    // Get the poem
    let poem = match fs::read_to_string("poem.txt") {
        Ok(poem) => poem,
        Err(_) => {
            println!("Could not read the poem! Please make sure that poem.txt is in the same folder as the executable.");
            println!("Keep in mind that the poem structure should be as follows:");
            println!("Line 1");
            println!("Line 2");
            println!("");
            println!("Line 3");
            println!("Line 4");
            println!("");
            println!("Line 5");
            println!("Line 6");
            println!("");
            println!("It is important that there are two newlines between each poem piece.");
            return;
        }
    };
    let poem = poem.trim().to_string();

    // Print the poem
    println!("{}", poem);
    println!("");

    // Wait for the user to press enter 3 times. Remove the user input after each press

    for i in 0..3 {
        println!(
            "Press enter to confirm that you have read the poem. ({}/3)",
            i + 1
        );
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        remove_input();
    }

    println!("Great job! Now let's start learning the poem by heart.");
    println!("This process will take some time so don't worry if you don't get it right away.");
    println!("");

    // Split the poem into pieces
    //let poem_pieces: Vec<&str> = poem.split("\n\n").collect();
    let poem_pieces: Vec<&str> = poem.split("\r\n\r\n").collect();

    println!("The poem has {} pieces.", poem_pieces.len());

    // Seperate the poem into pieces
    for i in 0..poem_pieces.len() {
        // Show the whole poem
        println!("Here is the {} piece of the poem:", i + 1);
        println!("{}", poem_pieces[i]);
        println!("");
        println!("Press enter to continue when ready.");

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        remove_input();

        // Replace 1% of random characters from the poem with an underline, incrementing by 1% each time the user presses enter
        let poem_piece = poem_pieces[i].to_string();
        let mut modified_poem_piece = poem_piece.clone();

        // Replace 1% more of the characters each time
        for replace_percentage in 0..100 {
            // Replace random characters with a underline. Make sure that the character is not a newline, space or underscore or a comma. If so, skip it and try again. Keep in mind the replace_percentage
            modified_poem_piece = replace_with_newlines(modified_poem_piece, replace_percentage);

            // Show the poem
            println!(
                "Here is the {} piece of the poem with {}% of the characters hidden:",
                i + 1,
                replace_percentage + 1
            );
            println!("{}", modified_poem_piece);
            println!("");
            println!("Press enter to continue when ready.");

            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            remove_input();
        }
    }
}
