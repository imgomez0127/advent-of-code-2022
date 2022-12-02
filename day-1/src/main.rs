use std::fs;

fn parse1(input: &str) -> Vec<Vec<i32>> {
    use nom::{
        multi::separated_list1,
        character::complete::{newline, i32},
        error::Error
    };
    let meals = input.split("\n\n");
    let parse_calories = |meal: &str| {
        separated_list1::<_, _, _, Error<_>, _, _>(newline, i32)(meal).unwrap().1
    };
    meals.map(parse_calories).collect()
}

fn lomuto_partition(vector: &mut Vec<i32>, lo: usize, hi:usize) -> usize {
    let pivot = vector[hi];
    let mut i = lo;
    for j in lo..hi {
        if vector[j] < pivot {
            i += 1;
            vector.swap(i-1, j);
        }
    }
    i += 1;
    vector.swap(i-1, hi);
    i-1
}

fn partition_vec(vector: &mut Vec<i32>, lo:usize, hi:usize, k: usize) {
    if lo > hi {
        return;
    }
    let pivot = lomuto_partition(vector, lo, hi);
    if pivot == vector.len() - k {
        return;
    }
    if pivot < vector.len() - k {
        partition_vec(vector, pivot+1, hi, k);
    }
    else {
        partition_vec(vector, lo, pivot-1, k)
    }
}

fn task(meals: Vec<Vec<i32>>, k: usize) -> i32 {
    let mut calories : Vec<i32> = meals
        .iter()
        .cloned()
        .map(|meal: Vec<i32>| meal.iter().sum::<i32>())
        .collect();
    let size = calories.len() - 1;
    partition_vec(&mut calories, 0, size, k);
    calories.reverse();
    calories[..k].iter().sum()
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let meals = parse1(&input);
    println!("Task 1: {}", task(meals, 3));
}
