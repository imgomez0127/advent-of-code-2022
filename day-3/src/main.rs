use std::fs;
use std::collections::HashSet;
use std::iter::FromIterator;


fn parse(input: &str) -> Vec<Vec<char>> {
    let bags = input.lines();
    bags.map(|bag: &str| bag.chars().collect()).collect()
}


fn priorities(items: Vec<char>) -> i32 {
    let mut s: i32 = 0;
    for item in items.iter() {
        if item.is_lowercase() {
            s += *item as i32 - 96;
        }
        else {
            s += *item as i32 - 38
        }
    }
    s
}


fn task(bags: Vec<Vec<char>>) -> i32 {
    let mut intersection : Vec<char> = Vec::new();
    for bag in bags.iter() {
        let len: usize = bag.len();
        let a: HashSet<char> = HashSet::from_iter(bag[..len/2].iter().cloned());
        let b: HashSet<char> = HashSet::from_iter(bag[len/2..].iter().cloned());
        intersection.extend(a.intersection(&b).into_iter());
    }
    priorities(intersection)

}


fn task2(bags: Vec<Vec<char>>) -> i32 {
    let mut intersection : Vec<char> = Vec::new();
    for i in (0..bags.len()).step_by(3) {
        let a: HashSet<char> = HashSet::from_iter(bags[i].iter().cloned());
        let b: HashSet<char> = HashSet::from_iter(bags[i+1].iter().cloned());
        let c: HashSet<char> = HashSet::from_iter(bags[i+2].iter().cloned());
        let a_and_b : HashSet<char> = a.intersection(&b).copied().collect();
        intersection.extend(a_and_b.intersection(&c).into_iter());
    }
    priorities(intersection)
}


fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let bags = parse(&input);
    println!("Task 1: {}", task(bags.clone()));
    println!("Task 2: {}", task2(bags));
}
