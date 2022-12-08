use std::fs;
use std::collections::HashSet;
use std::iter::FromIterator;

fn parse(input: &str) -> Vec<char> {
   input.chars().collect()
}

fn task(input: Vec<char>, window_len: usize) -> usize {
    for (i, slice) in input.windows(window_len).enumerate() {
        let unique_chars: HashSet<char> = HashSet::from_iter(slice.iter().cloned());
        if unique_chars.len() == window_len{
            return i + window_len;
        }
    }
    0
}

fn main() {
    let problem: String = fs::read_to_string("input.txt").unwrap();
    let input: Vec<char> = parse(&problem);
    println!("Task 1: {}", task(input.clone(), 4));
    println!("Task 2: {}", task(input, 14));
}
