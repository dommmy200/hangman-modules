use serde::Deserialize;

// --- STRUCT DEFINITIONS (Matching JSON structure) ---
#[derive(Debug, Deserialize)]
pub struct WordLists {
    pub four_letter_words: Vec<String>,
    pub five_letter_words: Vec<String>,
    pub six_letter_words: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Root {
    pub word_lists: WordLists,
}