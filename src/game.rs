use std::io::{self, Write}; // Import io for user input and output
use crate::utils::MAX_WRONG_GUESSES; // Import constant for max wrong guesses

// --- FUNCTION 4: display_game_state (Helper for play_hangman_round) ---
// Displays the current state of the game to the user.
pub fn display_game_state(
    hidden_display_chars: &[char], // Using slice for efficiency
    guessed_letters: &[char],     // Using slice for efficiency
    remaining_guesses: u8,
) {
    println!("\nWord: {}", hidden_display_chars.iter().collect::<String>());
    println!(
        "Guessed Letters: {}",
        guessed_letters
            .iter()
            .map(|&c| c.to_string())
            .collect::<Vec<String>>()
            .join(", ")
    );
    println!("Guesses Left: {}", remaining_guesses);
}

// --- FUNCTION 5: play_hangman_round ---
// Contains the core game logic and user interaction for one round.
// Returns BOOLEAN: TRUE if player wins, FALSE if player loses.
pub fn play_hangman_round(secret_word_str: &str) -> bool {
    // Convert secret word to uppercase characters for case-insensitive comparison
    let secret_word_chars: Vec<char> = secret_word_str.to_uppercase().chars().collect();
    let word_length = secret_word_chars.len();

    // Initialize the hidden word display with underscores
    let mut hidden_word_display: Vec<char> = vec!['_'; word_length];

    let mut guessed_letters: Vec<char> = Vec::new(); // Stores unique guessed letters
    let mut wrong_guesses_count: u8 = 0;

    println!("\n--- Hangman Round Started! ---");
    println!("Your word has {} letters.", word_length);

    loop {
        // Display current game state
        display_game_state(
            &hidden_word_display,
            &guessed_letters,
            MAX_WRONG_GUESSES - wrong_guesses_count,
        );

        // Check for game over (loss)
        if wrong_guesses_count >= MAX_WRONG_GUESSES {
            println!("\n--- GAME OVER! ---");
            println!("You ran out of guesses. The word was: {}", secret_word_str);
            return false; // Player lost
        }

        // Check for win condition
        if hidden_word_display.iter().all(|&c| c != '_') {
            println!("\n--- CONGRATULATIONS! ---");
            println!("You guessed the word: {}", secret_word_str);
            return true; // Player won
        }

        // Prompt for guess
        print!("Guess a letter: ");
        io::stdout().flush().expect("Failed to flush stdout"); // Ensure prompt appears

        let mut guess_input = String::new();
        io::stdin()
            .read_line(&mut guess_input)
            .expect("Failed to read line");
        let guess_input = guess_input.trim(); // Remove newline and whitespace

        // Validate guess input
        if guess_input.len() != 1 {
            println!("Invalid input. Please enter exactly one letter.");
            continue; // Ask again
        }

        let guessed_char = guess_input
            .chars()
            .next()
            .unwrap()
            .to_ascii_uppercase(); // Get char and convert to uppercase

        if !guessed_char.is_ascii_alphabetic() {
            println!("Invalid input. Please enter an alphabetic character.");
            continue;
        }

        // Check if letter was already guessed
        if guessed_letters.contains(&guessed_char) {
            println!("You already guessed '{}'. Try a new letter.", guessed_char);
            continue;
        }

        // Add the new guess to the list of guessed letters
        guessed_letters.push(guessed_char);
        guessed_letters.sort_unstable(); // Keep list sorted for better display

        // Compare with secret word and update display (handling duplicates)
        let mut found_in_word = false;
        for (i, &secret_char) in secret_word_chars.iter().enumerate() {
            if secret_char == guessed_char {
                hidden_word_display[i] = guessed_char;
                found_in_word = true;
            }
        }

        // Handle correct/incorrect guess
        if found_in_word {
            println!("Good guess! '{}' is in the word.", guessed_char);
        } else {
            println!("'{}' is not in the word. You lose a guess.", guessed_char);
            wrong_guesses_count += 1;
        }
    }
}