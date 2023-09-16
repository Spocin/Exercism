use std::collections::HashSet;

pub fn anagrams_for<'data>(word: &str, possible_anagrams: &[&str]) -> HashSet<&'data str> {
    let mut word_sorted: Vec<char> = word.chars().collect();
    word_sorted.sort_unstable();

    let possible_filtered: Vec<&&str> = possible_anagrams
        .iter()
        .filter(|s| s.len() == word_sorted.len())
        .collect();

    let mut possible_sorted: Vec<Vec<char>> = possible_filtered
        .iter()
        .map(|s| s.chars().collect::<Vec<char>>())
        .map(|mut s| s.sort_unstable())
        .collect();

    return HashSet::new();
}
