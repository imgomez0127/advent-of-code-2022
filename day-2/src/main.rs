use std::fs;

#[derive(Debug, Clone)]
struct Strategy{
    opponent: Choice,
    yours: Choice
}

#[derive(Copy, Clone, Debug)]
enum Choice {
    Rock,
    Paper,
    Scissor,
}

fn parse(input: &str) -> Vec<Strategy> {
    use nom::{
        bytes::complete::tag,
        multi::{separated_list1},
        character::complete::{space1},
        combinator::value,
        branch::alt,
        error::Error
    };
    let strategies = input.split('\n');
    let parse_choices = |choice: &str| -> Strategy {
        let rock = alt((value(Choice::Rock, tag("A")), value(Choice::Rock, tag("X"))));
        let paper = alt((value(Choice::Paper, tag("B")), value(Choice::Paper, tag("Y"))));
        let scissor = alt((value(Choice::Scissor, tag("C")), value(Choice::Scissor, tag("Z"))));
        let parse_choice = alt((rock, paper, scissor));
        let parsed_choice =
            separated_list1::<_, _, _, Error<_>, _, _>(space1, parse_choice)(choice).unwrap().1;
        Strategy {
            opponent: parsed_choice[0],
            yours: parsed_choice[1]
        }

    };
    strategies.map(parse_choices).collect()
}

fn score1(strategy: &Strategy) -> i32 {
    let Strategy{opponent, yours} = strategy;
    if  (*opponent as i32) == (*yours as i32) {
        *yours as i32 + 4
    }
    else if (*opponent as i32 + 1).rem_euclid(3) == (*yours as i32) {
        *yours as i32 + 7
    } else {
        *yours as i32 + 1
    }
}

fn score2(strategy: &Strategy) -> i32 {
    let Strategy{opponent, yours} = strategy;
    match yours {
        Choice::Rock => (*opponent as i32 - 1).rem_euclid(3) + 1,
        Choice::Paper => (*opponent as i32) + 4,
        Choice::Scissor => (*opponent as i32 + 1).rem_euclid(3) + 7,
    }
}


fn task<F>(strategies: Vec<Strategy>, score: F) -> i32
where F: Fn(&Strategy) -> i32 {
     strategies.iter().map(score).sum::<i32>()
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap().trim().to_string();
    let strategies = parse(&input);
    println!("{}", task(strategies.to_vec(), score1));
    println!("{}", task(strategies.to_vec(), score2));
}
