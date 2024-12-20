use regex::Regex;
use std::env;
mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod read_input;

fn text(day: &str, part: &str) -> String {
    return format!("Day {day}, puzzle {part}");
}

fn print_solution(day: &str, part: &str) {
    match (day, part) {
        ("1", "1") => println!("Day 1, puzzle 1: {}", day_01::solve_1()),
        ("1", "2") => println!("Day 1, puzzle 2: {}", day_01::solve_2()),
        ("2", "1") => println!("{}: {}", text(day, part), day_02::solve_1()),
        ("2", "2") => println!("{}: {}", text(day, part), day_02::solve_2()),
        ("3", "1") => println!("{}: {}", text(day, part), day_03::solve_1()),
        ("3", "2") => println!("{}: {}", text(day, part), day_03::solve_2()),
        ("4", "1") => println!("{}: {}", text(day, part), day_04::solve_1()),
        ("4", "2") => println!("{}: {}", text(day, part), day_04::solve_2()),
        ("5", "1") => println!("{}: {}", text(day, part), day_05::solve_1()),
        ("5", "2") => println!("{}: {}", text(day, part), day_05::solve_2()),
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
