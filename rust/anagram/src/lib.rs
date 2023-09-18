use std::collections::{HashMap, HashSet};

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&str]) -> HashSet<&'a str> {
    let mut word_chars_vec: Vec<char> = word.chars().collect();
    word_chars_vec.sort_unstable();

    let candidates: HashMap<&&str, Vec<char>> = possible_anagrams
        .iter()
        .filter(|s| s.len() == word_chars_vec.len())
        .map(|s| (s, s.to_lowercase().chars().collect::<Vec<char>>()))
        .collect();

    for mut i in candidates { i.1.sort_unstable() };

    candidates
        .into_iter()
        .filter(|(_, vec)| word_chars_vec.eq(vec))
        .map(|(s, _) | s)
        .cloned()
        .collect()
}
