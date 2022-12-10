use std::collections::HashSet;
use std::collections::HashMap;
use ndarray::{array};
use std::fs;

fn parse(input: &str) -> Vec<(&str, usize)>{
    let lines = input.lines();
    lines.map(|line: &str| -> (&str, usize){
        let values: Vec<&str> = line.split(' ').collect();
        (values[0], values[1].parse::<usize>().unwrap())
    }).collect()
}


fn task(input: Vec<(&str, usize)>, num_knots: usize) -> usize{
    let mut seen = HashSet::new();
    let force_vector = HashMap::from([
        ("U", array![0, 1]),
        ("D", array![0, -1]),
        ("L", array![-1, 0]),
        ("R", array![1, 0]),
    ]);
    let mut knots = Vec::new();
    for _i in 0..num_knots {
        knots.push(array![0, 0]);
    }
    seen.insert((0, 0));
    for (command, amt) in input.iter() {
        for _i in 0..*amt {
            knots[0] = &knots[0] + force_vector.get(command).unwrap();
            for (i, j) in (0..num_knots-1).zip(1..num_knots) {
                let (head, tail) = (&knots[i], &knots[j]);
                let distance = head - tail;
                let clipped_distance = distance.mapv(
                    |x: i32| i32::max(-1, i32::min(x, 1)));
                let l_inf_distance = (head-tail).fold(
                    0, |acc:i32, x: &i32| i32::max(acc, x.abs()));
                let far = clipped_distance * ((l_inf_distance > 1) as i32);
                knots[j] = tail + far;
            }
            let tail = knots.last().unwrap();
            seen.insert((tail[0], tail[1]));
        }
    }
    seen.len()

}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let commands = parse(&input);
    println!("{}", task(commands.clone(), 2));
    println!("{}", task(commands, 10))
}
