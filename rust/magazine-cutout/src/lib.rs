use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut counter = magazine.iter().fold(HashMap::new(), |mut counter, word| {
        *counter.entry(word).or_insert(0) += 1;
        counter
    });

    note.iter().all(|word| {
        *counter.entry(word).or_insert(0) -= 1;
        counter[word] >= 0
    })
}
