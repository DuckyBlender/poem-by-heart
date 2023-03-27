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
use std::io::stdout;
use unicode_segmentation::UnicodeSegmentation;

use crossterm::{style::Stylize, terminal, ExecutableCommand};

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
            println!("{}", "Could not read the poem!".red());
            println!("Please make sure that poem.txt is in the same folder as the executable.");
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
            // Wait for the user to press enter
            while !device_state.get_keys().contains(&Keycode::Enter) {
                // Do nothing
            }
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

    'program: loop {
        // Wait for the user to press enter 3 times. Remove the user input after each press
        for i in 0..3 {
            // Wait for the user to release enter
            while device_state.get_keys().contains(&Keycode::Enter) {
                // Do nothing
            }
            let msg = format!(
                "Press enter to confirm that you have read the poem. ({}/3)",
                i + 1
            );
            println!("{}", msg.green());

            // Wait for the user to press enter
            while !device_state.get_keys().contains(&Keycode::Enter) {
                // Do nothing
            }
            // Clear the user input
            remove_last_line();
        }

        stdout()
            .execute(terminal::Clear(terminal::ClearType::All))
            .unwrap();

        println!(
            "{}",
            "Great job! Now let's start learning the poem by heart.\n
This process will take some time so don't worry if you don't get it right away.\n
Drink some water and let's get started!\n
Press enter to continue.\n"
                .green()
        );

        // Wait for the user to release enter
        while device_state.get_keys().contains(&Keycode::Enter) {
            // Do nothing
        }

        // Wait for the user to press enter
        while !device_state.get_keys().contains(&Keycode::Enter) {
            // Do nothing
        }

        // Clear the terminal
        stdout()
            .execute(terminal::Clear(terminal::ClearType::All))
            .unwrap();

        // Split the poem into pieces
        let poem_pieces: Vec<&str> = poem.split("\r\n\r\n").collect();
        if poem_pieces.len() == 1 {
            println!("{}", "===\nWARNING! The poem is not split into pieces. This program will work best if the poem is split into pieces by double newlines.\n===\n".red());
            println!("Press enter to continue...");
            // Wait for the user to press enter
            while !device_state.get_keys().contains(&Keycode::Enter) {
                // Do nothing
            }
            return;
        }

        // Seperate the poem into pieces
        for i in 0..poem_pieces.len() {
            // Wait for the user to release enter

            // Show the whole poem
            if i != 0 {
                println!("{}", "Great work!".green());
            }
            let msg = format!(
                "Here is piece number {}/{} of the poem:\n",
                i + 1,
                poem_pieces.len()
            );
            println!("{}", msg.green());

            println!("{}", poem_pieces[i]);

            println!("{}", "\nPress enter to continue when ready.".green());

            while device_state.get_keys().contains(&Keycode::Enter) {
                // Do nothing
            }

            // Wait for the user to press enter
            while !device_state.get_keys().contains(&Keycode::Enter) {
                // Do nothing
            }
            // Clear the terminal
            stdout()
                .execute(terminal::Clear(terminal::ClearType::All))
                .unwrap();

            // Replace 1% of random characters from the poem with an underline, incrementing by 1% each time the user presses enter
            let poem_piece = poem_pieces[i].to_string();

            // Replace 1% more of the characters each time
            for replace_percentage in 0..25 {
                let replace_percentage = (replace_percentage + 1) * 4;

                // Replace random characters with a underline. Make sure that the character is not a newline, space or underscore or a comma. If so, skip it and try again. Keep in mind the replace_percentage
                let modified_poem_piece =
                    replace_with_underlines(poem_piece.clone(), replace_percentage);

                'choice: loop {
                    // Show the poem
                    let msg = format!(
                        "Here is piece number {}/{} of the poem with {}% of the characters hidden:",
                        i + 1,
                        poem_pieces.len(),
                        replace_percentage
                    );

                    println!("{}", msg.green());

                    println!("\n{}\n\n", modified_poem_piece);

                    println!(
                        "{}",
                        "Press enter to continue when ready. If you are unsure, press \"a\"."
                            .green()
                    );

                    // Wait for the user to release enter
                    while device_state.get_keys().contains(&Keycode::Enter) {
                        // Do nothing
                    }

                    // Wait for the user to press enter or a
                    while !device_state.get_keys().contains(&Keycode::Enter)
                        && !device_state.get_keys().contains(&Keycode::A)
                    {
                        // Do nothing
                    }

                    // Check if the user pressed enter or a
                    if device_state.get_keys().contains(&Keycode::Enter) {
                        // Clear the terminal
                        stdout()
                            .execute(terminal::Clear(terminal::ClearType::All))
                            .unwrap();
                        break 'choice;
                    } else if device_state.get_keys().contains(&Keycode::A) {
                        // Clear the terminal
                        stdout()
                            .execute(terminal::Clear(terminal::ClearType::All))
                            .unwrap();
                        println!("{}", "Here is the answer:\n".green());
                        println!("{}\n", poem_piece);
                        println!("{}", "Unpress \"a\" to continue.".green());
                        // Wait for the user to unpress "a"
                        while device_state.get_keys().contains(&Keycode::A) {
                            // Do nothing
                        }
                        // Clear the terminal
                        stdout()
                            .execute(terminal::Clear(terminal::ClearType::All))
                            .unwrap();
                    }

                    // Clear the terminal
                    stdout()
                        .execute(terminal::Clear(terminal::ClearType::All))
                        .unwrap();
                }
            }
        }
        // Make the same thing for all of the pieces
        println!(
            "{}",
            "Great job! To finish off, let's see if you can remember the whole poem!".green()
        );
        println!("{}", "Press enter to continue.".green());

        // Wait for the user to release enter
        while device_state.get_keys().contains(&Keycode::Enter) {
            // Do nothing
        }

        // Wait for the user to press enter
        while !device_state.get_keys().contains(&Keycode::Enter) {
            // Do nothing
        }

        // Clear the terminal
        stdout()
            .execute(terminal::Clear(terminal::ClearType::All))
            .unwrap();

        // Replace 1% of random characters from the poem with an underline, incrementing by 1% each time the user presses enter
        let poem = poem.to_string();

        // Replace 1% more of the characters each time
        for replace_percentage in 0..25 {
            let replace_percentage = (replace_percentage + 1) * 4;

            // Replace random characters with a underline. Make sure that the character is not a newline, space or underscore or a comma. If so, skip it and try again. Keep in mind the replace_percentage
            let modified_poem = replace_with_underlines(poem.clone(), replace_percentage);

            'choice: loop {
                // Show the poem
                let msg = format!(
                    "Here is the entire poem with {}% of the characters hidden:",
                    replace_percentage
                );

                println!("{}", msg.green());

                println!("\n{}\n\n", modified_poem);

                println!(
                    "{}",
                    "Press enter to continue when ready. If you are unsure, press \"a\".".green()
                );

                // Wait for the user to release enter
                while device_state.get_keys().contains(&Keycode::Enter) {
                    // Do nothing
                }

                // Wait for the user to press enter or a
                while !device_state.get_keys().contains(&Keycode::Enter)
                    && !device_state.get_keys().contains(&Keycode::A)
                {
                    // Do nothing
                }

                // Check if the user pressed enter or a
                if device_state.get_keys().contains(&Keycode::Enter) {
                    // Clear the terminal
                    stdout()
                        .execute(terminal::Clear(terminal::ClearType::All))
                        .unwrap();
                    break 'choice;
                } else if device_state.get_keys().contains(&Keycode::A) {
                    // Clear the terminal
                    stdout()
                        .execute(terminal::Clear(terminal::ClearType::All))
                        .unwrap();
                    println!("{}", "Here is the answer:\n".green());
                    println!("{}\n", poem);
                    println!("{}", "Unpress \"a\" to continue.".green());
                    // Wait for the user to unpress "a"
                    while device_state.get_keys().contains(&Keycode::A) {
                        // Do nothing
                    }
                    // Clear the terminal
                    stdout()
                        .execute(terminal::Clear(terminal::ClearType::All))
                        .unwrap();
                }

                // Clear the terminal
                stdout()
                    .execute(terminal::Clear(terminal::ClearType::All))
                    .unwrap();
            }
        }

        // Clear the terminal
        stdout()
            .execute(terminal::Clear(terminal::ClearType::All))
            .unwrap();

        println!(
            "{}",
            "Great job! You have memorized the entire poem!".green()
        );
        println!(
            "{}",
            "Press enter to restart the program. Press ESC to stop.".green()
        );

        // Wait for the user to release enter
        while device_state.get_keys().contains(&Keycode::Enter) {
            // Do nothing
        }

        while !device_state.get_keys().contains(&Keycode::Enter)
            && !device_state.get_keys().contains(&Keycode::Escape)
        {
            // Do nothing
        }
        if device_state.get_keys().contains(&Keycode::Escape) {
            break 'program;
        }
    }
}
