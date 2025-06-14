use std::io::{self, Write}; // For standard input/output and flushing

mod models;
mod utils;
mod game;

use utils::{load_words_from_json, get_word_list_choice, select_random_word};
use game::play_hangman_round;


// --- MAIN PROGRAM FLOW ---
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let all_words = match load_words_from_json() {
        Ok(words) => words,
        Err(e) => {
            eprintln!("Failed to start game due to data loading error: {}", e);
            return Err(e); // Exit program with error
        }
    };

    loop {
        let selected_list_option = get_word_list_choice(&all_words);

        let selected_word_list = match selected_list_option {
            Some(list) => list,
            None => {
                // User chose to quit
                println!("Exiting Hangman game. Goodbye!");
                break; // Exit the main game loop
            }
        };

        if selected_word_list.is_empty() {
            println!("The selected word list is empty. Please check your JSON file.");
            continue; // Ask for choice again
        }

        let secret_word = select_random_word(selected_word_list);

        // Play the actual game round
        let _player_won = play_hangman_round(secret_word); // We don't strictly need to use `_player_won` here

        // Ask if user wants to play another round
        println!("\nPlay another round? (yes/no)");
        print!("> "); // Simple prompt for consistency
        io::stdout().flush()?;

        let mut play_again_input = String::new();
        io::stdin().read_line(&mut play_again_input)?;
        if !play_again_input.trim().eq_ignore_ascii_case("yes") {
            println!("Thanks for playing!");
            break; // Exit the main game loop
        }
    }

    Ok(())
}