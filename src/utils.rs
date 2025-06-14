use crate::models::{Root};
use rand::seq::SliceRandom;
use std::fs;
use std::io::{self};

// --- GLOBAL CONSTANTS ---
pub const MAX_WRONG_GUESSES: u8 = 6;
const WORDS_JSON_PATH: &str = "hidden_words.json";

pub fn load_words_from_json() -> Result<Root, Box<dyn std::error::Error>> {
    println!("Attempting to load words from: {}", WORDS_JSON_PATH);
    let json_content = fs::read_to_string(WORDS_JSON_PATH)?;
    let root: Root = serde_json::from_str(&json_content)?;
    println!("Words loaded successfully.");
    Ok(root)
}

pub fn select_random_word<'a>(word_list: &'a Vec<String>) -> &'a str {
    word_list.choose(&mut rand::thread_rng()).unwrap()
}
pub fn get_word_list_choice(all_words: &Root) -> Option<&Vec<String>> {
    loop {
        println!("\nChoose a word list for Hangman:");
        println!("1. 4-letter words");
        println!("2. 5-letter words");
        println!("3. 6-letter words");
        println!("Enter your choice (1, 2, or 3, or 'q' to quit):");

        let mut choice_input = String::new();
        io::stdin()
            .read_line(&mut choice_input)
            .expect("Failed to read line");
        let choice = choice_input.trim(); // Remove newline and whitespace

        if choice.eq_ignore_ascii_case("q") {
            return None; // User wants to quit
        }

        match choice {
            "1" => return Some(&all_words.word_lists.four_letter_words),
            "2" => return Some(&all_words.word_lists.five_letter_words),
            "3" => return Some(&all_words.word_lists.six_letter_words),
            _ => {
                println!("Invalid choice. Please enter 1, 2, 3, or 'q'.");
                // Loop continues
            }
        }
    }
}