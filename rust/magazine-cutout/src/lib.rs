// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
//#![allow(unused)]

use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut mag_count: HashMap<&str, u32> = HashMap::new();
    let mut note_count: HashMap<&str, u32> = HashMap::new();

    for word in magazine {
        mag_count.entry(*word).and_modify(|x| *x += 1).or_insert(1);
    }
    for word in note {
        note_count.entry(*word).and_modify(|x| *x += 1).or_insert(1);
    }
    note_count
        .iter()
        .all(|(k, v)| mag_count.contains_key(*k) && mag_count[*k] >= *v)
}
