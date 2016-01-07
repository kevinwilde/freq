use std::collections::HashMap;

#[doc="
Counts the frequencies of words read from the standard input, and print a sorted frequency table.

Assumptions:

"]
fn main() {
//    let word_counts = read_and_count(...);
//    print_counts(word_counts);
}

fn increment_word(map: &mut HashMap<String, usize>, word: String) {
    let count_ref = map.entry(word).or_insert(0);
    *count_ref += 1;
}
