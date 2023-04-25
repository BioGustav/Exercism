use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let word = word.to_lowercase();
    let mut matches = HashSet::new();

    let mut w_sorted = word.chars().collect::<Vec<char>>();
    w_sorted.sort();

    for a in possible_anagrams {
        let anagram = a.to_lowercase();
        if anagram == word {
            continue;
        }
        let mut a_sorted = anagram.chars().collect::<Vec<char>>();
        a_sorted.sort();
        if w_sorted == a_sorted {
            matches.insert(a.to_owned());
        }
    }
    matches
}
