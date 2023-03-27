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

use device_query::{DeviceQuery, DeviceState, Keycode};
use rand::seq::SliceRandom;
use unicode_segmentation::UnicodeSegmentation;

fn remove_last_line() {
    // Move the cursor up to remove the user newline
    print!("{}[1A", 27 as char);
    // Remove the line
    print!("{}[2K", 27 as char);
}

fn replace_with_underlines(input: String, percentage: u8) -> String {
    let mut rng = rand::thread_rng();
    let num_replacements = input.chars().count() * (percentage as usize) / 100;

    let mut replacement_indices: Vec<usize> = (0..input.chars().count()).collect();
    replacement_indices.shuffle(&mut rng);
    let replacement_indices = &replacement_indices[0..num_replacements];

    let mut output = String::new();
    let mut index = 0;

    for grapheme in UnicodeSegmentation::graphemes(input.as_str(), true) {
        if replacement_indices.contains(&index) && grapheme.chars().all(char::is_alphabetic) {
            output.push('_');
        } else {
            output.push_str(grapheme);
        }
        index += 1;
    }

    output
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

    // Welcome message
    println!("Welcome to the poem learning program!");
    println!("This program will help you learn your favourite poems by heart.");
    println!("First, please read the poem below at least 3 times.");

    // Print the poem
    println!("{}", poem);
    println!("");

    // Wait for the user to press enter 3 times. Remove the user input after each press

    for i in 0..3 {
        // Wait for the user to release enter
        while device_state.get_keys().contains(&Keycode::Enter) {
            // Do nothing
        }
        println!(
            "Press enter to confirm that you have read the poem. ({}/3)",
            i + 1
        );
        // let mut input = String::new();
        // io::stdin().read_line(&mut input).unwrap();

        // remove_input();

        // This code is replaced with the device_query implementation below

        // Wait for the user to press enter
        while !device_state.get_keys().contains(&Keycode::Enter) {
            // Do nothing
        }
        // Clear the user input
        remove_last_line();
    }

    print!("\x1B[2J\x1B[1;1H");
    println!("Great job! Now let's start learning the poem by heart.");
    println!("This process will take some time so don't worry if you don't get it right away.");
    println!("Drink some water and let's get started!");
    println!("Press enter to continue.");

    // Wait for the user to release enter
    while device_state.get_keys().contains(&Keycode::Enter) {
        // Do nothing
    }

    // Wait for the user to press enter
    while !device_state.get_keys().contains(&Keycode::Enter) {
        // Do nothing
    }

    // Clear the terminal
    print!("\x1B[2J\x1B[1;1H");

    // Split the poem into pieces
    let poem_pieces: Vec<&str> = poem.split("\r\n\r\n").collect();

    // Seperate the poem into pieces
    for i in 0..poem_pieces.len() {
        // Wait for the user to release enter
        while device_state.get_keys().contains(&Keycode::Enter) {
            // Do nothing
        }

        // Show the whole poem
        if i != 0 {
            println!("Great work!");
        }
        println!(
            "Here is piece number {}/{} of the poem:\n",
            i + 1,
            poem_pieces.len()
        );
        println!("{}", poem_pieces[i]);
        println!("");
        println!("Press enter to continue when ready.");

        // Wait for the user to press enter
        while !device_state.get_keys().contains(&Keycode::Enter) {
            // Do nothing
        }
        // Clear the terminal
        print!("\x1B[2J\x1B[1;1H");

        // Replace 1% of random characters from the poem with an underline, incrementing by 1% each time the user presses enter
        let poem_piece = poem_pieces[i].to_string();

        // Replace 1% more of the characters each time
        for replace_percentage in 0..25 {
            let replace_percentage = (replace_percentage + 1) * 4;
            // Wait for the user to release enter
            while device_state.get_keys().contains(&Keycode::Enter) {
                // Do nothing
            }

            // Replace random characters with a underline. Make sure that the character is not a newline, space or underscore or a comma. If so, skip it and try again. Keep in mind the replace_percentage
            let modified_poem_piece =
                replace_with_underlines(poem_piece.clone(), replace_percentage);

            'choice: loop {
                // Show the poem
                println!(
                    "Here is piece number {}/{} of the poem with {}% of the characters hidden:",
                    i + 1,
                    poem_pieces.len(),
                    replace_percentage
                );
                println!(
                    "{}\n\nPress enter to continue when ready. If you are unsure, press \"a\".",
                    modified_poem_piece
                );

                // Wait for the user to press enter or a
                while !device_state.get_keys().contains(&Keycode::Enter)
                    && !device_state.get_keys().contains(&Keycode::A)
                {
                    // Do nothing
                }

                // Check if the user pressed enter or a
                if device_state.get_keys().contains(&Keycode::Enter) {
                    // Clear the terminal
                    print!("\x1B[2J\x1B[1;1H");
                    break 'choice;
                } else if device_state.get_keys().contains(&Keycode::A) {
                    // Clear the terminal
                    print!("\x1B[2J\x1B[1;1H");
                    println!("Here is the answer:");
                    println!("{}\n\nUnpress \"a\" to continue", poem_piece);
                    // Wait for the user to unpress "a"
                    while device_state.get_keys().contains(&Keycode::A) {
                        // Do nothing
                    }
                    // Clear the terminal
                    print!("\x1B[2J\x1B[1;1H");
                }

                // Clear the terminal
                print!("\x1B[2J\x1B[1;1H");
            }
        }
    }
}
