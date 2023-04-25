use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let word = word.to_lowercase();

    let mut w_sorted = word.chars().collect::<Vec<char>>();
    w_sorted.sort();

    possible_anagrams
        .iter()
        .filter(|possible_anagram| {
            let possible_anagram = possible_anagram.to_lowercase();
            if possible_anagram == word {
                false
            } else {
                let mut a_sorted: Vec<char> = possible_anagram.chars().collect();
                a_sorted.sort();
                w_sorted == a_sorted
            }
        })
        .copied()
        .collect()
}
