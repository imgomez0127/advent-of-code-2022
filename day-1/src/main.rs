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

fn task(meals: Vec<Vec<i32>>, k: usize) -> i32 {
    let mut calories : Vec<i32> = meals.into_iter()
                                       .map(|meal: Vec<i32>| meal.iter().sum::<i32>()).collect();
    calories.sort_unstable();
    calories.reverse();
    calories[..k].iter().sum()
}


fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let meals = parse1(&input);
    println!("Task 1: {}", task(meals, 3));
}
