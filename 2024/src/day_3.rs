use regex::Regex;
use std::fs;

pub fn solve_1() -> u32 {
    let contents = fs::read_to_string(FILE_PATH).expect("Should have been able to read the file");
    decorrupt_memory(contents.as_str())
}

pub fn solve_2() -> u32 {
    let contents = fs::read_to_string(FILE_PATH).expect("Should have been able to read the file");
    decorrupt_enabled_memory(contents.as_str())
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

pub fn decorrupt_enabled_memory(input: &str) -> u32 {
    let enabled_segments: Vec<&str> = find_enabled_segments(input);

    enabled_segments.iter().map(|s| decorrupt_memory(s)).sum()
}

fn find_enabled_segments(input: &str) -> Vec<&str> {
    // TODO: one approach is to collect all regexes of kind r"do\(\).*don't\(\)", the first via
    // r"^do" - and then there's the last one, not sure how yet
    // The first one will go up until "do", because from there the "do.*don't" should find that one
    // anyway, and if the first "do" belongs to a don't, then I want to stop there, too.
    // I might run into the issue that "do() mul(1,2) do() mul(2,3) don't()" might pick up mul(2,3)
    // twice.
    // How do I pick up the last "do()mul(2,3)don't()mul(0,0)do()mul(1,2)$" without picking up
    // mul(2,3)?
    todo!()
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
