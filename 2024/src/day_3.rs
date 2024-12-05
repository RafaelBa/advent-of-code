use regex::Regex;
use std::fs;

pub fn solve_1() -> u32 {
    let contents = fs::read_to_string(FILE_PATH).expect("Should have been able to read the file");
    decorrupt_memory(contents.as_str())
}

pub fn solve_2() -> u32 {
    todo!();
}

pub fn decorrupt_memory(input: &str) -> u32 {
    let re = Regex::new(r"mul\(([\d]{1,3}),([\d]{1,3})\)").unwrap();
    let result: u32 = re
        .captures_iter(input)
        .map(|caps| {
            let (_, [left, right]) = caps.extract();
            left.parse::<u32>()
                .expect(format!("left is not a number: {left}").as_str())
                * right
                    .parse::<u32>()
                    .expect(format!("right is not a number: {right}").as_str())
        })
        .sum();
    return result;
}

static FILE_PATH: &str = "src/day_3-input.txt";

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example() {
        let test_input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(self::decorrupt_memory(test_input), 161);
    }

    #[test]
    fn test_solve_1() {
        assert_eq!(solve_1(), 159892596);
    }
}
