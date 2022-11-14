use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    println!("Hello, world!");
}
//ANAGRAMS

pub fn anagrams_for<'a>(word: &str, candidates: &[&'a str]) -> HashSet<&'a str> {
    let word_lower = word.to_lowercase();
    let required_frequencies = letter_frequencies(&word_lower);
    candidates.iter().cloned().filter(|candidate| {
        let candidate_lower = candidate.to_lowercase();
        candidate_lower != word_lower && letter_frequencies(&candidate_lower) == required_frequencies
    }).collect()
}
fn letter_frequencies(word: &str) -> HashMap<char, usize> {
    word.chars().fold(HashMap::new(), |mut freqs, c| {
        *freqs.entry(c).or_insert(0) += 1;
        freqs
    })
}

//SUBLIST
#[derive(Debug, PartialEq, Eq)]

pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(a: &[T], b: &[T]) -> Comparison {
    use Comparison::*;
    match (a.len(), b.len()) {
        (0, 0) => Equal,
        (0, _) => Sublist,
        (_, 0) => Superlist,
        (m, n) if m > n => if a.windows(n).any(|v| v == b) {Superlist} else {Unequal},
        (m, n) if m < n => if b.windows(m).any(|v| v == a) {Sublist} else {Unequal},
        (_, _) => if a == b {Equal} else {Unequal},
    }
}

//REVERSE STRING

pub fn reverse(input: &str) -> String {
    input.chars().rev().collect::<String>()
}


