use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let word_lowercase = word.to_lowercase();
    let word_sorted = str_to_sorted_vec(&word_lowercase);

    possible_anagrams
        .iter()
        .filter(|candidate| {
            let candidate_lowercase = candidate.to_lowercase();

            candidate_lowercase.len() == word.len()
                && candidate_lowercase != word_lowercase
                && word_sorted == str_to_sorted_vec(&candidate_lowercase)
        })
        .copied()
        .collect()
}

fn str_to_sorted_vec(word: &str) -> Vec<char> {
    let mut chars: Vec<char> = word.chars().collect();
    chars.sort_unstable();

    return chars;
}
