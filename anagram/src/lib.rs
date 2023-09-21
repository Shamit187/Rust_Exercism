use std::collections::HashSet;

fn get_sorted(word: &str) -> Vec<char> {
    let mut chars: Vec<char> = word.chars().collect();
    chars.sort_unstable();
    chars
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let word_lower = word.to_lowercase();
    let word_sorted = get_sorted(&word_lower);
    
    possible_anagrams
        .iter()
        .filter(|canditate| {
            let canditate_lower = canditate.to_lowercase();
            canditate_lower.len() == word_lower.len()
                && canditate_lower != word_lower
                && get_sorted(&canditate_lower) == word_sorted
        })
        .copied()
        .collect()
}
