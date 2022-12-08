use std::fs;
use std::str::Lines;
use std::cmp::max;

fn parse(input: &str) -> Vec<Vec<u32>> {
    let rows: Lines = input.lines();
    rows.map(|row: &str| -> Vec<u32> {
        row.chars().map(|c: char| c.to_digit(10).unwrap()).collect()
    }).collect()
}

fn check_neighbors(input: &[Vec<u32>], i: usize, j: usize) -> bool {
    let mut above: bool = true;
    let mut below: bool = true;
    let mut left: bool = true;
    let mut right: bool = true;
    for k in 0..i {
        above &= input[i][j] > input[k][j];
    }
    for k in i+1..input.len() {
        below &= input[i][j] > input[k][j];
    }
    for k in 0..j {
        left &= input[i][j] > input[i][k];
    }
    for k in j+1..input[0].len() {
        right &= input[i][j] > input[i][k];
    }
    above || below || left || right
}

fn view_score(input: &[Vec<u32>], i: usize, j: usize) -> i32 {
    let mut above: i32 = 1;
    let mut below: i32 = 1;
    let mut left: i32 = 1;
    let mut right: i32 = 1;
    for k in (1..i).rev() {
        if input[i][j] <= input[k][j] {
            break;
        }
        above += 1;
    }
    for k in i+1..input.len()-1 {
        if input[i][j] <= input[k][j] {
            break;
        }
        below += 1;
    }
    for k in (1..j).rev() {
        if input[i][j] <= input[i][k] {
            break;
        }
        left += 1;
    }
    for k in j+1..input[0].len()-1 {
        if input[i][j] <= input[i][k] {
            break;
        }
        right += 1;
    }
    above * below * left * right
}

fn task1(input: &[Vec<u32>]) -> i32 {
    let mut count: i32 = 0;
    let row_size: usize = input.len();
    let col_size: usize = input[0].len();
    for i in 1..row_size-1 {
        for j in 1..col_size-1 {
            if check_neighbors(input, i, j) {
                count += 1;
            }
        }
    }
    count + (row_size as i32) * 2 + (col_size as i32) * 2 - 4
}

fn task2(input: &[Vec<u32>]) -> i32 {
    let mut max_viewability = 0;
    let row_size: usize = input.len();
    let col_size: usize = input[0].len();
    for i in 1..row_size-1 {
        for j in 1..col_size-1 {
            max_viewability = max(max_viewability, view_score(input, i, j));
        }
    }
    max_viewability
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let tree_grid = parse(&input);
    println!("{}", task1(&tree_grid));
    println!("{}", task2(&tree_grid))
}
