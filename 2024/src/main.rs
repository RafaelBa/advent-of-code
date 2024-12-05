use regex::Regex;
use std::env;
mod day_1;

fn print_solution(day: &str, part: &str) {
    match (day, part) {
        ("1", "1") =>     println!("Day 1, puzzle 1: {}", day_1::solve_1()),
        ("4", "1") => println!("here could be the solution for day 4, part 1"),
        (d, p) => println!("No solution for day {} part {}", d, p),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let solution = args.iter().nth(1);
    let re = Regex::new(r"([\d]{1,2})\.([12])").unwrap();
    match solution {
        Some(s) => {
            let caps = re.captures(s);
            match caps {
                Some(c) => print_solution(&c[1], &c[2]),
                None => println!("Invalid input: {} - Input must [day].[part], eg. 12.1", s),
            }
        }
        None => println!("Invalid input. Input must [day].[part], eg. 12.1"),
    }
}
