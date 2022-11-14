use std::collections::HashMap;
use std::collections::HashSet;

//ANAGRAMS

pub fn anagrams_for<'a>(word: &str, candidates: &[&'a str]) -> HashSet<&'a str> {
    let word_lower = word.to_lowercase();
    let required_frequencies = letter_frequencies(&word_lower);
    return candidates.into_iter().cloned().filter(|x| word.chars().all(|ch| x.contains(ch))
        && x.chars().all(|ch| word.contains(ch))
        && word.len() == x.len())
        .collect();
    /*candidates.iter().cloned().filter(|candidate| {
        let candidate_lower = candidate.to_lowercase();
        candidate_lower != word_lower && letter_frequencies(&candidate_lower) == required_frequencies
    }).collect()*/
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

//HIGH SCORES
#[derive(Debug)]
pub struct HighScores{
    scores: Vec<u32>
}

impl HighScores {
    pub fn new(scores: &[u32]) -> Self {
        HighScores {
            scores: Vec::from(scores)
        }
    }

    pub fn scores(&self) -> &[u32] {
        &self.scores[..]
    }

    pub fn latest(&self) -> Option<u32> {
        self.scores.last().cloned()
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.scores.iter().max().cloned()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut score_copy = self.scores.clone();
        score_copy.sort();
        score_copy.iter().copied().rev().take(3).collect()
        /*let mut res_vec = self.scores.to_vec();
        res_vec.sort_unstable_by(|a, b| b.cmp(a));
        res_vec.truncate(3);
        res_vec*/
    }
}

//BINARY SEARCH
use std::cmp::Ordering::{Equal, Less, Greater};

pub fn find<T,R>(ar:R,v:T)->Option<usize>
    where
        T: Ord,
        R: AsRef<[T]>
{
    let ar = ar.as_ref();
    if ar.len() == 0 {
        return None
    }
    let mid = ar.len()/2;
    match v.cmp(&ar[mid]) {
        Equal => Some(mid),
        Less => find(&ar[..mid],v),
        Greater => match find(&ar[mid+1..],v) {
            Some(ndx) => Some(mid+1+ndx),
            None => None,
        }
    }
}

// TODO: modify only this function.
fn copy_and_return<'a>(vector: &'a mut Vec<String>, value: &'a str) -> &'a str {
    vector.push(String::from(value));
    value
}

fn main() {
    let name1 = "Joe";
    let name2 = "Chris";
    let name3 = "Anne";

    let mut names = Vec::new();

    assert_eq!("Joe", copy_and_return(&mut names, &name1));
    assert_eq!("Chris", copy_and_return(&mut names, &name2));
    assert_eq!("Anne", copy_and_return(&mut names, &name3));

    assert_eq!(
        names,
        vec!["Joe".to_string(), "Chris".to_string(), "Anne".to_string()]
    )
}

//LINKED LIST
use std::iter::FromIterator;

struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}
pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(data: T, next: Option<Box<Node<T>>>) -> Self {
        Self { data, next }
    }
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList{
            head: None
        }
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn len(&self) -> usize {
        let mut current_node = &self.head;
        let mut size = 0;
        while let Some(x) = current_node {
            size += 1;
            current_node = &x.next;
        }
        size
    }

    pub fn push(&mut self, element: T) {
        let node = Box::new(Node::new(element, self.head.take()));
        self.head = Some(node);
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.head.is_some() {
            let head_node = self.head.take().unwrap();
            self.head = head_node.next;
            Some(head_node.data)
        } else {
            None
        }
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|head| &(head.data))
    }

    #[must_use]
    pub fn rev(&mut self) -> SimpleLinkedList<T> {
        let mut reverse = SimpleLinkedList::new();
        while let Some(data) = self.pop() {
            reverse.push(data);
        }
        reverse
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut list = SimpleLinkedList::new();
        for item in iter {
            list.push(item);
        }
        list
    }
}

/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    let mut letters: Vec<char> = sentence.chars().filter(|c| c.is_alphabetic()).collect();
    letters.sort();
    letters.dedup();
    letters.len() >= 26
}
